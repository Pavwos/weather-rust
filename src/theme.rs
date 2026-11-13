use eframe::egui::{self, Color32, CornerRadius, Margin, Stroke, Vec2, Visuals};

#[cfg(clima_emoji_font)]
use eframe::egui::FontFamily;

pub fn apply(context: &egui::Context) {
    let mut visuals = Visuals::dark();
    visuals.window_corner_radius = CornerRadius::same(12);
    visuals.widgets.noninteractive.corner_radius = CornerRadius::same(8);
    visuals.widgets.inactive.corner_radius = CornerRadius::same(8);
    visuals.widgets.hovered.corner_radius = CornerRadius::same(8);
    visuals.widgets.active.corner_radius = CornerRadius::same(8);
    visuals.panel_fill = Color32::from_rgb(18, 22, 32);
    visuals.window_fill = Color32::from_rgb(24, 29, 42);
    context.set_visuals(visuals);

    let mut style = (*context.style()).clone();
    style.spacing.item_spacing = Vec2::new(10.0, 10.0);
    style.spacing.button_padding = Vec2::new(12.0, 8.0);
    context.set_style(style);

    #[cfg(clima_emoji_font)]
    {
        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert(
            "noto_emoji".to_owned(),
            std::sync::Arc::new(egui::FontData::from_static(include_bytes!(
                "../assets/NotoEmoji-Regular.ttf"
            ))),
        );
        fonts
            .families
            .entry(FontFamily::Proportional)
            .or_default()
            .push("noto_emoji".to_owned());
        context.set_fonts(fonts);
    }
}

pub fn temperature_color(temperature: f64, units_imperial: bool) -> Color32 {
    let (cold_limit, mild_limit, warm_limit) = if units_imperial {
        (59.0, 77.0, 89.6)
    } else {
        (15.0, 25.0, 32.0)
    };

    if temperature < cold_limit {
        Color32::from_rgb(95, 180, 255)
    } else if temperature <= mild_limit {
        Color32::from_rgb(105, 220, 160)
    } else if temperature <= warm_limit {
        Color32::from_rgb(255, 205, 95)
    } else {
        Color32::from_rgb(255, 115, 105)
    }
}

pub fn weather_card_color(weather_code: u8, is_day: bool) -> Color32 {
    match weather_code {
        0 | 1 if is_day => Color32::from_rgb(41, 103, 158),
        0 | 1 => Color32::from_rgb(38, 48, 86),
        2..=3 | 45 | 48 => Color32::from_rgb(74, 84, 105),
        95..=99 => Color32::from_rgb(71, 56, 103),
        _ => Color32::from_rgb(57, 78, 105),
    }
}

pub fn card_frame(color: Color32) -> egui::Frame {
    egui::Frame::new()
        .fill(color)
        .corner_radius(CornerRadius::same(12))
        .inner_margin(Margin::same(16))
        .stroke(Stroke::NONE)
}

#[cfg(test)]
mod tests {
    use super::temperature_color;
    use eframe::egui::Color32;

    #[test]
    fn selects_temperature_colors_for_metric_and_imperial() {
        assert_eq!(
            temperature_color(14.9, false),
            Color32::from_rgb(95, 180, 255)
        );
        assert_eq!(
            temperature_color(15.0, false),
            Color32::from_rgb(105, 220, 160)
        );
        assert_eq!(
            temperature_color(25.0, false),
            Color32::from_rgb(105, 220, 160)
        );
        assert_eq!(
            temperature_color(26.0, false),
            Color32::from_rgb(255, 205, 95)
        );
        assert_eq!(
            temperature_color(33.0, false),
            Color32::from_rgb(255, 115, 105)
        );
        assert_eq!(
            temperature_color(58.9, true),
            Color32::from_rgb(95, 180, 255)
        );
        assert_eq!(
            temperature_color(59.0, true),
            Color32::from_rgb(105, 220, 160)
        );
        assert_eq!(
            temperature_color(78.0, true),
            Color32::from_rgb(255, 205, 95)
        );
        assert_eq!(
            temperature_color(90.0, true),
            Color32::from_rgb(255, 115, 105)
        );
    }
}
