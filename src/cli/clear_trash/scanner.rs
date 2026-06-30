use crate::cli::logger;
use rust_i18n::t;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use super::cleanup::CleanupTarget;

pub(super) fn find_matching_folders(root_path: &Path, target: CleanupTarget) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    let mut walker = WalkDir::new(root_path).into_iter();

    while let Some(entry) = walker.next() {
        let entry = match entry {
            Ok(entry) => entry,
            Err(error) => {
                logger::warn(&format!("{} {}", t!(target.read_failed_key()), error));
                continue;
            }
        };

        let path = entry.path();

        if target.should_skip_children(path) {
            walker.skip_current_dir();
            continue;
        }

        if path.is_dir()
            && path
                .file_name()
                .is_some_and(|name| target.folder_names().iter().any(|folder| name == *folder))
        {
            paths.push(path.to_path_buf());
            walker.skip_current_dir();
        }
    }

    paths
}
