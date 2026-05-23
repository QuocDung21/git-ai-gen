use super::AppTheme;
use ratatui::style::Color;

pub fn palette() -> AppTheme {
    AppTheme {
        fg: Color::Rgb(101, 123, 131),        // Base00
        border: Color::Rgb(147, 161, 161),    // Base1
        purple: Color::Rgb(108, 113, 196),    // Violet
        green: Color::Rgb(133, 153, 0),       // Green
        red: Color::Rgb(220, 50, 47),         // Red
        yellow: Color::Rgb(181, 137, 0),      // Yellow
        cyan: Color::Rgb(42, 161, 152),       // Cyan
        orange: Color::Rgb(203, 75, 22),      // Orange
        select_bg: Color::Rgb(238, 232, 213), // Base2 (Highlight)
        select_fg: Color::Rgb(88, 110, 117),  // Base01
        bg: Color::Rgb(253, 246, 227),        // Base3
    }
}
