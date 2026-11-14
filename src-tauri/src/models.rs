use anyhow::{bail, Result};
use chrono::{SecondsFormat, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Units {
    Metric,
    Imperial,
}

impl Units {
    pub fn is_imperial(self) -> bool {
        matches!(self, Self::Imperial)
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct GeocodingResponse {
    pub results: Option<Vec<GeoResult>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GeoResult {
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub country: Option<String>,
    pub admin1: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ForecastResponse {
    #[serde(skip)]
    pub location: GeoResult,
    pub current: CurrentWeather,
    pub daily: DailyWeather,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CurrentWeather {
    pub temperature_2m: f64,
    pub apparent_temperature: f64,
    pub relative_humidity_2m: u8,
    pub wind_speed_10m: f64,
    pub weather_code: u8,
    pub is_day: u8,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
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
#[serde(rename_all = "camelCase")]
pub struct DayForecast {
    pub date: String,
    pub temp_max: f64,
    pub temp_min: f64,
    pub precipitation_probability: u8,
    pub weather_code: u8,
    pub description: String,
    pub icon: String,
    pub sunrise: String,
    pub sunset: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentData {
    pub temperature: f64,
    pub apparent_temperature: f64,
    pub humidity: u8,
    pub wind_speed: f64,
    pub weather_code: u8,
    pub is_day: bool,
    pub description: String,
    pub icon: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WeatherData {
    pub location: GeoResult,
    pub current: CurrentData,
    pub daily: Vec<DayForecast>,
    pub units: Units,
    pub fetched_at: String,
}

impl ForecastResponse {
    pub fn into_weather_data(self, units: Units) -> Result<WeatherData> {
        let current_is_day = self.current.is_day != 0;
        let (current_description, current_icon) =
            crate::weather_code::describe(self.current.weather_code, current_is_day);
        let daily = self.daily.to_day_forecasts()?;

        Ok(WeatherData {
            location: self.location,
            current: CurrentData {
                temperature: self.current.temperature_2m,
                apparent_temperature: self.current.apparent_temperature,
                humidity: self.current.relative_humidity_2m,
                wind_speed: self.current.wind_speed_10m,
                weather_code: self.current.weather_code,
                is_day: current_is_day,
                description: current_description.to_owned(),
                icon: current_icon.to_owned(),
            },
            daily,
            units,
            fetched_at: Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true),
        })
    }
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
            .map(|index| {
                let code = self.weather_code[index];
                let (description, icon) = crate::weather_code::describe(code, true);
                DayForecast {
                    date: self.time[index].clone(),
                    temp_max: self.temperature_2m_max[index],
                    temp_min: self.temperature_2m_min[index],
                    precipitation_probability: self.precipitation_probability_max[index],
                    weather_code: code,
                    description: description.to_owned(),
                    icon: icon.to_owned(),
                    sunrise: self.sunrise[index].clone(),
                    sunset: self.sunset[index].clone(),
                }
            })
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::{CurrentData, DailyWeather, DayForecast, GeoResult, Units, WeatherData};

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
        assert_eq!(days[0].temp_min, 18.0);
        assert_eq!(days[0].description, "Chuva fraca");
        assert_eq!(days[0].icon, "rain");
        assert_eq!(days[1].precipitation_probability, 20);
    }

    #[test]
    fn serializes_weather_data_using_the_frontend_contract() {
        let data = WeatherData {
            location: GeoResult {
                name: "São Paulo".to_owned(),
                latitude: -23.55,
                longitude: -46.63,
                country: Some("Brasil".to_owned()),
                admin1: Some("São Paulo".to_owned()),
            },
            current: CurrentData {
                temperature: 20.0,
                apparent_temperature: 19.0,
                humidity: 60,
                wind_speed: 10.0,
                weather_code: 3,
                is_day: true,
                description: "Nublado".to_owned(),
                icon: "cloudy".to_owned(),
            },
            daily: vec![DayForecast {
                date: "2026-11-14".to_owned(),
                temp_max: 26.0,
                temp_min: 18.0,
                precipitation_probability: 70,
                weather_code: 61,
                description: "Chuva fraca".to_owned(),
                icon: "rain".to_owned(),
                sunrise: "2026-11-14T05:30".to_owned(),
                sunset: "2026-11-14T18:20".to_owned(),
            }],
            units: Units::Imperial,
            fetched_at: "2026-11-14T09:00:00Z".to_owned(),
        };
        let value = serde_json::to_value(data).expect("dados serializáveis");

        assert_eq!(value["location"]["admin1"], "São Paulo");
        assert_eq!(value["current"]["apparentTemperature"], 19.0);
        assert_eq!(value["current"]["isDay"], true);
        assert_eq!(value["daily"][0]["tempMax"], 26.0);
        assert_eq!(value["daily"][0]["precipitationProbability"], 70);
        assert_eq!(value["units"], "imperial");
        assert_eq!(value["fetchedAt"], "2026-11-14T09:00:00Z");
    }
}
