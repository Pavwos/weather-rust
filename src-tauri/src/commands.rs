use crate::{
    api,
    config::Config,
    models::{Units, WeatherData},
};
use serde::Serialize;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ApiErrorKind {
    NotFound,
    Network,
    Timeout,
    Unknown,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ApiError {
    pub kind: ApiErrorKind,
    pub message: String,
}

impl ApiError {
    fn from_anyhow(error: anyhow::Error) -> Self {
        let details = format!("{error:#}");
        let kind = if details.contains("Cidade não encontrada") {
            ApiErrorKind::NotFound
        } else if details.contains("Tempo limite de 10 segundos") {
            ApiErrorKind::Timeout
        } else if error.chain().any(|cause| {
            cause
                .downcast_ref::<reqwest::Error>()
                .is_some_and(|request_error| request_error.is_connect())
        }) {
            ApiErrorKind::Network
        } else {
            ApiErrorKind::Unknown
        };
        let message = match kind {
            ApiErrorKind::NotFound => "Cidade não encontrada. Confira o nome e tente novamente.",
            ApiErrorKind::Network => {
                "Sem conexão com a internet. Verifique sua rede e tente novamente."
            }
            ApiErrorKind::Timeout => {
                "A busca demorou mais que o esperado. Tente novamente em instantes."
            }
            ApiErrorKind::Unknown => "Não foi possível concluir a operação. Tente novamente.",
        };

        Self {
            kind,
            message: message.to_owned(),
        }
    }

    fn invalid_config(message: &str) -> Self {
        Self {
            kind: ApiErrorKind::Unknown,
            message: message.to_owned(),
        }
    }
}

#[tauri::command]
pub async fn fetch_weather(city: String, units: String) -> Result<WeatherData, ApiError> {
    let units = match units.as_str() {
        "metric" => Units::Metric,
        "imperial" => Units::Imperial,
        _ => {
            return Err(ApiError::invalid_config(
                "Unidade inválida. Escolha Celsius ou Fahrenheit.",
            ))
        }
    };
    let forecast = api::fetch_forecast(&city, units, 5)
        .await
        .map_err(ApiError::from_anyhow)?;
    forecast
        .into_weather_data(units)
        .map_err(ApiError::from_anyhow)
}

#[tauri::command]
pub async fn get_config() -> Result<Config, ApiError> {
    Config::load().map_err(ApiError::from_anyhow)
}

#[tauri::command]
pub async fn save_config(mut config: Config) -> Result<(), ApiError> {
    if !matches!(config.units.as_str(), "metric" | "imperial") {
        return Err(ApiError::invalid_config(
            "Unidade inválida. Escolha Celsius ou Fahrenheit.",
        ));
    }
    config.default_city = trim_optional_city(config.default_city);
    config.last_city = trim_optional_city(config.last_city);
    config.save().map_err(ApiError::from_anyhow)
}

#[tauri::command]
pub async fn set_default_city(city: String) -> Result<(), ApiError> {
    let city = city.trim();
    if city.is_empty() {
        return Err(ApiError::invalid_config(
            "Digite uma cidade antes de salvá-la como padrão.",
        ));
    }

    let mut config = Config::load().map_err(ApiError::from_anyhow)?;
    config.default_city = Some(city.to_owned());
    config.save().map_err(ApiError::from_anyhow)
}

fn trim_optional_city(city: Option<String>) -> Option<String> {
    city.and_then(|value| {
        let value = value.trim();
        (!value.is_empty()).then(|| value.to_owned())
    })
}

#[cfg(test)]
mod tests {
    use super::{ApiError, ApiErrorKind};

    #[test]
    fn serializes_api_error_with_kind_and_friendly_message() {
        let error = ApiError::from_anyhow(anyhow::anyhow!("Cidade não encontrada: São Paulo"));
        let value = serde_json::to_value(&error).expect("erro serializável");

        assert_eq!(error.kind, ApiErrorKind::NotFound);
        assert_eq!(value["kind"], "not_found");
        assert_eq!(
            value["message"],
            "Cidade não encontrada. Confira o nome e tente novamente."
        );
    }
}
