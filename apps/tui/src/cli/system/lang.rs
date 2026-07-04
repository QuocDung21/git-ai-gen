use crate::cli::Locales;
use crate::helper::Helper;
use anyhow::Result;
use std::process::Command;

pub fn handle_lang(lang: &str, locales: &Locales) -> Result<String> {
    match lang {
        "vi" | "en" => {
            Command::new("git")
                .args(["config", "--global", "git-ai.lang", lang])
                .output()?;
            let new_locales = Locales::new(lang);
            Ok(format!("{} {}", new_locales.lang_set, lang))
        }
        "auto" => {
            let _ = Command::new("git")
                .args(["config", "--global", "--unset", "git-ai.lang"])
                .output();
            let resolved_lang = Helper::get_ai_language();
            let new_locales = Locales::new(&resolved_lang);
            Ok(new_locales.lang_auto)
        }
        _ => Ok(locales.lang_invalid.clone()),
    }
}
