use crate::cli::Locales;
use std::process::Command;

pub struct Helper;

impl Helper {
    pub fn get_os_theme() -> dark_light::Mode {
        match dark_light::detect() {
            Ok(mode) => mode,
            Err(_) => dark_light::Mode::Unspecified,
        }
    }

    pub fn get_ai_language() -> String {
        if let Ok(output) = Command::new("git")
            .args(["config", "--global", "--get", "git-ai.lang"])
            .output()
        {
            let stdout = String::from_utf8_lossy(&output.stdout)
                .trim()
                .to_lowercase();
            if stdout == "vi" || stdout == "en" {
                return stdout;
            }
        }

        let locale = sys_locale::get_locale().unwrap_or_else(|| String::from("en-US"));
        if locale.starts_with("vi") {
            "vi".to_string()
        } else {
            "en".to_string()
        }
    }

    pub fn get_locales() -> Locales {
        let lang = Helper::get_ai_language();
        Locales::new(&lang)
    }
}
