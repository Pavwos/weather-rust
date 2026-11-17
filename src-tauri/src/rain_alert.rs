use anyhow::{bail, Context, Result};
use chrono::{Duration, NaiveDateTime, Timelike};
use std::{
    sync::Mutex,
    time::{Duration as StdDuration, Instant},
};
use tauri::{AppHandle, Manager};
use tauri_plugin_notification::NotificationExt;

use crate::{
    api,
    config::Config,
    models::{ForecastResponse, HourForecast, Units},
};

pub const DEFAULT_HORIZON_HOURS: u8 = 2;
const RAIN_PROBABILITY_THRESHOLD: u8 = 60;
const RELEVANT_PRECIPITATION_MM: f64 = 0.3;
const ALERT_COOLDOWN: StdDuration = StdDuration::from_secs(3 * 60 * 60);

#[derive(Clone, Debug, PartialEq)]
pub struct RainAlert {
    pub starts_at: String,
    pub probability: u8,
    pub precipitation: f64,
}

pub fn next_rain(hourly: &[HourForecast], now: &str, horizon_hours: u8) -> Option<RainAlert> {
    let now = parse_datetime(now)?;
    if hourly.iter().any(|hour| {
        parse_datetime(&hour.time) == Some(now) && hour.precipitation >= RELEVANT_PRECIPITATION_MM
    }) {
        return None;
    }

    let horizon_end = now.checked_add_signed(Duration::hours(i64::from(horizon_hours)))?;
    hourly
        .iter()
        .filter_map(|hour| {
            let starts_at = parse_datetime(&hour.time)?;
            (starts_at > now
                && starts_at <= horizon_end
                && (hour.precipitation_probability >= RAIN_PROBABILITY_THRESHOLD
                    || hour.precipitation >= RELEVANT_PRECIPITATION_MM))
                .then_some((starts_at, hour))
        })
        .min_by_key(|(starts_at, _)| *starts_at)
        .map(|(_, hour)| RainAlert {
            starts_at: hour.time.clone(),
            probability: hour.precipitation_probability,
            precipitation: hour.precipitation,
        })
}

#[derive(Default)]
pub struct RainAlertDeduplicator {
    last_sent_at: Option<Instant>,
}

impl RainAlertDeduplicator {
    pub fn can_send_at(&self, now: Instant) -> bool {
        self.last_sent_at
            .is_none_or(|last_sent| now.saturating_duration_since(last_sent) >= ALERT_COOLDOWN)
    }

    pub fn mark_sent_at(&mut self, now: Instant) {
        self.last_sent_at = Some(now);
    }
}

#[derive(Default)]
pub struct RainAlertState(pub Mutex<RainAlertDeduplicator>);

pub async fn check_last_city(app: &AppHandle) -> Result<()> {
    let config = Config::load()?;
    if !config.rain_alerts {
        return Ok(());
    }

    let Some(city) = config
        .last_city
        .as_deref()
        .map(str::trim)
        .filter(|city| !city.is_empty())
    else {
        return Ok(());
    };
    let units = match config.units.as_str() {
        "metric" => Units::Metric,
        "imperial" => Units::Imperial,
        _ => bail!("A unidade salva para o aviso de chuva é inválida"),
    };
    let forecast = api::fetch_forecast(city, units, 5)
        .await
        .context("Não foi possível consultar a previsão para o aviso de chuva")?;

    notify_if_needed(app, &forecast)
}

fn notify_if_needed(app: &AppHandle, forecast: &ForecastResponse) -> Result<()> {
    let hourly = forecast
        .hourly
        .to_hour_forecasts(&forecast.current.time)
        .context("Não foi possível converter a previsão por hora para o aviso")?;
    let Some(alert) = next_rain(&hourly, &forecast.current.time, DEFAULT_HORIZON_HOURS) else {
        return Ok(());
    };
    let now = Instant::now();
    let state = app.state::<RainAlertState>();
    let mut deduplicator = state
        .0
        .lock()
        .map_err(|_| anyhow::anyhow!("Não foi possível acessar a deduplicação dos avisos"))?;
    if !deduplicator.can_send_at(now) {
        return Ok(());
    }

    app.notification()
        .builder()
        .title(format!("Clima: {}", forecast.location.name))
        .body(alert_message(&alert)?)
        .show()
        .context("Não foi possível exibir a notificação de chuva")?;
    deduplicator.mark_sent_at(now);
    Ok(())
}

fn alert_message(alert: &RainAlert) -> Result<String> {
    let starts_at = parse_datetime(&alert.starts_at)
        .ok_or_else(|| anyhow::anyhow!("O horário do aviso de chuva é inválido"))?;
    let detail = if alert.probability >= RAIN_PROBABILITY_THRESHOLD {
        format!("{}% de chance", alert.probability)
    } else {
        let precipitation = format!("{:.1}", alert.precipitation).replace('.', ",");
        format!("{precipitation} mm previstos")
    };
    Ok(format!(
        "Chuva chegando por volta das {}h ({detail}). Leve o guarda-chuva.",
        starts_at.hour()
    ))
}

