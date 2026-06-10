#[cfg(feature = "tui")]
pub type Color = ratatui::style::Color;

#[cfg(not(feature = "tui"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    Rgb(u8, u8, u8),
}

mod catppuccin_latte;
mod dark;
mod gruvbox;
mod light;
mod nord;
mod solarized_light;
mod vscode;

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
            id: "dark",
            name_en: "Dracula (Dark) 🌌",
            name_vi: "Dracula (Tối / Dark) 🌌",
            shortcut: "[d]",
            hotkey: 'd',
        },
        ThemeInfo {
            id: "solarized_light",
            name_en: "Solarized Light (Sáng / Light) ☀️",
            name_vi: "Solarized Light (Sáng / Light) ☀️",
            shortcut: "[s]",
            hotkey: 's',
        },
        ThemeInfo {
            id: "catppuccin_latte",
            name_en: "Catppuccin Latte (Light) 🍂",
            name_vi: "Catppuccin Latte (Sáng / Light) 🍂",
            shortcut: "[c]",
            hotkey: 'c',
        },
        ThemeInfo {
            id: "light",
            name_en: "Premium Light ☀️",
            name_vi: "Premium Light (Sáng / Light) ☀️",
            shortcut: "[l]",
            hotkey: 'l',
        },
        ThemeInfo {
            id: "nord",
            name_en: "Nord (Arctic Ice) ❄️",
            name_vi: "Nord (Băng Tuyết / Arctic Ice) ❄️",
            shortcut: "[n]",
            hotkey: 'n',
        },
        ThemeInfo {
            id: "gruvbox",
            name_en: "Gruvbox (Retro Warm) 🍂",
            name_vi: "Gruvbox (Cổ Điển / Retro Warm) 🍂",
            shortcut: "[g]",
            hotkey: 'g',
        },
        ThemeInfo {
            id: "vscode",
            name_en: "VS Code (Dark Modern) 💻",
            name_vi: "VS Code (Tối Hiện Đại) 💻",
            shortcut: "[v]",
            hotkey: 'v',
        },
    ]
}

pub fn get_theme(theme_id: &str) -> AppTheme {
    match theme_id {
        "light" => light::palette(),
        "nord" => nord::palette(),
        "gruvbox" => gruvbox::palette(),
        "catppuccin_latte" => catppuccin_latte::palette(),
        "solarized_light" => solarized_light::palette(),
        "vscode" => vscode::palette(),
        _ => dark::palette(),
    }
}
