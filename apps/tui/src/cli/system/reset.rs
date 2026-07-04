use crate::cli::logger;
use crate::cli::{ask_confirm_default_no, uninstall, Locales};
use anyhow::Result;
use std::process::Command;

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
