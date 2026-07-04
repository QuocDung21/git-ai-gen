#[cfg(feature = "tui")]
pub type Color = ratatui::style::Color;

#[cfg(not(feature = "tui"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    Reset,
    White,
    Blue,
    Magenta,
    Green,
    Red,
    Yellow,
    Cyan,
    LightRed,
    Rgb(u8, u8, u8),
}

mod midnight;

#[derive(Clone, Debug, PartialEq)]
pub struct AppTheme {
    pub fg: Color,
    pub border: Color,
    pub purple: Color,
    pub green: Color,
    pub red: Color,
    pub yellow: Color,
    pub cyan: Color,
    pub orange: Color,
    pub select_bg: Color,
    pub select_fg: Color,
    pub bg: Color,
}

impl AppTheme {
    pub fn shadow(&self) -> Color {
        match self.bg {
            Color::Rgb(red, green, blue) => Color::Rgb(red / 2, green / 2, blue / 2),
            _ => self.bg,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ThemeInfo {
    pub id: &'static str,
    pub name_en: &'static str,
    pub name_vi: &'static str,
    pub shortcut: &'static str,
    pub hotkey: char,
}

pub fn get_all_themes() -> Vec<ThemeInfo> {
    vec![ThemeInfo {
        id: "midnight",
        name_en: "Git-AI Midnight",
        name_vi: "Git-AI Midnight",
        shortcut: "[m]",
        hotkey: 'm',
    }]
}

pub fn get_theme(_theme_id: &str) -> AppTheme {
    midnight::palette()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exposes_midnight_as_the_application_theme() {
        let themes = get_all_themes();

        assert_eq!(themes.len(), 1);
        assert_eq!(themes[0].id, "midnight");
        assert_eq!(get_theme("midnight"), get_theme("native"));
    }

    #[test]
    fn midnight_palette_uses_the_expected_color_roles() {
        let theme = get_theme("midnight");

        assert_eq!(theme.bg, Color::Rgb(11, 17, 32));
        assert_eq!(theme.fg, Color::Rgb(230, 237, 247));
        assert_eq!(theme.border, Color::Rgb(107, 122, 153));
        assert_eq!(theme.select_bg, Color::Rgb(36, 71, 102));
        assert_eq!(theme.select_fg, Color::Rgb(248, 251, 255));
        assert_eq!(theme.shadow(), Color::Rgb(5, 8, 16));
    }
}
