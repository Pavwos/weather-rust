use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::models::{HourForecast, Units};

const RAIN_PROBABILITY_THRESHOLD: u8 = 50;
const RELEVANT_PRECIPITATION_MM: f64 = 0.2;
const DAWN_START_HOUR: u8 = 0;
const MORNING_START_HOUR: u8 = 6;
const AFTERNOON_START_HOUR: u8 = 12;
const EVENING_START_HOUR: u8 = 18;
const DAY_END_HOUR: u8 = 24;
const TOMORROW_SUMMARY_AFTER_HOUR: u8 = 20;
const MAX_SUMMARY_RULES: usize = 2;
const METRIC_TEMPERATURE_CHANGE: f64 = 3.0;
const IMPERIAL_TEMPERATURE_CHANGE: f64 = 5.0;
const HIGH_HUMIDITY_THRESHOLD: f64 = 80.0;
const LOW_HUMIDITY_THRESHOLD: f64 = 40.0;
const METRIC_STUFFY_TEMPERATURE: f64 = 26.0;
const IMPERIAL_STUFFY_TEMPERATURE: f64 = 78.8;
const METRIC_TEMPERATURE_RANGE: f64 = 10.0;
const IMPERIAL_TEMPERATURE_RANGE: f64 = 18.0;
const METRIC_WIND_THRESHOLD: f64 = 30.0;
const IMPERIAL_WIND_THRESHOLD: f64 = 19.0;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DaySummary {
    pub headline: String,
}

pub fn build_day_summary(
    current_time: &str,
    hourly: &[HourForecast],
    yesterday_max: Option<f64>,
    units: Units,
) -> DaySummary {
    let Some((current_date, current_hour)) = parse_time(current_time) else {
        return DaySummary {
            headline: neutral_headline(false),
        };
    };
    let is_tomorrow = current_hour > TOMORROW_SUMMARY_AFTER_HOUR;
    let target_date = if is_tomorrow {
        next_date(&current_date).unwrap_or_default()
    } else {
        current_date
    };
    let target_hours: Vec<&HourForecast> = hourly
        .iter()
        .filter(|hour| {
            parse_time(&hour.time).is_some_and(|(date, hour_of_day)| {
                date == target_date
                    && hour_of_day < DAY_END_HOUR
                    && (is_tomorrow || hour_of_day >= current_hour)
            })
        })
        .collect();

    if target_hours.is_empty() {
        return DaySummary {
            headline: neutral_headline(is_tomorrow),
        };
    }

    let mut clauses = vec![rain_clause(&target_hours)];
    let temperatures: Vec<f64> = target_hours.iter().map(|hour| hour.temperature).collect();
    let max_temperature = temperatures.iter().copied().fold(f64::MIN, f64::max);
    let min_temperature = temperatures.iter().copied().fold(f64::MAX, f64::min);
    let mean_humidity = target_hours
        .iter()
        .map(|hour| f64::from(hour.humidity))
        .sum::<f64>()
        / target_hours.len() as f64;
    let mean_wind =
        target_hours.iter().map(|hour| hour.wind_speed).sum::<f64>() / target_hours.len() as f64;

    if let Some(yesterday_max) = yesterday_max {
        let change_threshold = if units.is_imperial() {
            IMPERIAL_TEMPERATURE_CHANGE
        } else {
            METRIC_TEMPERATURE_CHANGE
        };
        if max_temperature - yesterday_max >= change_threshold {
            clauses.push("o dia será mais quente que ontem".to_owned());
        } else if yesterday_max - max_temperature >= change_threshold {
            clauses.push("o dia será mais fresco que ontem".to_owned());
        }
    }

    if clauses.len() < MAX_SUMMARY_RULES {
        if mean_humidity >= HIGH_HUMIDITY_THRESHOLD {
            let stuffy_threshold = if units.is_imperial() {
                IMPERIAL_STUFFY_TEMPERATURE
            } else {
                METRIC_STUFFY_TEMPERATURE
            };
            if max_temperature >= stuffy_threshold {
                clauses.push("o dia será úmido e abafado".to_owned());
            } else {
                clauses.push("o dia será úmido".to_owned());
            }
        } else if mean_humidity <= LOW_HUMIDITY_THRESHOLD {
            clauses.push("o ar ficará seco".to_owned());
        }
    }

    if clauses.len() < MAX_SUMMARY_RULES {
        let temperature_range = if units.is_imperial() {
            IMPERIAL_TEMPERATURE_RANGE
        } else {
            METRIC_TEMPERATURE_RANGE
        };
        if max_temperature - min_temperature >= temperature_range {
            clauses.push("a manhã será fresca e a tarde será quente".to_owned());
        }
    }

    if clauses.len() < MAX_SUMMARY_RULES {
        let wind_threshold = if units.is_imperial() {
            IMPERIAL_WIND_THRESHOLD
        } else {
            METRIC_WIND_THRESHOLD
        };
        if mean_wind >= wind_threshold {
            clauses.push("vai ventar bastante".to_owned());
        }
    }

    DaySummary {
        headline: compose_headline(&clauses, is_tomorrow),
    }
}

