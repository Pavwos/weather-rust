use crate::{
    api,
    config::Config,
    models::{CurrentWeather, DayForecast, ForecastResponse, GeoResult},
    ui::{self, UiAction},
    Units,
};
use anyhow::{Context, Result};
use chrono::{DateTime, Local};
use eframe::egui;
use std::{
    sync::mpsc::{self, Receiver, Sender},
    time::{Duration, Instant},
};
use tokio::runtime::Runtime;

const FORECAST_DAYS: u8 = 5;
const REFRESH_INTERVAL: Duration = Duration::from_secs(15 * 60);

pub enum LoadState {
    Idle,
    Loading,
    Loaded(WeatherData),
    Error(String),
}

pub struct WeatherData {
    pub location: GeoResult,
    pub current: CurrentWeather,
    pub days: Vec<DayForecast>,
    pub updated_at: DateTime<Local>,
}

impl WeatherData {
    fn from_response(response: ForecastResponse) -> Result<Self> {
        let days = response.daily.to_day_forecasts()?;
        Ok(Self {
            location: response.location,
            current: response.current,
            days,
            updated_at: Local::now(),
        })
    }
}

pub struct ClimaApp {
    runtime: Runtime,
    config: Config,
    city_input: String,
    units: Units,
    state: LoadState,
    sender: Sender<(u64, Result<WeatherData, String>)>,
    receiver: Receiver<(u64, Result<WeatherData, String>)>,
    request_id: u64,
    next_refresh: Instant,
}

impl ClimaApp {
    pub fn new(
        creation_context: &eframe::CreationContext<'_>,
        runtime: Runtime,
        config: Config,
    ) -> Self {
        let (sender, receiver) = mpsc::channel();
        let units = Units::from_config(&config.units);
        let initial_city = config
            .last_city
            .clone()
            .or_else(|| config.default_city.clone());
        let mut app = Self {
            runtime,
            config,
            city_input: initial_city.clone().unwrap_or_default(),
            units,
            state: LoadState::Idle,
            sender,
            receiver,
            request_id: 0,
            next_refresh: Instant::now() + REFRESH_INTERVAL,
        };

        if let Some(city) = initial_city {
            app.search(city, creation_context.egui_ctx.clone());
        }
        app
    }

    fn search(&mut self, city: String, context: egui::Context) {
        let city = city.trim().to_owned();
        if city.is_empty() {
            self.state = LoadState::Error("Digite o nome de uma cidade para buscar.".to_owned());
            return;
        }

        self.city_input.clone_from(&city);
        self.config.last_city = Some(city.clone());
        self.config.units = self.units.as_config_value().to_owned();
        if let Err(error) = self.config.save() {
            self.state = LoadState::Error(format!(
                "Não foi possível salvar as preferências: {error:#}"
            ));
            return;
        }

        self.request_id = self.request_id.wrapping_add(1);
        let request_id = self.request_id;
        let sender = self.sender.clone();
        let units = self.units;
        self.state = LoadState::Loading;
        self.next_refresh = Instant::now() + REFRESH_INTERVAL;

        self.runtime.spawn(async move {
            let result = async {
                let response = api::fetch_forecast(&city, units, FORECAST_DAYS).await?;
                WeatherData::from_response(response)
                    .with_context(|| "Não foi possível processar a previsão".to_owned())
            }
            .await
            .map_err(|error: anyhow::Error| format!("{error:#}"));

            let _ = sender.send((request_id, result));
            context.request_repaint();
        });
    }

    fn handle_action(&mut self, action: UiAction, context: &egui::Context) {
        match action {
            UiAction::Search => self.search(self.city_input.clone(), context.clone()),
            UiAction::Refresh => {
                let city = match &self.state {
                    LoadState::Loaded(data) => Some(data.location.name.clone()),
                    _ => Some(self.city_input.clone()),
                };
                if let Some(city) = city {
                    self.search(city, context.clone());
                }
            }
            UiAction::SetUnits(units) if units != self.units => {
                self.units = units;
                let city = self.city_input.clone();
                if city.trim().is_empty() {
                    self.config.units = units.as_config_value().to_owned();
                    if let Err(error) = self.config.save() {
                        self.state = LoadState::Error(format!(
                            "Não foi possível salvar as preferências: {error:#}"
                        ));
                    }
                } else {
                    self.search(city, context.clone());
                }
            }
            UiAction::SetUnits(_) => {}
            UiAction::SaveDefaultCity => {
                let default_city = match &self.state {
                    LoadState::Loaded(data) => Some(data.location.name.clone()),
                    _ => None,
                };
                if let Some(default_city) = default_city {
                    self.config.default_city = Some(default_city);
                    match self.config.save() {
                        Ok(()) => {}
                        Err(error) => {
                            self.state = LoadState::Error(format!(
                                "Não foi possível salvar a cidade padrão: {error:#}"
                            ));
                        }
                    }
                }
            }
        }
    }

    fn receive_result(&mut self) {
        while let Ok((request_id, result)) = self.receiver.try_recv() {
            if request_id != self.request_id {
                continue;
            }
            match result {
                Ok(data) => {
                    self.next_refresh = Instant::now() + REFRESH_INTERVAL;
                    self.state = LoadState::Loaded(data);
                }
                Err(message) => self.state = LoadState::Error(message),
            }
        }
    }

    fn refresh_if_due(&mut self, context: &egui::Context) {
        if Instant::now() < self.next_refresh {
            return;
        }
        if let LoadState::Loaded(data) = &self.state {
            let city = data.location.name.clone();
            self.search(city, context.clone());
        }
    }
}

impl eframe::App for ClimaApp {
    fn update(&mut self, context: &egui::Context, _frame: &mut eframe::Frame) {
        self.receive_result();
        self.refresh_if_due(context);
        context.request_repaint_after(self.next_refresh.saturating_duration_since(Instant::now()));

        egui::CentralPanel::default().show(context, |ui| {
            let actions = ui::draw(ui, &mut self.city_input, self.units, &self.state);
            for action in actions {
                self.handle_action(action, context);
            }
        });
    }
}
