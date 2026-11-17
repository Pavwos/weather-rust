use anyhow::{anyhow, Context, Result};
use reqwest::Client;
use std::{
    collections::HashMap,
    sync::{Mutex, OnceLock},
    time::{Duration, Instant},
};

use crate::{
    models::Units,
    models::{ForecastResponse, GeocodingResponse},
};

const GEOCODING_URL: &str = "https://geocoding-api.open-meteo.com/v1/search";
const FORECAST_URL: &str = "https://api.open-meteo.com/v1/forecast";
const CACHE_DURATION: Duration = Duration::from_secs(10 * 60);

#[derive(Eq, Hash, PartialEq)]
struct CacheKey {
    city: String,
    imperial: bool,
    days: u8,
}

struct CacheEntry {
    forecast: ForecastResponse,
    cached_at: Instant,
}

static FORECAST_CACHE: OnceLock<Mutex<HashMap<CacheKey, CacheEntry>>> = OnceLock::new();

pub async fn fetch_forecast(city: &str, units: Units, days: u8) -> Result<ForecastResponse> {
    let key = CacheKey {
        city: city.trim().to_lowercase(),
        imperial: units.is_imperial(),
        days,
    };
    let cache = FORECAST_CACHE.get_or_init(|| Mutex::new(HashMap::new()));
    if let Some(forecast) = {
        let mut cache = cache
            .lock()
            .map_err(|_| anyhow!("Não foi possível acessar o cache da previsão"))?;
        cached_forecast(&mut cache, &key, Instant::now())
    } {
        return Ok(forecast);
    }

    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .context("Não foi possível configurar o cliente HTTP")?;

    let geocoding: GeocodingResponse = client
        .get(GEOCODING_URL)
        .query(&[("name", city), ("count", "1"), ("language", "pt")])
        .send()
        .await
        .map_err(|error| {
            request_error(
                error,
                "Falha na conexão com o serviço de localização. Verifique sua internet.",
            )
        })?
        .error_for_status()
        .context("O serviço de localização retornou um erro")?
        .json()
        .await
        .map_err(|error| request_error(error, "Resposta inválida do serviço de localização"))?;

    let location = geocoding
        .results
        .and_then(|mut results| {
            if results.is_empty() {
                None
            } else {
                Some(results.remove(0))
            }
        })
        .with_context(|| format!("Cidade não encontrada: {city}"))?;

    let mut request = client.get(FORECAST_URL).query(&[
        ("latitude", location.latitude.to_string()),
        ("longitude", location.longitude.to_string()),
        (
            "current",
            "temperature_2m,apparent_temperature,relative_humidity_2m,wind_speed_10m,weather_code,is_day"
                .to_owned(),
        ),
        (
            "daily",
            "weather_code,temperature_2m_max,temperature_2m_min,precipitation_probability_max,sunrise,sunset"
                .to_owned(),
        ),
        (
            "hourly",
            "temperature_2m,precipitation_probability,precipitation,weather_code,relative_humidity_2m,is_day,wind_speed_10m"
                .to_owned(),
        ),
        ("timezone", "auto".to_owned()),
        ("forecast_days", days.to_string()),
        ("past_days", "1".to_owned()),
    ]);

    if units.is_imperial() {
        request = request.query(&[
            ("temperature_unit", "fahrenheit"),
            ("wind_speed_unit", "mph"),
        ]);
    }

    let mut forecast: ForecastResponse = request
        .send()
        .await
        .map_err(|error| {
            request_error(
                error,
                "Falha na conexão com o serviço de previsão. Verifique sua internet.",
            )
        })?
        .error_for_status()
        .context("O serviço de previsão retornou um erro")?
        .json()
        .await
        .map_err(|error| request_error(error, "Resposta inválida do serviço de previsão"))?;
    forecast.location = location;

    cache
        .lock()
        .map_err(|_| anyhow!("Não foi possível acessar o cache da previsão"))?
        .insert(
            key,
            CacheEntry {
                forecast: forecast.clone(),
                cached_at: Instant::now(),
            },
        );

    Ok(forecast)
}

fn cached_forecast(
    cache: &mut HashMap<CacheKey, CacheEntry>,
    key: &CacheKey,
    now: Instant,
) -> Option<ForecastResponse> {
    cache.retain(|_, entry| now.saturating_duration_since(entry.cached_at) < CACHE_DURATION);
    cache.get(key).map(|entry| entry.forecast.clone())
}

fn request_error(error: reqwest::Error, context: &str) -> anyhow::Error {
    if error.is_timeout() {
        anyhow::Error::new(error).context(
            "Tempo limite de 10 segundos excedido. Verifique sua conexão e tente novamente.",
        )
    } else {
        anyhow::Error::new(error).context(context.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{CurrentWeather, DailyWeather, GeoResult, HourlyWeather};

    fn sample_forecast() -> ForecastResponse {
        ForecastResponse {
            location: GeoResult {
                name: "São Paulo".to_owned(),
                latitude: -23.55,
                longitude: -46.63,
                country: Some("Brasil".to_owned()),
                admin1: Some("São Paulo".to_owned()),
            },
            current: CurrentWeather {
                time: "2026-11-14T09:00".to_owned(),
                temperature_2m: 20.0,
                apparent_temperature: 20.0,
                relative_humidity_2m: 60,
                wind_speed_10m: 10.0,
                weather_code: 0,
                is_day: 1,
            },
            hourly: HourlyWeather {
                time: Vec::new(),
                temperature_2m: Vec::new(),
                precipitation_probability: Vec::new(),
                precipitation: Vec::new(),
                relative_humidity_2m: Vec::new(),
                weather_code: Vec::new(),
                is_day: Vec::new(),
                wind_speed_10m: Vec::new(),
            },
            daily: DailyWeather {
                time: Vec::new(),
                weather_code: Vec::new(),
                temperature_2m_max: Vec::new(),
                temperature_2m_min: Vec::new(),
                precipitation_probability_max: Vec::new(),
                sunrise: Vec::new(),
                sunset: Vec::new(),
            },
        }
    }

    #[test]
    fn cached_forecast_expires_after_ten_minutes() {
        let key = CacheKey {
            city: "são paulo".to_owned(),
            imperial: false,
            days: 5,
        };
        let now = Instant::now();
        let mut cache = HashMap::from([(
            CacheKey {
                city: "são paulo".to_owned(),
                imperial: false,
                days: 5,
            },
            CacheEntry {
                forecast: sample_forecast(),
                cached_at: now,
            },
        )]);

        assert!(cached_forecast(
            &mut cache,
            &key,
            now + CACHE_DURATION - Duration::from_secs(1)
        )
        .is_some());
        assert!(cached_forecast(&mut cache, &key, now + CACHE_DURATION).is_none());
    }
}
