use super::{AppTheme, Color};

pub fn palette() -> AppTheme {
    AppTheme {
        fg: Color::Reset,
        border: Color::Blue,
        purple: Color::Magenta,
        green: Color::Green,
        red: Color::Red,
        yellow: Color::Yellow,
        cyan: Color::Cyan,
        orange: Color::LightRed,
        select_bg: Color::Blue,
        select_fg: Color::White,
        bg: Color::Reset,
    }
}
