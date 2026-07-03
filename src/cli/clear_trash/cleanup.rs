use crate::cleanup::{delete_folders, format_size_bytes, scan_folders, CleanupTarget, CleanupTask};
use crate::cli::{ask_confirm_default_no, logger};
use anyhow::Result;
use rust_i18n::t;
use std::path::Path;

use super::selector::select_cleanup_folders;

impl CleanupTarget {
    pub(super) fn heading_key(self) -> &'static str {
        match self {
            CleanupTarget::NodeModules => "node_modules_search_heading",
            CleanupTarget::BuildFolders => "build_folders_search_heading",
            CleanupTarget::DevCleaner => "devcleaner_search_heading",
        }
    }

    pub(super) fn root_key(self) -> &'static str {
        match self {
            CleanupTarget::NodeModules => "node_modules_search_root",
            CleanupTarget::BuildFolders => "build_folders_search_root",
            CleanupTarget::DevCleaner => "devcleaner_search_root",
        }
    }

    pub(super) fn empty_key(self) -> &'static str {
        match self {
            CleanupTarget::NodeModules => "node_modules_search_empty",
            CleanupTarget::BuildFolders => "build_folders_search_empty",
            CleanupTarget::DevCleaner => "devcleaner_search_empty",
        }
    }

    pub(super) fn found_key(self) -> &'static str {
        match self {
            CleanupTarget::NodeModules => "node_modules_search_done",
            CleanupTarget::BuildFolders => "build_folders_search_done",
            CleanupTarget::DevCleaner => "devcleaner_search_done",
        }
    }

    pub(super) fn confirm_key(self) -> &'static str {
        match self {
            CleanupTarget::NodeModules => "node_modules_delete_confirm",
            CleanupTarget::BuildFolders => "build_folders_delete_confirm",
            CleanupTarget::DevCleaner => "devcleaner_delete_confirm",
        }
    }

    pub(super) fn cancelled_key(self) -> &'static str {
        match self {
            CleanupTarget::NodeModules => "node_modules_delete_cancelled",
            CleanupTarget::BuildFolders => "build_folders_delete_cancelled",
            CleanupTarget::DevCleaner => "devcleaner_delete_cancelled",
        }
    }

    pub(super) fn success_key(self) -> &'static str {
        match self {
            CleanupTarget::NodeModules => "node_modules_delete_success",
            CleanupTarget::BuildFolders => "build_folders_delete_success",
            CleanupTarget::DevCleaner => "devcleaner_delete_success",
        }
    }

    pub(super) fn failed_key(self) -> &'static str {
        match self {
            CleanupTarget::NodeModules => "node_modules_delete_failed",
            CleanupTarget::BuildFolders => "build_folders_delete_failed",
            CleanupTarget::DevCleaner => "devcleaner_delete_failed",
        }
    }

    pub(super) fn done_key(self) -> &'static str {
        match self {
            CleanupTarget::NodeModules => "node_modules_delete_done",
            CleanupTarget::BuildFolders => "build_folders_delete_done",
            CleanupTarget::DevCleaner => "devcleaner_delete_done",
        }
    }

    pub(super) fn select_title_key(self) -> &'static str {
        match self {
            CleanupTarget::NodeModules => "node_modules_select_title",
            CleanupTarget::BuildFolders => "build_folders_select_title",
            CleanupTarget::DevCleaner => "devcleaner_select_title",
        }
    }
}

pub(super) fn handle_folder_cleanup(
    root_path: &Path,
    target: CleanupTarget,
    select: bool,
) -> Result<()> {
    let mut tasks = scan_folders(root_path, target);
    tasks.sort_by(|a, b| a.path.cmp(&b.path));
    print_cleanup_tasks(root_path, target, &tasks);

    if tasks.is_empty() {
        return Ok(());
    }

    let tasks = if select {
        match select_cleanup_folders(target, &tasks)? {
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

        tasks
    };

    delete_and_print_reports(target, &tasks);

    Ok(())
}

fn print_cleanup_tasks(root_path: &Path, target: CleanupTarget, tasks: &[CleanupTask]) {
    logger::heading(&t!(target.heading_key()).to_string());
    logger::info(&format!(
        "{} {}",
        t!(target.root_key()),
        root_path.display()
    ));

    for task in tasks {
        logger::text(&format!(
            "{} ({})",
            task.path.display(),
            format_size_bytes(task.size_bytes)
        ));
    }

    if tasks.is_empty() {
        logger::info(&t!(target.empty_key()).to_string());
    } else {
        logger::success(&format!("{} {}", t!(target.found_key()), tasks.len()));
        logger::info(&format!(
            "{} {}",
            t!("cleanup_total_size"),
            format_size_bytes(total_size(tasks))
        ));
    }
}

fn delete_and_print_reports(target: CleanupTarget, tasks: &[CleanupTask]) {
    let reports = delete_folders(tasks);
    let mut deleted_count = 0;

    for report in reports {
        if report.deleted {
            deleted_count += 1;
            logger::success(&format!("{} {}", t!(target.success_key()), report.path));
        } else {
            logger::warn(&format!(
                "{} {} ({})",
                t!(target.failed_key()),
                report.path,
                report.error.unwrap_or_default()
            ));
        }
    }

    logger::success(&format!("{} {}", t!(target.done_key()), deleted_count));
}

fn total_size(tasks: &[CleanupTask]) -> u64 {
    tasks.iter().map(|task| task.size_bytes).sum()
}
