use super::AppTheme;
use ratatui::style::Color;

pub fn palette() -> AppTheme {
    AppTheme {
        fg: Color::Rgb(248, 248, 242),
        border: Color::Rgb(98, 114, 164),
        purple: Color::Rgb(189, 147, 249),
        green: Color::Rgb(80, 250, 123),
        red: Color::Rgb(255, 85, 85),
        yellow: Color::Rgb(241, 250, 140),
        cyan: Color::Rgb(139, 233, 253),
        orange: Color::Rgb(255, 184, 108),
        select_bg: Color::Rgb(68, 71, 90),
        select_fg: Color::Rgb(248, 248, 242),
        bg: Color::Rgb(40, 42, 54),
    }
}