fn parse_datetime(value: &str) -> Option<NaiveDateTime> {
    NaiveDateTime::parse_from_str(value, "%Y-%m-%dT%H:%M")
        .or_else(|_| NaiveDateTime::parse_from_str(value, "%Y-%m-%dT%H:%M:%S"))
        .ok()
}

#[cfg(test)]
mod tests {
    use super::{
        alert_message, next_rain, RainAlert, RainAlertDeduplicator, DEFAULT_HORIZON_HOURS,
    };
    use crate::models::HourForecast;
    use std::time::{Duration, Instant};

    fn hour(time: &str, probability: u8, precipitation: f64) -> HourForecast {
        HourForecast {
            time: time.to_owned(),
            temperature: 20.0,
            precipitation_probability: probability,
            precipitation,
            humidity: 60,
            weather_code: 0,
            icon: "clear-day".to_owned(),
            is_day: true,
            wind_speed: 10.0,
        }
    }

    fn forecast() -> Vec<HourForecast> {
        vec![
            hour("2026-01-15T14:00", 0, 0.0),
            hour("2026-01-15T15:00", 59, 0.0),
            hour("2026-01-15T16:00", 0, 0.0),
            hour("2026-01-15T17:00", 60, 0.0),
        ]
    }

    #[test]
    fn alerts_for_rain_within_the_default_horizon() {
        let mut hourly = forecast();
        hourly[2].precipitation = 0.3;

        let alert = next_rain(&hourly, "2026-01-15T14:00", DEFAULT_HORIZON_HOURS)
            .expect("deve alertar sobre chuva dentro do horizonte");

        assert_eq!(alert.starts_at, "2026-01-15T16:00");
        assert_eq!(alert.probability, 0);
        assert_eq!(alert.precipitation, 0.3);
    }

    #[test]
    fn does_not_alert_below_threshold_or_outside_the_horizon() {
        let hourly = forecast();

        assert!(next_rain(&hourly, "2026-01-15T14:00", DEFAULT_HORIZON_HOURS).is_none());
    }

    #[test]
    fn does_not_alert_when_current_hour_is_already_raining() {
        let mut hourly = forecast();
        hourly[0].precipitation = 0.3;
        hourly[1].precipitation_probability = 90;

        assert!(next_rain(&hourly, "2026-01-15T14:00", DEFAULT_HORIZON_HOURS).is_none());
    }

    #[test]
    fn honors_exact_probability_precipitation_and_horizon_limits() {
        let mut hourly = forecast();
        hourly[1].precipitation_probability = 60;
        assert_eq!(
            next_rain(&hourly, "2026-01-15T14:00", DEFAULT_HORIZON_HOURS)
                .map(|alert| alert.starts_at),
            Some("2026-01-15T15:00".to_owned())
        );

        hourly[1].precipitation_probability = 0;
        hourly[1].precipitation = 0.3;
        assert_eq!(
            next_rain(&hourly, "2026-01-15T14:00", DEFAULT_HORIZON_HOURS)
                .map(|alert| alert.starts_at),
            Some("2026-01-15T15:00".to_owned())
        );

        hourly[1].precipitation = 0.0;
        assert!(next_rain(&hourly, "2026-01-15T14:00", 1).is_none());
    }

    #[test]
    fn uses_the_api_time_instead_of_the_system_clock() {
        let hourly = forecast();

        assert_eq!(
            next_rain(&hourly, "2026-01-15T15:00", DEFAULT_HORIZON_HOURS)
                .map(|alert| alert.starts_at),
            Some("2026-01-15T17:00".to_owned())
        );
    }

    #[test]
    fn deduplicates_alerts_for_three_hours() {
        let now = Instant::now();
        let mut deduplicator = RainAlertDeduplicator::default();

        assert!(deduplicator.can_send_at(now));
        deduplicator.mark_sent_at(now);
        assert!(!deduplicator.can_send_at(now + Duration::from_secs(3 * 60 * 60 - 1)));
        assert!(deduplicator.can_send_at(now + Duration::from_secs(3 * 60 * 60)));
    }

    #[test]
    fn formats_alerts_with_the_relevant_rain_detail() {
        let probability_alert = RainAlert {
            starts_at: "2026-01-15T15:00".to_owned(),
            probability: 70,
            precipitation: 0.2,
        };
        assert_eq!(
            alert_message(&probability_alert).expect("mensagem válida"),
            "Chuva chegando por volta das 15h (70% de chance). Leve o guarda-chuva."
        );

        let precipitation_alert = RainAlert {
            starts_at: "2026-01-15T16:00".to_owned(),
            probability: 10,
            precipitation: 0.3,
        };
        assert_eq!(
            alert_message(&precipitation_alert).expect("mensagem válida"),
            "Chuva chegando por volta das 16h (0,3 mm previstos). Leve o guarda-chuva."
        );
    }
}
