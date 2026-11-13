#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod api;
mod app;
mod config;
mod models;
mod theme;
mod ui;
mod weather_code;

use anyhow::{Context, Result};
use eframe::egui;

#[cfg(clima_icon_asset)]
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Units {
    Metric,
    Imperial,
}

impl Units {
    pub fn from_config(value: &str) -> Self {
        match value {
            "imperial" => Self::Imperial,
            _ => Self::Metric,
        }
    }

    pub fn as_config_value(self) -> &'static str {
        match self {
            Self::Metric => "metric",
            Self::Imperial => "imperial",
        }
    }

    pub fn is_imperial(self) -> bool {
        matches!(self, Self::Imperial)
    }
}

fn main() -> Result<()> {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .context("Não foi possível iniciar o runtime assíncrono")?;

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([420.0, 640.0])
            .with_min_inner_size([380.0, 520.0])
            .with_resizable(true),
        ..Default::default()
    };

    #[cfg(clima_icon_asset)]
    let options = {
        let mut options = options;
        if let Ok(icon) = eframe::icon_data::from_png_bytes(include_bytes!("../assets/icon.png")) {
            options.viewport = options.viewport.with_icon(Arc::new(icon));
        }
        options
    };

    eframe::run_native(
        "Clima",
        options,
        Box::new(move |creation_context| {
            theme::apply(&creation_context.egui_ctx);
            let config = config::Config::load()?;
            Ok(Box::new(app::ClimaApp::new(
                creation_context,
                runtime,
                config,
            )))
        }),
    )
    .map_err(|error| anyhow::anyhow!("Não foi possível iniciar a janela: {error}"))
}
