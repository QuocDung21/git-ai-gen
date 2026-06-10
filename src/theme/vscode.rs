use super::{AppTheme, Color};

pub fn palette() -> AppTheme {
    AppTheme {
        fg: Color::Rgb(212, 212, 212),
        border: Color::Rgb(0, 122, 204),
        purple: Color::Rgb(197, 134, 192),
        green: Color::Rgb(78, 201, 176),
        red: Color::Rgb(244, 71, 71),
        yellow: Color::Rgb(220, 220, 170),
        cyan: Color::Rgb(79, 193, 255),
        orange: Color::Rgb(206, 145, 120),
        select_bg: Color::Rgb(38, 79, 120),
        select_fg: Color::Rgb(255, 255, 255),
        bg: Color::Rgb(30, 30, 30),
    }
}
