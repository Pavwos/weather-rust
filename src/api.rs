use anyhow::{anyhow, Context, Result};
use reqwest::Client;

use crate::{
    models::{ForecastResponse, GeocodingResponse},
    Units,
};

const GEOCODING_URL: &str = "https://geocoding-api.open-meteo.com/v1/search";
const FORECAST_URL: &str = "https://api.open-meteo.com/v1/forecast";

pub async fn fetch_forecast(city: &str, units: Units, days: u8) -> Result<ForecastResponse> {
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
        ("timezone", "auto".to_owned()),
        ("forecast_days", days.to_string()),
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
    Ok(forecast)
}

fn request_error(error: reqwest::Error, context: &str) -> anyhow::Error {
    if error.is_timeout() {
        anyhow!("Tempo limite de 10 segundos excedido. Verifique sua conexão e tente novamente.")
    } else {
        anyhow::Error::new(error).context(context.to_owned())
    }
}