fn rain_clause(hours: &[&HourForecast]) -> String {
    let mut periods = Vec::new();
    for hour in hours.iter().filter(|hour| {
        hour.precipitation_probability >= RAIN_PROBABILITY_THRESHOLD
            || hour.precipitation >= RELEVANT_PRECIPITATION_MM
    }) {
        if let Some((_, hour_of_day)) = parse_time(&hour.time) {
            if let Some(period) = period_for_hour(hour_of_day) {
                if !periods.contains(&period) {
                    periods.push(period);
                }
            }
        }
    }

    if periods.is_empty() {
        "Sem previsão de chuva".to_owned()
    } else {
        format!("A previsão indica chuva {}", join_periods(&periods))
    }
}

fn period_for_hour(hour: u8) -> Option<&'static str> {
    match hour {
        DAWN_START_HOUR..MORNING_START_HOUR => Some("de madrugada"),
        MORNING_START_HOUR..AFTERNOON_START_HOUR => Some("pela manhã"),
        AFTERNOON_START_HOUR..EVENING_START_HOUR => Some("à tarde"),
        EVENING_START_HOUR..DAY_END_HOUR => Some("à noite"),
        _ => None,
    }
}

fn join_periods(periods: &[&str]) -> String {
    match periods {
        [] => String::new(),
        [period] => (*period).to_owned(),
        [first, second] => format!("{first} e {second}"),
        _ => format!(
            "{} e {}",
            periods[..periods.len() - 1].join(", "),
            periods[periods.len() - 1]
        ),
    }
}

fn compose_headline(clauses: &[String], is_tomorrow: bool) -> String {
    let phrase = match clauses {
        [] => "Dia tranquilo, sem grandes mudanças".to_owned(),
        [first] => first.clone(),
        [first, second, ..] => format!("{first}, e {second}"),
    };
    let phrase = if is_tomorrow {
        format!("Amanhã, {}", lowercase_first(&phrase))
    } else {
        phrase
    };
    format!("{phrase}.")
}

fn lowercase_first(value: &str) -> String {
    let mut chars = value.chars();
    match chars.next() {
        Some(first) => first.to_lowercase().chain(chars).collect(),
        None => String::new(),
    }
}

fn neutral_headline(is_tomorrow: bool) -> String {
    compose_headline(&[], is_tomorrow)
}

fn parse_time(value: &str) -> Option<(String, u8)> {
    let date = value.get(..10)?;
    let hour = value.get(11..13)?.parse().ok()?;
    Some((date.to_owned(), hour))
}

fn next_date(date: &str) -> Option<String> {
    NaiveDate::parse_from_str(date, "%Y-%m-%d")
        .ok()?
        .succ_opt()
        .map(|date| date.format("%Y-%m-%d").to_string())
}

#[cfg(test)]
mod tests {
    use super::build_day_summary;
    use crate::models::{HourForecast, Units};

    const DATE: &str = "2026-11-16";

    fn hours_for_day(date: &str, temperature: f64) -> Vec<HourForecast> {
        (0..24)
            .map(|hour| HourForecast {
                time: format!("{date}T{hour:02}:00"),
                temperature,
                precipitation_probability: 0,
                precipitation: 0.0,
                humidity: 60,
                wind_speed: 10.0,
                weather_code: 0,
                icon: "clear-day".to_owned(),
                is_day: true,
            })
            .collect()
    }

