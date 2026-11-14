use anyhow::{Context, Result};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct Config {
    pub default_city: Option<String>,
    pub units: String,
    pub last_city: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            default_city: None,
            units: "metric".to_owned(),
            last_city: None,
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let path = config_path()?;
        let contents = match fs::read_to_string(&path) {
            Ok(contents) => contents,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                return Ok(Self::default())
            }
            Err(error) => {
                return Err(error).with_context(|| {
                    format!("Não foi possível ler a configuração em {}", path.display())
                })
            }
        };
        Ok(Self::parse_or_default(&contents))
    }

    pub fn save(&self) -> Result<()> {
        self.save_to(&config_path()?)
    }

    fn save_to(&self, path: &PathBuf) -> Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Não foi possível criar {}", parent.display()))?;
        }
        let contents =
            toml::to_string_pretty(self).context("Não foi possível serializar a configuração")?;
        fs::write(path, contents).with_context(|| {
            format!(
                "Não foi possível salvar a configuração em {}",
                path.display()
            )
        })
    }

    fn parse_or_default(contents: &str) -> Self {
        toml::from_str(contents).unwrap_or_default()
    }
}

fn config_path() -> Result<PathBuf> {
    let directories = ProjectDirs::from("", "", "weather-cli")
        .context("Não foi possível localizar a pasta de configuração do sistema")?;
    Ok(directories.config_dir().join("config.toml"))
}

#[cfg(test)]
mod tests {
    use super::Config;

    #[test]
    fn reads_old_configuration_without_last_city() {
        let config = Config::parse_or_default("default_city = \"Curitiba\"\nunits = \"imperial\"");
        assert_eq!(config.default_city.as_deref(), Some("Curitiba"));
        assert_eq!(config.units, "imperial");
        assert_eq!(config.last_city, None);
    }

    #[test]
    fn falls_back_to_defaults_for_corrupted_configuration() {
        let config = Config::parse_or_default("units = [not valid toml");
        assert_eq!(config.default_city, None);
        assert_eq!(config.units, "metric");
        assert_eq!(config.last_city, None);
    }

    #[test]
    fn uses_weather_cli_project_directory() {
        let path = super::config_path().expect("diretório de configuração disponível");
        assert_eq!(
            path.parent()
                .and_then(|parent| parent.parent())
                .and_then(|parent| parent.file_name())
                .and_then(|name| name.to_str()),
            Some("weather-cli")
        );
        assert_eq!(
            path.file_name().and_then(|name| name.to_str()),
            Some("config.toml")
        );
    }
}
