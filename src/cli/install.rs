use crate::cli::logger;
use crate::cli::spinner::with_spinner;
use crate::cli::{append_to_file, ask_confirm_default_no, clean_profile_file, print_commands_help};
use anyhow::Result;
use rust_i18n::t;
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
                t!("install_configuring").to_string(),
                || -> anyhow::Result<String> {
                    let raw = std::fs::read_to_string(&target_profile)?;
                    Ok(raw)
                },
            )?;
            if content.contains("# ULTIMATE GIT-AI WORKFLOW") {
                logger::path(
                    t!("install_existing_config").as_ref(),
                    &target_profile.display().to_string(),
                );

                if ask_confirm_default_no(t!("install_overwrite_prompt").as_ref())? {
                    clean_profile_file(&target_profile)?;
                    logger::info(t!("install_cleaned_old_config").as_ref());
                } else {
                    logger::success(t!("install_cancelled_kept_config").as_ref());
                    return Ok(());
                }
            }
        }

        let alias_lines = format!(
            "\n# ULTIMATE GIT-AI WORKFLOW\nalias git-copydiff=\"'{}' diff\"\nalias git-go=\"'{}' go\"\nalias git-clear-trash=\"'{}' clear-trash\"\nalias git-ai-uninstall=\"'{}' uninstall\"\nalias git-ai=\"'{}'\"\n",
            exe_str, exe_str, exe_str, exe_str, exe_str
        );

        append_to_file(&target_profile, &alias_lines)?;

        logger::success(t!("install_success_aliases").as_ref());
        let locales = crate::helper::Helper::get_locales();
        print_commands_help(&locales);
        logger::note(
            t!(
                "install_source_command",
                path = target_profile.display().to_string()
            )
            .as_ref(),
        );
    }

    #[cfg(target_os = "windows")]
    {
        let profile_path = get_windows_profile()?;

        if profile_path.exists() {
            let content = std::fs::read_to_string(&profile_path)?;
            if content.contains("# ULTIMATE GIT-AI WORKFLOW") {
                logger::path(
                    t!("install_existing_config").as_ref(),
                    &profile_path.display().to_string(),
                );

                if ask_confirm_default_no(t!("install_overwrite_prompt").as_ref())? {
                    clean_profile_file(&profile_path)?;
                    logger::info(t!("install_cleaned_old_config").as_ref());
                } else {
                    logger::success(t!("install_cancelled_kept_config").as_ref());
                    return Ok(());
                }
            }
        }

        if let Some(parent) = profile_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let func_lines = format!(
            "\n# ULTIMATE GIT-AI WORKFLOW\nfunction git-copydiff {{ & \"{}\" diff }}\nfunction git-go {{ & \"{}\" go }}\nfunction git-clear-trash {{ & \"{}\" clear-trash }}\nfunction git-ai-uninstall {{ & \"{}\" uninstall }}\nfunction git-ai {{ & \"{}\" }}\n",
            exe_str, exe_str, exe_str, exe_str, exe_str
        );

        append_to_file(&profile_path, &func_lines)?;

        logger::success(t!("install_success_aliases").as_ref());
        let locales = crate::helper::Helper::get_locales();
        print_commands_help(&locales);
        logger::note(t!("install_restart_powershell").as_ref());
    }
    Ok(())
}