    fn summary(hours: &[HourForecast], yesterday_max: Option<f64>, units: Units) -> String {
        build_day_summary(&format!("{DATE}T06:00"), hours, yesterday_max, units).headline
    }

    #[test]
    fn describes_rain_by_period_and_relevant_precipitation() {
        let mut hours = hours_for_day(DATE, 22.0);
        hours[14].precipitation_probability = 50;
        assert!(summary(&hours, None, Units::Metric).contains("chuva à tarde"));

        let mut hours = hours_for_day(DATE, 22.0);
        hours[8].precipitation = 0.2;
        assert!(summary(&hours, None, Units::Metric).contains("chuva pela manhã"));
    }

    #[test]
    fn compares_maximum_temperature_with_yesterday() {
        let hours = hours_for_day(DATE, 25.0);
        assert!(summary(&hours, Some(22.0), Units::Metric).contains("mais quente que ontem"));
    }

    #[test]
    fn describes_humidity_and_stuffy_heat() {
        let mut hours = hours_for_day(DATE, 26.0);
        for hour in &mut hours {
            hour.humidity = 80;
        }
        let headline = summary(&hours, None, Units::Metric);
        assert!(headline.contains("úmido e abafado"));

        let mut hours = hours_for_day(DATE, 22.0);
        for hour in &mut hours {
            hour.humidity = 40;
        }
        assert!(summary(&hours, None, Units::Metric).contains("ar ficará seco"));
    }

    #[test]
    fn describes_large_temperature_range() {
        let mut hours = hours_for_day(DATE, 20.0);
        hours[6].temperature = 14.0;
        hours[15].temperature = 24.0;

        assert!(summary(&hours, Some(24.0), Units::Metric)
            .contains("manhã será fresca e a tarde será quente"));
    }

    #[test]
    fn describes_high_average_wind() {
        let mut hours = hours_for_day(DATE, 22.0);
        for hour in &mut hours {
            hour.wind_speed = 30.0;
        }

        assert!(summary(&hours, None, Units::Metric).contains("vai ventar bastante"));
    }

    #[test]
    fn combines_rain_and_temperature_comparison_naturally() {
        let mut hours = hours_for_day(DATE, 25.0);
        hours[14].precipitation_probability = 50;
        hours[6].temperature = 14.0;
        for hour in &mut hours {
            hour.humidity = 80;
            hour.wind_speed = 30.0;
        }

        assert_eq!(
            summary(&hours, Some(22.0), Units::Metric),
            "A previsão indica chuva à tarde, e o dia será mais quente que ontem."
        );
    }

    #[test]
    fn describes_tomorrow_after_twenty_hours() {
        let mut hours = hours_for_day(DATE, 22.0);
        let mut tomorrow = hours_for_day("2026-11-17", 22.0);
        tomorrow[15].precipitation_probability = 50;
        hours.append(&mut tomorrow);

        let headline =
            build_day_summary("2026-11-16T21:00", &hours, Some(22.0), Units::Metric).headline;

        assert!(headline.starts_with("Amanhã,"));
        assert!(headline.contains("chuva à tarde"));
    }

    #[test]
    fn applies_fahrenheit_temperature_thresholds() {
        let hours = hours_for_day(DATE, 80.0);

        assert!(summary(&hours, Some(75.0), Units::Imperial).contains("mais quente que ontem"));

        let mut hours = hours_for_day(DATE, 80.0);
        hours[6].temperature = 62.0;
        assert!(summary(&hours, Some(80.0), Units::Imperial)
            .contains("manhã será fresca e a tarde será quente"));

        let mut hours = hours_for_day(DATE, 78.8);
        for hour in &mut hours {
            hour.humidity = 80;
        }
        assert!(summary(&hours, None, Units::Imperial).contains("úmido e abafado"));

        let mut hours = hours_for_day(DATE, 80.0);
        for hour in &mut hours {
            hour.wind_speed = 19.0;
        }
        assert!(summary(&hours, None, Units::Imperial).contains("vai ventar bastante"));
    }

    #[test]
    fn skips_yesterday_comparison_when_its_data_is_missing() {
        let hours = hours_for_day(DATE, 35.0);

        assert_eq!(
            summary(&hours, None, Units::Metric),
            "Sem previsão de chuva."
        );
    }
}
