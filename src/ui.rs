use crate::{
    app::{LoadState, WeatherData},
    theme, weather_code, Units,
};
use chrono::{Datelike, NaiveDate};
use eframe::egui::{self, Align, Color32, Layout, RichText, ScrollArea};

pub enum UiAction {
    Search,
    Refresh,
    SetUnits(Units),
    SaveDefaultCity,
}

pub fn draw(
    ui: &mut egui::Ui,
    city_input: &mut String,
    units: Units,
    state: &LoadState,
) -> Vec<UiAction> {
    let mut actions = Vec::new();
    draw_search_bar(ui, city_input, units, state, &mut actions);
    ui.separator();

    ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| match state {
            LoadState::Idle => draw_welcome(ui),
            LoadState::Loading => draw_loading(ui),
            LoadState::Error(message) => {
                draw_error(ui, message);
                draw_welcome(ui);
            }
            LoadState::Loaded(data) => draw_weather(ui, data, units, &mut actions),
        });
    actions
}

fn draw_search_bar(
    ui: &mut egui::Ui,
    city_input: &mut String,
    units: Units,
    state: &LoadState,
    actions: &mut Vec<UiAction>,
) {
    ui.horizontal(|ui| {
        let response = ui.add(
            egui::TextEdit::singleline(city_input)
                .hint_text("Digite uma cidade")
                .desired_width(155.0),
        );
        let enter_pressed =
            response.lost_focus() && ui.input(|input| input.key_pressed(egui::Key::Enter));
        if ui.button("Buscar").clicked() || enter_pressed {
            actions.push(UiAction::Search);
        }
        if matches!(state, LoadState::Loading) {
            ui.add(egui::Spinner::new());
        }

        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            if ui
                .selectable_label(units == Units::Imperial, "°F")
                .clicked()
            {
                actions.push(UiAction::SetUnits(Units::Imperial));
            }
            if ui.selectable_label(units == Units::Metric, "°C").clicked() {
                actions.push(UiAction::SetUnits(Units::Metric));
            }
        });
    });
}

fn draw_welcome(ui: &mut egui::Ui) {
    ui.add_space(28.0);
    ui.vertical_centered(|ui| {
        ui.label(RichText::new("🌤️").size(52.0));
        ui.add_space(8.0);
        ui.label(RichText::new("Bem-vindo ao Clima").size(23.0).strong());
        ui.add_space(8.0);
        ui.label("Digite uma cidade acima para ver a previsão.");
    });
}

fn draw_loading(ui: &mut egui::Ui) {
    ui.add_space(24.0);
    ui.vertical_centered(|ui| {
        ui.add(egui::Spinner::new().size(32.0));
        ui.add_space(8.0);
        ui.label("Buscando previsão...");
    });
}

fn draw_error(ui: &mut egui::Ui, message: &str) {
    egui::Frame::new()
        .fill(Color32::from_rgb(83, 43, 52))
        .corner_radius(egui::CornerRadius::same(10))
        .inner_margin(egui::Margin::same(14))
        .show(ui, |ui| {
            ui.label(RichText::new("Não foi possível carregar o clima").strong());
            ui.add_space(4.0);
            ui.label(message);
        });
    ui.add_space(12.0);
}

fn draw_weather(ui: &mut egui::Ui, data: &WeatherData, units: Units, actions: &mut Vec<UiAction>) {
    ui.horizontal(|ui| {
        ui.vertical(|ui| {
            ui.label(RichText::new(location_label(data)).size(18.0).strong());
            ui.label(
                RichText::new(format!("Atualizado às {}", data.updated_at.format("%H:%M")))
                    .size(12.0)
                    .weak(),
            );
        });
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            if ui.button("⭐ Salvar como padrão").clicked() {
                actions.push(UiAction::SaveDefaultCity);
            }
            if ui.button("🔄").on_hover_text("Atualizar agora").clicked() {
                actions.push(UiAction::Refresh);
            }
        });
    });
    ui.add_space(12.0);

    draw_current_card(ui, data, units);
    ui.add_space(16.0);
    ui.label(RichText::new("Próximos dias").size(17.0).strong());
    ui.add_space(6.0);
    for day in &data.days {
        draw_day_card(ui, day, units);
        ui.add_space(7.0);
    }
}

