use anyhow::{bail, Result};
use chrono::{SecondsFormat, Utc};
use serde::{Deserialize, Serialize};

const HOURLY_FORECAST_HOURS: usize = 24;

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
    pub hourly: HourlyWeather,
    pub daily: DailyWeather,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CurrentWeather {
    pub time: String,
    pub temperature_2m: f64,
    pub apparent_temperature: f64,
    pub relative_humidity_2m: u8,
    pub wind_speed_10m: f64,
    pub weather_code: u8,
    pub is_day: u8,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HourlyWeather {
    pub time: Vec<String>,
    pub temperature_2m: Vec<f64>,
    pub precipitation_probability: Vec<u8>,
    pub precipitation: Vec<f64>,
    pub relative_humidity_2m: Vec<u8>,
    pub weather_code: Vec<u8>,
    pub is_day: Vec<u8>,
    pub wind_speed_10m: Vec<f64>,
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

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HourForecast {
    pub time: String,
    pub temperature: f64,
    pub precipitation_probability: u8,
    pub precipitation: f64,
    pub humidity: u8,
    pub weather_code: u8,
    pub icon: String,
    pub is_day: bool,
    pub wind_speed: f64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WeatherData {
    pub location: GeoResult,
    pub current: CurrentData,
    pub summary: crate::summary::DaySummary,
    pub hourly: Vec<HourForecast>,
    pub daily: Vec<DayForecast>,
    pub units: Units,
    pub fetched_at: String,
}

impl ForecastResponse {
    pub fn into_weather_data(self, units: Units) -> Result<WeatherData> {
        let current_is_day = self.current.is_day != 0;
        let (current_description, current_icon) =
            crate::weather_code::describe(self.current.weather_code, current_is_day);
        let hourly = self.hourly.to_hour_forecasts(&self.current.time)?;
        let all_hourly = self.hourly.to_all_hour_forecasts()?;
        let yesterday_max = self.daily.yesterday_max()?;
        let summary = crate::summary::build_day_summary(
            &self.current.time,
            &all_hourly,
            yesterday_max,
            units,
        );
        let daily = self.daily.to_day_forecasts()?.into_iter().skip(1).collect();

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
            summary,
            hourly,
            daily,
            units,
            fetched_at: Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true),
        })
    }
}

impl HourlyWeather {
    pub fn to_hour_forecasts(&self, current_time: &str) -> Result<Vec<HourForecast>> {
        next_hour_forecasts(&self.to_all_hour_forecasts()?, current_time)
    }

    pub fn to_all_hour_forecasts(&self) -> Result<Vec<HourForecast>> {
        let length = self.time.len();
        if [
            self.temperature_2m.len(),
            self.precipitation_probability.len(),
            self.precipitation.len(),
            self.relative_humidity_2m.len(),
            self.weather_code.len(),
            self.is_day.len(),
            self.wind_speed_10m.len(),
        ]
        .iter()
        .any(|&field_length| field_length != length)
        {
            bail!("A resposta horária da previsão contém arrays com tamanhos diferentes");
        }

        Ok((0..length)
            .map(|index| {
                let weather_code = self.weather_code[index];
                let is_day = self.is_day[index] != 0;
                let (_, icon) = crate::weather_code::describe(weather_code, is_day);
                HourForecast {
                    time: self.time[index].clone(),
                    temperature: self.temperature_2m[index],
                    precipitation_probability: self.precipitation_probability[index],
                    precipitation: self.precipitation[index],
                    humidity: self.relative_humidity_2m[index],
                    weather_code,
                    icon: icon.to_owned(),
                    is_day,
                    wind_speed: self.wind_speed_10m[index],
                }
            })
            .collect())
    }
}

fn next_hour_forecasts(hourly: &[HourForecast], current_time: &str) -> Result<Vec<HourForecast>> {
    let start = hourly
        .iter()
        .position(|hour| hour.time == current_time)
        .ok_or_else(|| {
            anyhow::anyhow!("O horário atual não foi encontrado na previsão por hora")
        })?;

    Ok(hourly
        .iter()
        .skip(start)
        .take(HOURLY_FORECAST_HOURS)
        .cloned()
        .collect())
}

