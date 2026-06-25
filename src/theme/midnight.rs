use super::{AppTheme, Color};

pub fn palette() -> AppTheme {
    AppTheme {
        fg: Color::Rgb(230, 237, 247),
        border: Color::Rgb(107, 122, 153),
        purple: Color::Rgb(185, 163, 255),
        green: Color::Rgb(87, 217, 155),
        red: Color::Rgb(255, 107, 129),
        yellow: Color::Rgb(243, 201, 105),
        cyan: Color::Rgb(98, 214, 255),
        orange: Color::Rgb(255, 155, 113),
        select_bg: Color::Rgb(36, 71, 102),
        select_fg: Color::Rgb(248, 251, 255),
        bg: Color::Rgb(11, 17, 32),
    }
}
