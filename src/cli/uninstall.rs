use crate::cli::clean_profile_file;
use crate::cli::logger;
use anyhow::Result;

#[cfg(target_family = "unix")]
use crate::cli::get_active_unix_profile;
#[cfg(target_os = "windows")]
use crate::cli::get_windows_profile;

pub fn handle_uninstall() -> Result<()> {
    logger::warn("🗑️  Uninstalling configuration from system...");
    #[cfg(target_family = "unix")]
    {
        let profile = get_active_unix_profile();
        if clean_profile_file(&profile)? {
            logger::success(&format!("Successfully removed from: {}", profile.display()));
            logger::note(&format!(
                "👉 Please restart Terminal or run 'source {}' to apply.",
                profile.display()
            ));
        } else {
            logger::info("No git-ai configuration found to remove.");
        }
    }

    #[cfg(target_os = "windows")]
    {
        let profile = get_windows_profile()?;
        if clean_profile_file(&profile)? {
            logger::success("Removed functions from PowerShell Profile!");
            logger::note("👉 Please restart PowerShell to apply changes.");
        } else {
            logger::info("No PowerShell Profile configuration found to remove.");
        }
    }

    let _ = std::process::Command::new("git")
        .args(["config", "--global", "--remove-section", "git-ai"])
        .output();
    logger::success("Cleaned up all git-ai configurations in Git Config.");

    let chill_dir = crate::helper::Helper::get_git_chill_dir();
    if chill_dir.exists() {
        let _ = std::fs::remove_dir_all(&chill_dir);
        logger::success("Cleaned up ~/.git-chill history directory.");
    }

    Ok(())
}
