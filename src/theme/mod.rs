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

mod native;

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

#[derive(Clone, Debug, PartialEq)]
pub struct ThemeInfo {
    pub id: &'static str,
    pub name_en: &'static str,
    pub name_vi: &'static str,
    pub shortcut: &'static str,
    pub hotkey: char,
}

pub fn get_all_themes() -> Vec<ThemeInfo> {
    vec![
        ThemeInfo {
            id: "native",
            name_en: "Native Terminal",
            name_vi: "Màu terminal mặc định",
            shortcut: "[n]",
            hotkey: 'n',
        },
    ]
}

pub fn get_theme(_theme_id: &str) -> AppTheme {
    native::palette()
}
