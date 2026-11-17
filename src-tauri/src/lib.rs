mod api;
mod commands;
mod config;
mod models;
mod summary;
mod weather_code;

pub fn run() {
    let result = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::fetch_weather,
            commands::get_config,
            commands::save_config,
            commands::set_default_city
        ])
        .run(tauri::generate_context!());

    if let Err(error) = result {
        eprintln!("Não foi possível iniciar o Clima: {error}");
        std::process::exit(1);
    }
}