fn draw_current_card(ui: &mut egui::Ui, data: &WeatherData, units: Units) {
    let (description, emoji) =
        weather_code::describe(data.current.weather_code, data.current.is_day != 0);
    let unit = if units.is_imperial() { "°F" } else { "°C" };
    let speed_unit = if units.is_imperial() { "mph" } else { "km/h" };

    theme::card_frame(theme::weather_card_color(
        data.current.weather_code,
        data.current.is_day != 0,
    ))
    .show(ui, |ui| {
        ui.set_min_width(ui.available_width());
        ui.horizontal(|ui| {
            ui.label(RichText::new(emoji).size(54.0));
            ui.vertical(|ui| {
                ui.label(
                    RichText::new(format!(
                        "{}{}",
                        format_temperature(data.current.temperature_2m),
                        unit
                    ))
                    .size(48.0)
                    .strong()
                    .color(theme::temperature_color(
                        data.current.temperature_2m,
                        units.is_imperial(),
                    )),
                );
                ui.label(RichText::new(description).size(16.0));
            });
        });

        ui.add_space(14.0);
        ui.horizontal_wrapped(|ui| {
            ui.label(format!(
                "🌡️ Sensação {}{}",
                format_temperature(data.current.apparent_temperature),
                unit
            ));
            ui.label(format!("💧 {}%", data.current.relative_humidity_2m));
            ui.label(format!(
                "💨 {} {}",
                format_temperature(data.current.wind_speed_10m),
                speed_unit
            ));
        });
        ui.add_space(10.0);
        ui.horizontal(|ui| {
            ui.label(
                RichText::new(format!(
                    "🌅 {}",
                    time_only(data.days.first().map(|day| &day.sunrise))
                ))
                .size(13.0)
                .weak(),
            );
            ui.label(
                RichText::new(format!(
                    "🌇 {}",
                    time_only(data.days.first().map(|day| &day.sunset))
                ))
                .size(13.0)
                .weak(),
            );
        });
    });
}

fn draw_day_card(ui: &mut egui::Ui, day: &crate::models::DayForecast, units: Units) {
    let (description, emoji) = weather_code::describe(day.weather_code, true);
    let (weekday, date) = date_parts(&day.date);
    let unit = if units.is_imperial() { "°F" } else { "°C" };
    egui::Frame::new()
        .fill(Color32::from_rgb(29, 35, 49))
        .corner_radius(egui::CornerRadius::same(9))
        .inner_margin(egui::Margin::symmetric(10, 9))
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            ui.horizontal(|ui| {
                ui.add_sized(
                    [50.0, 20.0],
                    egui::Label::new(RichText::new(format!("{weekday} {date}")).size(12.0)),
                );
                ui.add_sized(
                    [24.0, 24.0],
                    egui::Label::new(RichText::new(emoji).size(19.0)),
                );
                ui.add_sized(
                    [76.0, 30.0],
                    egui::Label::new(RichText::new(description).size(11.0))
                        .wrap_mode(egui::TextWrapMode::Truncate),
                );
                ui.label(
                    RichText::new(format!(
                        "{}{}",
                        format_temperature(day.temperature_min),
                        unit
                    ))
                    .size(11.0)
                    .color(theme::temperature_color(
                        day.temperature_min,
                        units.is_imperial(),
                    )),
                );
                ui.label("/");
                ui.label(
                    RichText::new(format!(
                        "{}{}",
                        format_temperature(day.temperature_max),
                        unit
                    ))
                    .size(11.0)
                    .color(theme::temperature_color(
                        day.temperature_max,
                        units.is_imperial(),
                    )),
                );
                ui.label(format!("☔ {}%", day.precipitation_probability));
            });
        });
}

fn location_label(data: &WeatherData) -> String {
    let mut parts = vec![data.location.name.as_str()];
    if let Some(region) = data.location.admin1.as_deref() {
        parts.push(region);
    }
    if let Some(country) = data.location.country.as_deref() {
        parts.push(country);
    }
    parts.join(", ")
}

fn date_parts(value: &str) -> (String, String) {
    let Ok(date) = NaiveDate::parse_from_str(value, "%Y-%m-%d") else {
        return ("".to_owned(), value.to_owned());
    };
    let weekday = match date.weekday().num_days_from_monday() {
        0 => "Seg",
        1 => "Ter",
        2 => "Qua",
        3 => "Qui",
        4 => "Sex",
        5 => "Sáb",
        _ => "Dom",
    };
    (weekday.to_owned(), date.format("%d/%m").to_string())
}

fn format_temperature(value: f64) -> String {
    if value.fract() == 0.0 {
        format!("{value:.0}")
    } else {
        format!("{value:.1}")
    }
}

fn time_only(value: Option<&String>) -> String {
    value
        .and_then(|value| value.split_once('T').map(|(_, time)| time))
        .and_then(|time| time.get(..5))
        .unwrap_or("--:--")
        .to_owned()
}