impl DailyWeather {
    pub fn to_day_forecasts(&self) -> Result<Vec<DayForecast>> {
        self.to_day_forecasts_from(0)
    }

    fn to_day_forecasts_from(&self, start: usize) -> Result<Vec<DayForecast>> {
        self.validate_lengths()?;

        Ok((start.min(self.time.len())..self.time.len())
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

    fn yesterday_max(&self) -> Result<Option<f64>> {
        self.validate_lengths()?;
        Ok(self.temperature_2m_max.first().copied())
    }

    fn validate_lengths(&self) -> Result<()> {
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
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CurrentData, DailyWeather, DayForecast, GeoResult, HourlyWeather, Units, WeatherData,
    };

    const HOURLY_JSON: &str = r#"{
        "time":["2026-11-16T09:00","2026-11-16T10:00"],
        "temperature_2m":[18.5,20.0],
        "precipitation_probability":[10,35],
        "precipitation":[0.0,0.2],
        "relative_humidity_2m":[62,58],
        "weather_code":[0,61],
        "is_day":[1,1],
        "wind_speed_10m":[10.0,11.0]
    }"#;

    const FORECAST_JSON: &str = r#"{
        "current":{
            "time":"2026-11-16T00:00",
            "temperature_2m":18.0,
            "apparent_temperature":18.0,
            "relative_humidity_2m":60,
            "wind_speed_10m":10.0,
            "weather_code":0,
            "is_day":0
        },
        "hourly":{
            "time":[
                "2026-11-16T00:00","2026-11-16T01:00","2026-11-16T02:00",
                "2026-11-16T03:00","2026-11-16T04:00","2026-11-16T05:00",
                "2026-11-16T06:00","2026-11-16T07:00","2026-11-16T08:00",
                "2026-11-16T09:00","2026-11-16T10:00","2026-11-16T11:00",
                "2026-11-16T12:00","2026-11-16T13:00","2026-11-16T14:00",
                "2026-11-16T15:00","2026-11-16T16:00","2026-11-16T17:00",
                "2026-11-16T18:00","2026-11-16T19:00","2026-11-16T20:00",
                "2026-11-16T21:00","2026-11-16T22:00","2026-11-16T23:00",
                "2026-11-17T00:00","2026-11-17T01:00"
            ],
            "temperature_2m":[18.0,18.0,18.0,18.0,18.0,18.0,18.0,18.0,18.0,18.0,18.0,18.0,18.0,18.0,18.0,18.0,18.0,18.0,18.0,18.0,18.0,18.0,18.0,18.0,18.0,18.0],
            "precipitation_probability":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
            "precipitation":[0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0],
            "relative_humidity_2m":[60,60,60,60,60,60,60,60,60,60,60,60,60,60,60,60,60,60,60,60,60,60,60,60,60,60],
            "weather_code":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
            "is_day":[0,0,0,0,0,0,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0,0,0,0,0,0],
            "wind_speed_10m":[10.0,10.0,10.0,10.0,10.0,10.0,10.0,10.0,10.0,10.0,10.0,10.0,10.0,10.0,10.0,10.0,10.0,10.0,10.0,10.0,10.0,10.0,10.0,10.0,10.0,10.0]
        },
        "daily":{
            "time":[],"weather_code":[],"temperature_2m_max":[],
            "temperature_2m_min":[],"precipitation_probability_max":[],
            "sunrise":[],"sunset":[]
        }
    }"#;

