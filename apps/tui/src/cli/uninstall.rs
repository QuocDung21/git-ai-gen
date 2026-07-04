use crate::cli::clean_profile_file;
use crate::cli::logger;
use anyhow::Result;
use rust_i18n::t;

#[cfg(target_family = "unix")]
use crate::cli::get_active_unix_profile;
#[cfg(target_os = "windows")]
use crate::cli::get_windows_profile;

pub fn handle_uninstall() -> Result<()> {
    logger::warn(t!("uninstall_start").as_ref());
    #[cfg(target_family = "unix")]
    {
        let profile = get_active_unix_profile();
        if clean_profile_file(&profile)? {
            logger::success(
                t!(
                    "uninstall_removed_from",
                    path = profile.display().to_string()
                )
                .as_ref(),
            );
            logger::note(
                t!(
                    "uninstall_restart_terminal",
                    path = profile.display().to_string()
                )
                .as_ref(),
            );
        } else {
            logger::info(t!("uninstall_no_config").as_ref());
        }
    }

    #[cfg(target_os = "windows")]
    {
        let profile = get_windows_profile()?;
        if clean_profile_file(&profile)? {
            logger::success(t!("uninstall_removed_powershell").as_ref());
            logger::note(t!("uninstall_restart_powershell").as_ref());
        } else {
            logger::info(t!("uninstall_no_powershell_config").as_ref());
        }
    }

    let _ = std::process::Command::new("git")
        .args(["config", "--global", "--remove-section", "git-ai"])
        .output();
    logger::success(t!("uninstall_git_config_cleaned").as_ref());

    let chill_dir = crate::helper::Helper::get_git_chill_dir();
    if chill_dir.exists() {
        let _ = std::fs::remove_dir_all(&chill_dir);
        logger::success(t!("uninstall_history_cleaned").as_ref());
    }

    Ok(())
}
