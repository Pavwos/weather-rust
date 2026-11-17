mod api;
mod commands;
mod config;
mod models;
pub mod rain_alert;
mod summary;
mod weather_code;

use std::time::Duration;

use anyhow::{anyhow, Context};
use tauri::{
    image::Image,
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager, WindowEvent,
};

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
            build_tray(app)?;
            start_rain_alert_scheduler(app.handle().clone());
            Ok(())
        })
        .on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { api, .. } = event {
                match crate::config::Config::load() {
                    Ok(config) if config.rain_alerts => {
                        api.prevent_close();
                        if let Err(error) = window.hide() {
                            eprintln!("Não foi possível ocultar a janela do Clima: {error}");
                        }
                    }
                    Ok(_) => {}
                    Err(error) => {
                        eprintln!("Não foi possível ler a configuração ao fechar: {error:#}");
                    }
                }
            }
        })
        .run(tauri::generate_context!());

    if let Err(error) = result {
        eprintln!("Não foi possível iniciar o Clima: {error}");
        std::process::exit(1);
    }
}

fn build_tray(app: &mut tauri::App) -> tauri::Result<()> {
    let open = tauri::menu::MenuItem::with_id(app, "open", "Abrir", true, None::<&str>)?;
    let quit = tauri::menu::MenuItem::with_id(app, "quit", "Sair", true, None::<&str>)?;
    let menu = tauri::menu::Menu::with_items(app, &[&open, &quit])?;
    let icon = Image::from_bytes(include_bytes!("../icons/icon.png"))?;

    TrayIconBuilder::new()
        .icon(icon)
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| match event.id().as_ref() {
            "open" => report_tray_result(show_main_window(app)),
            "quit" => app.exit(0),
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if matches!(
                event,
                TrayIconEvent::Click {
                    button: MouseButton::Left,
                    button_state: MouseButtonState::Up,
                    ..
                }
            ) {
                report_tray_result(toggle_main_window(tray.app_handle()));
            }
        })
        .build(app)?;
    Ok(())
}

fn show_main_window(app: &AppHandle) -> anyhow::Result<()> {
    let window = app
        .get_webview_window("main")
        .ok_or_else(|| anyhow!("A janela principal do Clima não foi encontrada"))?;
    window
        .show()
        .context("Não foi possível mostrar a janela do Clima")?;
    window
        .set_focus()
        .context("Não foi possível focar a janela do Clima")
}

fn toggle_main_window(app: &AppHandle) -> anyhow::Result<()> {
    let window = app
        .get_webview_window("main")
        .ok_or_else(|| anyhow!("A janela principal do Clima não foi encontrada"))?;
    let is_visible = window
        .is_visible()
        .context("Não foi possível verificar a visibilidade da janela")?;
    let is_minimized = window
        .is_minimized()
        .context("Não foi possível verificar se a janela está minimizada")?;
    if is_visible && !is_minimized {
        window
            .hide()
            .context("Não foi possível ocultar a janela do Clima")
    } else {
        show_main_window(app)
    }
}

fn report_tray_result(result: anyhow::Result<()>) {
    if let Err(error) = result {
        eprintln!("Não foi possível executar uma ação da bandeja: {error:#}");
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
