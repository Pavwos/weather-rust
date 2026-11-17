mod api;
mod commands;
mod config;
mod models;
pub mod rain_alert;
mod summary;
mod weather_code;

use std::time::Duration;

use tauri::Manager;

pub fn run() {
    let result = tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .invoke_handler(tauri::generate_handler![
            commands::fetch_weather,
            commands::check_rain_alerts,
            commands::get_config,
            commands::save_config,
            commands::set_default_city
        ])
        .setup(|app| {
            app.manage(rain_alert::RainAlertState::default());
            start_rain_alert_scheduler(app.handle().clone());
            Ok(())
        })
        .run(tauri::generate_context!());

    if let Err(error) = result {
        eprintln!("Não foi possível iniciar o Clima: {error}");
        std::process::exit(1);
    }
}

fn start_rain_alert_scheduler(app: tauri::AppHandle) {
    tauri::async_runtime::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(15 * 60));
        loop {
            interval.tick().await;
            if let Err(error) = rain_alert::check_last_city(&app).await {
                eprintln!("Não foi possível verificar o aviso de chuva: {error:#}");
            }
        }
    });
}