    const MIDDLE_TIME_FORECAST_JSON: &str = r#"{
        "current":{
            "time":"2026-11-16T10:00",
            "temperature_2m":20.0,
            "apparent_temperature":20.0,
            "relative_humidity_2m":60,
            "wind_speed_10m":10.0,
            "weather_code":0,
            "is_day":1
        },
        "hourly":{
            "time":["2026-11-15T22:00","2026-11-15T23:00","2026-11-16T08:00","2026-11-16T09:00","2026-11-16T10:00","2026-11-16T11:00"],
            "temperature_2m":[18.0,18.0,18.0,19.0,20.0,21.0],
            "precipitation_probability":[0,0,0,0,10,20],
            "precipitation":[0.0,0.0,0.0,0.0,0.0,0.1],
            "relative_humidity_2m":[65,65,65,62,60,58],
            "weather_code":[0,0,0,0,0,61],
            "is_day":[0,0,1,1,1,1],
            "wind_speed_10m":[10.0,10.0,10.0,10.0,10.0,10.0]
        },
        "daily":{
            "time":["2026-11-15","2026-11-16","2026-11-17"],
            "weather_code":[0,61,3],
            "temperature_2m_max":[30.0,26.0,24.0],
            "temperature_2m_min":[20.0,18.0,17.0],
            "precipitation_probability_max":[0,50,20],
            "sunrise":["2026-11-15T05:30","2026-11-16T05:31","2026-11-17T05:32"],
            "sunset":["2026-11-15T18:20","2026-11-16T18:20","2026-11-17T18:21"]
        }
    }"#;

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
    fn converts_hourly_arrays_into_hour_forecasts() {
        let hourly: HourlyWeather =
            serde_json::from_str(HOURLY_JSON).expect("JSON de teste válido");
        let hours = hourly
            .to_hour_forecasts("2026-11-16T09:00")
            .expect("arrays alinhados");

        assert_eq!(hours.len(), 2);
        assert_eq!(hours[0].time, "2026-11-16T09:00");
        assert_eq!(hours[0].temperature, 18.5);
        assert_eq!(hours[0].precipitation_probability, 10);
        assert_eq!(hours[0].precipitation, 0.0);
        assert_eq!(hours[0].humidity, 62);
        assert_eq!(hours[0].weather_code, 0);
        assert_eq!(hours[0].icon, "clear-day");
        assert!(hours[0].is_day);
        assert_eq!(hours[0].wind_speed, 10.0);
        assert_eq!(hours[1].icon, "rain");
    }

    #[test]
    fn returns_only_the_next_24_hours_starting_at_current_time() {
        let forecast: super::ForecastResponse =
            serde_json::from_str(FORECAST_JSON).expect("JSON de teste válido");
        let data = forecast
            .into_weather_data(Units::Metric)
            .expect("previsão válida");

        assert_eq!(data.hourly.len(), 24);
        assert_eq!(
            data.hourly.first().map(|hour| hour.time.as_str()),
            Some("2026-11-16T00:00")
        );
        assert_eq!(
            data.hourly.last().map(|hour| hour.time.as_str()),
            Some("2026-11-16T23:00")
        );
    }

    #[test]
    fn starts_hourly_forecast_at_current_time_in_the_middle_of_the_list() {
        let forecast: super::ForecastResponse =
            serde_json::from_str(MIDDLE_TIME_FORECAST_JSON).expect("JSON de teste válido");
        let data = forecast
            .into_weather_data(Units::Metric)
            .expect("previsão válida");

        assert_eq!(data.hourly.len(), 2);
        assert_eq!(data.hourly[0].time, "2026-11-16T10:00");
        assert_eq!(data.hourly[0].temperature, 20.0);
        assert_eq!(data.hourly[1].time, "2026-11-16T11:00");
        assert_eq!(data.daily.len(), 2);
        assert_eq!(data.daily[0].date, "2026-11-16");
        assert_eq!(
            data.summary.headline,
            "Sem previsão de chuva, e o dia será mais fresco que ontem."
        );
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
            summary: crate::summary::DaySummary {
                headline: "Dia tranquilo.".to_owned(),
            },
            hourly: Vec::new(),
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
        assert_eq!(value["summary"]["headline"], "Dia tranquilo.");
        assert_eq!(value["hourly"], serde_json::json!([]));
        assert_eq!(value["daily"][0]["tempMax"], 26.0);
        assert_eq!(value["daily"][0]["precipitationProbability"], 70);
        assert_eq!(value["units"], "imperial");
        assert_eq!(value["fetchedAt"], "2026-11-14T09:00:00Z");
    }
}
