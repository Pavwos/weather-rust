pub fn describe(code: u8, is_day: bool) -> (&'static str, &'static str) {
    match code {
        0 if is_day => ("Céu limpo", "clear-day"),
        0 => ("Céu limpo", "clear-night"),
        1 if is_day => ("Predominantemente limpo", "partly-cloudy-day"),
        1 => ("Predominantemente limpo", "partly-cloudy-night"),
        2 if is_day => ("Parcialmente nublado", "partly-cloudy-day"),
        2 => ("Parcialmente nublado", "partly-cloudy-night"),
        3 => ("Nublado", "cloudy"),
        45 => ("Nevoeiro", "fog"),
        48 => ("Nevoeiro com geada", "fog"),
        51 => ("Garoa fraca", "drizzle"),
        53 => ("Garoa moderada", "drizzle"),
        55 => ("Garoa intensa", "drizzle"),
        56 => ("Garoa congelante fraca", "sleet"),
        57 => ("Garoa congelante intensa", "sleet"),
        61 => ("Chuva fraca", "rain"),
        63 => ("Chuva moderada", "rain"),
        65 => ("Chuva intensa", "heavy-rain"),
        66 => ("Chuva congelante fraca", "sleet"),
        67 => ("Chuva congelante intensa", "sleet"),
        71 => ("Neve fraca", "snow"),
        73 => ("Neve moderada", "snow"),
        75 => ("Neve intensa", "snow"),
        77 => ("Grãos de neve", "sleet"),
        80 => ("Pancadas de chuva fracas", "drizzle"),
        81 => ("Pancadas de chuva moderadas", "rain"),
        82 => ("Pancadas de chuva violentas", "heavy-rain"),
        85 => ("Pancadas de neve fracas", "snow"),
        86 => ("Pancadas de neve intensas", "snow"),
        95 => ("Trovoada", "thunderstorm"),
        96 => ("Trovoada com granizo fraco", "thunderstorm"),
        99 => ("Trovoada com granizo forte", "thunderstorm"),
        _ => ("Condição desconhecida", "cloudy"),
    }
}

#[cfg(test)]
mod tests {
    use super::describe;

    #[test]
    fn describes_wmo_codes_and_distinguishes_clear_day_and_night() {
        assert_eq!(describe(0, true), ("Céu limpo", "clear-day"));
        assert_eq!(describe(0, false), ("Céu limpo", "clear-night"));
        assert_eq!(
            describe(1, false),
            ("Predominantemente limpo", "partly-cloudy-night")
        );
        assert_eq!(describe(61, true), ("Chuva fraca", "rain"));
        assert_eq!(describe(65, true), ("Chuva intensa", "heavy-rain"));
        assert_eq!(
            describe(99, true),
            ("Trovoada com granizo forte", "thunderstorm")
        );
        assert_eq!(describe(255, true), ("Condição desconhecida", "cloudy"));
    }

    #[test]
    fn maps_every_supported_wmo_code_to_a_weather_icon() {
        let cases = [
            (0, "clear-day"),
            (1, "partly-cloudy-day"),
            (2, "partly-cloudy-day"),
            (3, "cloudy"),
            (45, "fog"),
            (48, "fog"),
            (51, "drizzle"),
            (53, "drizzle"),
            (55, "drizzle"),
            (56, "sleet"),
            (57, "sleet"),
            (61, "rain"),
            (63, "rain"),
            (65, "heavy-rain"),
            (66, "sleet"),
            (67, "sleet"),
            (71, "snow"),
            (73, "snow"),
            (75, "snow"),
            (77, "sleet"),
            (80, "drizzle"),
            (81, "rain"),
            (82, "heavy-rain"),
            (85, "snow"),
            (86, "snow"),
            (95, "thunderstorm"),
            (96, "thunderstorm"),
            (99, "thunderstorm"),
        ];

        for (code, expected_icon) in cases {
            assert_eq!(describe(code, true).1, expected_icon, "código WMO {code}");
        }
    }
}
