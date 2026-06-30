use anyhow::Result;
use rust_i18n::t;
use std::path::Path;

mod cleanup;
mod scanner;
mod selector;
mod trash;

use crate::cli::logger;
use cleanup::{handle_folder_cleanup, CleanupTarget};

pub use trash::empty_macos_trash;

pub fn handle_clear_trash(
    path: &Path,
    node_modules: bool,
    build_folders: bool,
    select: bool,
) -> Result<()> {
    if node_modules {
        handle_folder_cleanup(path, CleanupTarget::NodeModules, select)?;
    }

    if build_folders {
        handle_folder_cleanup(path, CleanupTarget::BuildFolders, select)?;
    }

    if node_modules || build_folders {
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
