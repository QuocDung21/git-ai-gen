use anyhow::Result;
use rust_i18n::t;
use std::path::Path;

mod auth;
mod cleanup;
mod selector;
mod trash;

use crate::cleanup::CleanupTarget;
use crate::cli::logger;
use cleanup::handle_folder_cleanup;

pub use trash::empty_macos_trash;

pub fn handle_clear_trash(
    path: &Path,
    node_modules: bool,
    build_folders: bool,
    devcleaner: bool,
    select: bool,
) -> Result<()> {
    auth::require_system_authentication()?;

    if node_modules {
        handle_folder_cleanup(path, CleanupTarget::NodeModules, select)?;
    }

    if build_folders {
        handle_folder_cleanup(path, CleanupTarget::BuildFolders, select)?;
    }

    if devcleaner {
        handle_folder_cleanup(path, CleanupTarget::DevCleaner, select)?;
    }

    if node_modules || build_folders || devcleaner {
        return Ok(());
    }

    logger::heading(&t!("clear_trash_heading").to_string());
    #[cfg(not(target_os = "macos"))]
    {
        logger::warn(&t!("clear_trash_unsupported").to_string());
        return Ok(());
    }

    #[cfg(target_os = "macos")]
    {
        if !trash::confirm_empty_trash()? {
            logger::success(&t!("clear_trash_cancelled").to_string());
            return Ok(());
        }

        empty_macos_trash()?;
        logger::success(&t!("clear_trash_success").to_string());
    }

    Ok(())
}
