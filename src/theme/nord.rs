use super::{AppTheme, Color};

pub fn palette() -> AppTheme {
    AppTheme {
        fg: Color::Rgb(236, 239, 244),
        border: Color::Rgb(129, 161, 193),
        purple: Color::Rgb(180, 142, 173),
        green: Color::Rgb(163, 190, 140),
        red: Color::Rgb(191, 97, 106),
        yellow: Color::Rgb(235, 203, 139),
        cyan: Color::Rgb(143, 188, 187),
        orange: Color::Rgb(208, 135, 112),
        select_bg: Color::Rgb(67, 76, 94),
        select_fg: Color::Rgb(236, 239, 244),
        bg: Color::Rgb(46, 52, 64),
    }
}
