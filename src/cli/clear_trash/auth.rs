use anyhow::{bail, Context, Result};
use rust_i18n::t;

use crate::cli::{ask_confirm_default_yes, logger};

pub(super) fn require_system_authentication() -> Result<()> {
    logger::heading(&t!("clear_trash_auth_heading").to_string());

    #[cfg(target_family = "unix")]
    {
        use std::process::Command;

        let auth_prompt = auth_prompt();
        let _ = Command::new("sudo").arg("-k").status();

        let status = Command::new("sudo")
            .arg("-p")
            .arg(format!("{} ", auth_prompt))
            .arg("-v")
            .status()
            .context(t!("clear_trash_auth_failed").to_string())?;

        if !status.success() {
            bail!("{}", t!("clear_trash_auth_failed"));
        }

        logger::success(&t!("clear_trash_auth_success").to_string());
    }

    #[cfg(not(target_family = "unix"))]
    {
        logger::success(&t!("clear_trash_auth_success").to_string());
    }

    Ok(())
}

#[cfg(target_os = "macos")]
fn auth_prompt() -> String {
    if !sudo_touch_id_configured() {
        logger::warn(&t!("clear_trash_touch_id_not_configured").to_string());
        logger::info(&t!("clear_trash_password_hint").to_string());
        return t!("clear_trash_auth_prompt").to_string();
    }

    if ask_confirm_default_yes(&t!("clear_trash_touch_id_confirm").to_string()).unwrap_or(false) {
        logger::info(&t!("clear_trash_touch_id_hint").to_string());
        t!("clear_trash_auth_prompt_touch_id").to_string()
    } else {
        logger::info(&t!("clear_trash_password_hint").to_string());
        t!("clear_trash_auth_prompt").to_string()
    }
}

#[cfg(target_os = "macos")]
fn sudo_touch_id_configured() -> bool {
    ["/etc/pam.d/sudo_local", "/etc/pam.d/sudo"]
        .iter()
        .filter_map(|path| std::fs::read_to_string(path).ok())
        .flat_map(|content| {
            content
                .lines()
                .map(str::trim)
                .map(str::to_string)
                .collect::<Vec<_>>()
        })
        .any(|line| !line.starts_with('#') && line.contains("pam_tid.so"))
}

#[cfg(not(target_os = "macos"))]
fn auth_prompt() -> String {
    t!("clear_trash_auth_prompt").to_string()
}
