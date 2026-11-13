use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct GeocodingResponse {
    pub results: Option<Vec<GeoResult>>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct GeoResult {
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub country: Option<String>,
    pub admin1: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ForecastResponse {
    #[serde(skip)]
    pub location: GeoResult,
    pub current: CurrentWeather,
    pub daily: DailyWeather,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CurrentWeather {
    pub temperature_2m: f64,
    pub apparent_temperature: f64,
    pub relative_humidity_2m: u8,
    pub wind_speed_10m: f64,
    pub weather_code: u8,
    pub is_day: u8,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DailyWeather {
    pub time: Vec<String>,
    pub weather_code: Vec<u8>,
    pub temperature_2m_max: Vec<f64>,
    pub temperature_2m_min: Vec<f64>,
    pub precipitation_probability_max: Vec<u8>,
    pub sunrise: Vec<String>,
    pub sunset: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DayForecast {
    pub date: String,
    pub weather_code: u8,
    pub temperature_max: f64,
    pub temperature_min: f64,
    pub precipitation_probability: u8,
    pub sunrise: String,
    pub sunset: String,
}

impl DailyWeather {
    pub fn to_day_forecasts(&self) -> Result<Vec<DayForecast>> {
        let length = self.time.len();
        if [
            self.weather_code.len(),
            self.temperature_2m_max.len(),
            self.temperature_2m_min.len(),
            self.precipitation_probability_max.len(),
            self.sunrise.len(),
            self.sunset.len(),
        ]
        .iter()
        .any(|&field_length| field_length != length)
        {
            bail!("A resposta diária da previsão contém arrays com tamanhos diferentes");
        }

        Ok((0..length)
            .map(|index| DayForecast {
                date: self.time[index].clone(),
                weather_code: self.weather_code[index],
                temperature_max: self.temperature_2m_max[index],
                temperature_min: self.temperature_2m_min[index],
                precipitation_probability: self.precipitation_probability_max[index],
                sunrise: self.sunrise[index].clone(),
                sunset: self.sunset[index].clone(),
            })
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::DailyWeather;

    #[test]
    fn converts_daily_arrays_into_day_forecasts() {
        let json = r#"{
            "time":["2026-11-14","2026-11-15"],
            "weather_code":[61,3],
            "temperature_2m_max":[26.0,25.0],
            "temperature_2m_min":[18.0,17.0],
            "precipitation_probability_max":[70,20],
            "sunrise":["2026-11-14T05:30","2026-11-15T05:31"],
            "sunset":["2026-11-14T18:20","2026-11-15T18:20"]
        }"#;
        let daily: DailyWeather = serde_json::from_str(json).expect("JSON de teste válido");
        let days = daily.to_day_forecasts().expect("arrays alinhados");

        assert_eq!(days.len(), 2);
        assert_eq!(days[0].date, "2026-11-14");
        assert_eq!(days[0].temperature_min, 18.0);
        assert_eq!(days[1].precipitation_probability, 20);
    }
}
