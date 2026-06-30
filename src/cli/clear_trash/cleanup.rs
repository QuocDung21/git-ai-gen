use crate::cli::{ask_confirm_default_no, logger};
use anyhow::Result;
use rust_i18n::t;
use std::fs;
use std::path::{Path, PathBuf};

use super::scanner::find_matching_folders;
use super::selector::select_cleanup_folders;

#[derive(Clone, Copy)]
pub(super) enum CleanupTarget {
    NodeModules,
    BuildFolders,
}

impl CleanupTarget {
    pub(super) fn folder_names(self) -> &'static [&'static str] {
        match self {
            CleanupTarget::NodeModules => &["node_modules"],
            CleanupTarget::BuildFolders => &[
                "target",
                "build",
                "dist",
                "out",
                ".next",
                ".nuxt",
                ".svelte-kit",
                ".vite",
                ".cache",
                ".parcel-cache",
                "coverage",
                "__pycache__",
                ".pytest_cache",
                ".mypy_cache",
                ".ruff_cache",
                ".tox",
                ".gradle",
                ".build",
                "DerivedData",
                "cmake-build-debug",
                "cmake-build-release",
            ],
        }
    }

    pub(super) fn skip_folder_names(self) -> &'static [&'static str] {
        match self {
            CleanupTarget::NodeModules => &[],
            CleanupTarget::BuildFolders => &[
                "node_modules",
                "vendor",
                ".git",
                ".svn",
                ".hg",
                ".pnpm-store",
                ".yarn",
                "Pods",
            ],
        }
    }

    pub(super) fn heading_key(self) -> &'static str {
        match self {
            CleanupTarget::NodeModules => "node_modules_search_heading",
            CleanupTarget::BuildFolders => "build_folders_search_heading",
        }
    }

    pub(super) fn root_key(self) -> &'static str {
        match self {
            CleanupTarget::NodeModules => "node_modules_search_root",
            CleanupTarget::BuildFolders => "build_folders_search_root",
        }
    }

    pub(super) fn read_failed_key(self) -> &'static str {
        match self {
            CleanupTarget::NodeModules => "node_modules_search_read_failed",
            CleanupTarget::BuildFolders => "build_folders_search_read_failed",
        }
    }

    pub(super) fn empty_key(self) -> &'static str {
        match self {
            CleanupTarget::NodeModules => "node_modules_search_empty",
            CleanupTarget::BuildFolders => "build_folders_search_empty",
        }
    }

    pub(super) fn found_key(self) -> &'static str {
        match self {
            CleanupTarget::NodeModules => "node_modules_search_done",
            CleanupTarget::BuildFolders => "build_folders_search_done",
        }
    }

    pub(super) fn confirm_key(self) -> &'static str {
        match self {
            CleanupTarget::NodeModules => "node_modules_delete_confirm",
            CleanupTarget::BuildFolders => "build_folders_delete_confirm",
        }
    }

    pub(super) fn cancelled_key(self) -> &'static str {
        match self {
            CleanupTarget::NodeModules => "node_modules_delete_cancelled",
            CleanupTarget::BuildFolders => "build_folders_delete_cancelled",
        }
    }

    pub(super) fn success_key(self) -> &'static str {
        match self {
            CleanupTarget::NodeModules => "node_modules_delete_success",
            CleanupTarget::BuildFolders => "build_folders_delete_success",
        }
    }

    pub(super) fn failed_key(self) -> &'static str {
        match self {
            CleanupTarget::NodeModules => "node_modules_delete_failed",
            CleanupTarget::BuildFolders => "build_folders_delete_failed",
        }
    }

    pub(super) fn done_key(self) -> &'static str {
        match self {
            CleanupTarget::NodeModules => "node_modules_delete_done",
            CleanupTarget::BuildFolders => "build_folders_delete_done",
        }
    }

    pub(super) fn select_title_key(self) -> &'static str {
        match self {
            CleanupTarget::NodeModules => "node_modules_select_title",
            CleanupTarget::BuildFolders => "build_folders_select_title",
        }
    }

    pub(super) fn should_skip_children(self, path: &Path) -> bool {
        path.file_name().is_some_and(|name| {
            self.skip_folder_names()
                .iter()
                .any(|folder| name == *folder)
        })
    }
}

pub(super) fn handle_folder_cleanup(
    root_path: &Path,
    target: CleanupTarget,
    select: bool,
) -> Result<()> {
    let mut folders = find_matching_folders(root_path, target);
    folders.sort();
    print_cleanup_folders(root_path, target, &folders);

    if folders.is_empty() {
        return Ok(());
    }

    let folders = if select {
        match select_cleanup_folders(target, &folders)? {
            Some(selected) => selected,
            None => {
                logger::success(&t!(target.cancelled_key()).to_string());
                return Ok(());
            }
        }
    } else {
        if !ask_confirm_default_no(&t!(target.confirm_key()).to_string())? {
            logger::success(&t!(target.cancelled_key()).to_string());
            return Ok(());
        }

        folders
    };

    delete_folders(target, folders);

    Ok(())
}

fn print_cleanup_folders(root_path: &Path, target: CleanupTarget, folders: &[PathBuf]) {
    logger::heading(&t!(target.heading_key()).to_string());
    logger::info(&format!(
        "{} {}",
        t!(target.root_key()),
        root_path.display()
    ));

    for path in folders {
        logger::text(&path.display().to_string());
    }

    if folders.is_empty() {
        logger::info(&t!(target.empty_key()).to_string());
    } else {
        logger::success(&format!("{} {}", t!(target.found_key()), folders.len()));
    }
}

fn delete_folders(target: CleanupTarget, folders: Vec<PathBuf>) {
    let mut deleted_count = 0;

    for path in folders {
        match fs::remove_dir_all(&path) {
            Ok(_) => {
                deleted_count += 1;
                logger::success(&format!("{} {}", t!(target.success_key()), path.display()));
            }
            Err(error) => {
                logger::warn(&format!(
                    "{} {} ({})",
                    t!(target.failed_key()),
                    path.display(),
                    error
                ));
            }
        }
    }

    logger::success(&format!("{} {}", t!(target.done_key()), deleted_count));
}
