use super::{AppTheme, Color};

pub fn palette() -> AppTheme {
    AppTheme {
        fg: Color::Rgb(40, 42, 54),
        border: Color::Rgb(140, 140, 140),
        purple: Color::Rgb(109, 40, 217),
        green: Color::Rgb(21, 128, 61),
        red: Color::Rgb(185, 28, 28),
        yellow: Color::Rgb(161, 98, 7),
        cyan: Color::Rgb(3, 105, 161),
        orange: Color::Rgb(194, 65, 12),
        select_bg: Color::Rgb(220, 224, 232),
        select_fg: Color::Rgb(17, 24, 39),
        bg: Color::Rgb(248, 249, 250),
    }
}
