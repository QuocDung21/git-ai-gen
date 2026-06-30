use crate::cli::logger;
use crate::cli::{ask_confirm, Locales};
use anyhow::Result;
use arboard::Clipboard;
use std::process::Command;

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

fn handle_check_status(locales: &Locales) -> Result<bool> {
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
