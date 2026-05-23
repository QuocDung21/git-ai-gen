use super::AppTheme;
use ratatui::style::Color;

pub fn palette() -> AppTheme {
    AppTheme {
        fg: Color::Rgb(76, 79, 105),          // Text / Text
        border: Color::Rgb(188, 192, 204),    // Surface1
        purple: Color::Rgb(136, 57, 239),     // Mauve
        green: Color::Rgb(64, 160, 43),       // Green
        red: Color::Rgb(210, 15, 57),         // Red
        yellow: Color::Rgb(223, 142, 29),     // Yellow
        cyan: Color::Rgb(4, 165, 229),        // Sky
        orange: Color::Rgb(254, 100, 11),     // Peach
        select_bg: Color::Rgb(172, 176, 190), // Surface2 (Highlight)
        select_fg: Color::Rgb(76, 79, 105),   // Text
        bg: Color::Rgb(239, 241, 245),        // Base
    }
}
