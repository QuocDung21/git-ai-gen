use crate::cli::Locales;
use crate::helper::Helper;
use anyhow::Result;
use arboard::Clipboard;
use std::process::Command;

pub fn handle_diff(locales: &Locales) -> Result<String> {
    let output = Command::new("git").args(["diff"]).output()?;
    let diff_str = String::from_utf8_lossy(&output.stdout);

    if diff_str.trim().is_empty() {
        return Ok(locales.no_changes.clone());
    }

    let ai_lang = Helper::get_ai_language_name();

    let prompt = format!(
        "{} {}.\n\nDiff:\n\n{}",
        crate::constant::Constant::PROMPT_EXPERT,
        ai_lang,
        diff_str
    );

    let mut clipboard = Clipboard::new()?;
    clipboard.set_text(prompt)?;

    Ok(locales.diff_success.clone())
}
