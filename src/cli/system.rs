use crate::cli::logger;
use crate::cli::{ask_confirm, ask_confirm_default_no, uninstall, Locales};
use crate::helper::Helper;
use anyhow::Result;
use arboard::Clipboard;
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

pub fn handle_restore(locales: &Locales) -> Result<()> {
    logger::heading(&locales.reset_heading);
    let status = Command::new("git")
        .args(["config", "--global", "--remove-section", "git-ai"])
        .output();

    match status {
        Ok(out) if out.status.success() => {
            logger::success(&locales.reset_success);
            logger::info(&locales.reset_info);
        }
        _ => {
            logger::info(&locales.reset_clean);
        }
    }

    if ask_confirm_default_no(&locales.confirm_remove_alias)? {
        uninstall::handle_uninstall()?;
    } else {
        logger::success(&locales.keep_alias);
    }
    Ok(())
}

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

pub fn handle_check_status(locales: &Locales) -> Result<bool> {
    let output = Command::new("git").args(["status", "-s"]).output()?;
    let status_text = String::from_utf8_lossy(&output.stdout);
    if status_text.trim().is_empty() {
        logger::info(&locales.status_clean);
        return Ok(false);
    }
    logger::info(&locales.status_pending);
    logger::text(&status_text);
    Ok(true)
}

pub fn handle_go(locales: &Locales) -> Result<()> {
    logger::heading(&locales.preview_heading);

    if !handle_check_status(locales)? {
        return Ok(());
    }

    let mut clipboard = Clipboard::new()?;
    let commit_msg = clipboard.get_text().unwrap_or_default();

    logger::system(&locales.commit_content);
    logger::green_text(&commit_msg);
    logger::text("");

    if ask_confirm(&locales.confirm_deploy)? {
        logger::heading(&locales.pushing);

        if !Command::new("git")
            .args(["add", "-A"])
            .output()?
            .status
            .success()
        {
            return Ok(());
        }
        if !Command::new("git")
            .args(["commit", "-m", &commit_msg])
            .output()?
            .status
            .success()
        {
            return Ok(());
        }

        if Command::new("git")
            .args(["push"])
            .output()?
            .status
            .success()
        {
            logger::success(&locales.push_success);
        }
    } else {
        logger::error(&locales.deploy_cancel);
    }
    Ok(())
}

pub fn handle_test() -> anyhow::Result<()> {
    Ok(())
}
