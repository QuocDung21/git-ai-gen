use crate::cli::logger;
use crate::cli::spinner::with_spinner;
use crate::cli::{
    append_to_file, ask_confirm_default_no, clean_profile_file, print_commands_help, Locales,
};
use anyhow::Result;
use std::env;

#[cfg(target_family = "unix")]
use crate::cli::get_active_unix_profile;
#[cfg(target_os = "windows")]
use crate::cli::get_windows_profile;

pub fn handle_install() -> Result<()> {
    let exe_path = env::current_exe()?;
    let exe_str = exe_path.to_string_lossy();

    #[cfg(target_family = "unix")]
    {
        let target_profile = get_active_unix_profile();

        if target_profile.exists() {
            let content = with_spinner(
                "Auto-configuring system...".to_string(),
                || -> anyhow::Result<String> {
                    let raw = std::fs::read_to_string(&target_profile)?;
                    Ok(raw)
                },
            )?;
            if content.contains("# ULTIMATE GIT-AI WORKFLOW") {
                logger::path(
                    "⚠️  Configuration already exists in:",
                    &target_profile.display().to_string(),
                );

                let prompt = "🔄 Overwrite existing configuration? (y/N): ";
                if ask_confirm_default_no(prompt)? {
                    clean_profile_file(&target_profile)?;
                    logger::info("🧹 Cleaned old configuration.");
                } else {
                    logger::success("Install cancelled. Kept existing config.");
                    return Ok(());
                }
            }
        }

        let alias_lines = format!(
            "\n# ULTIMATE GIT-AI WORKFLOW\nalias git-copydiff=\"'{}' diff\"\nalias git-go=\"'{}' go\"\nalias git-ai-uninstall=\"'{}' uninstall\"\nalias git-ai=\"'{}'\"\n",
            exe_str, exe_str, exe_str, exe_str
        );

        append_to_file(&target_profile, &alias_lines)?;

        logger::success("Configuration successful! Added aliases:");
        let dummy_locales = Locales::new("English");
        print_commands_help(&dummy_locales);
        logger::note(&format!(
            "\n👉 Please run command: source {}",
            target_profile.display()
        ));
    }

    #[cfg(target_os = "windows")]
    {
        let profile_path = get_windows_profile()?;

        if profile_path.exists() {
            let content = std::fs::read_to_string(&profile_path)?;
            if content.contains("# ULTIMATE GIT-AI WORKFLOW") {
                logger::path(
                    "⚠️  Configuration already exists in:",
                    &profile_path.display().to_string(),
                );

                let prompt = "🔄 Overwrite existing configuration? (y/N): ";
                if ask_confirm_default_no(prompt)? {
                    clean_profile_file(&profile_path)?;
                    logger::info("🧹 Cleaned old configuration.");
                } else {
                    logger::success("Install cancelled. Kept existing config.");
                    return Ok(());
                }
            }
        }

        if let Some(parent) = profile_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let func_lines = format!(
            "\n# ULTIMATE GIT-AI WORKFLOW\nfunction git-copydiff {{ & \"{}\" diff }}\nfunction git-go {{ & \"{}\" go }}\nfunction git-ai-uninstall {{ & \"{}\" uninstall }}\nfunction git-ai {{ & \"{}\" }}\n",
            exe_str, exe_str, exe_str, exe_str
        );

        append_to_file(&profile_path, &func_lines)?;

        logger::success("Configuration successful! Added aliases:");
        let dummy_locales = Locales::new("English");
        print_commands_help(&dummy_locales);
        logger::note("\n👉 Please restart PowerShell to apply new commands.");
    }
    Ok(())
}
