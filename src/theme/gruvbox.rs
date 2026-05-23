use super::AppTheme;
use ratatui::style::Color;

pub fn palette() -> AppTheme {
    AppTheme {
        fg: Color::Rgb(235, 219, 178),
        border: Color::Rgb(146, 131, 116),
        purple: Color::Rgb(211, 134, 155),
        green: Color::Rgb(184, 187, 38),
        red: Color::Rgb(251, 73, 52),
        yellow: Color::Rgb(250, 189, 47),
        cyan: Color::Rgb(142, 192, 124),
        orange: Color::Rgb(254, 128, 25),
        select_bg: Color::Rgb(80, 73, 69),
        select_fg: Color::Rgb(250, 245, 225),
        bg: Color::Rgb(40, 40, 40),
    }
}
