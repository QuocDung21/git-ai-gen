use std::collections::HashSet;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use super::model::{CleanupTarget, CleanupTask};

pub fn scan_folders(root_path: &Path, target: CleanupTarget) -> Vec<CleanupTask> {
    let mut tasks = Vec::new();
    scan_folders_each(root_path, target, |task| tasks.push(task));
    tasks
}

pub fn scan_folders_each<F>(root_path: &Path, target: CleanupTarget, mut on_task: F)
where
    F: FnMut(CleanupTask),
{
    scan_folders_each_until(root_path, target, |task| {
        on_task(task);
        true
    });
}

pub fn scan_folders_each_until<F>(root_path: &Path, target: CleanupTarget, mut on_task: F)
where
    F: FnMut(CleanupTask) -> bool,
{
    let mut emitted = HashSet::new();

    for relative_path in target.known_relative_paths() {
        let path = root_path.join(relative_path);
        if path.is_dir() && mark_emitted(&mut emitted, &path) {
            if !on_task(CleanupTask::new(
                path.clone(),
                target,
                directory_size(path.as_path()),
            )) {
                return;
            }
        }
    }

    let mut walker = WalkDir::new(root_path).into_iter();

    while let Some(entry) = walker.next() {
        let entry = match entry {
            Ok(entry) => entry,
            Err(_) => continue,
        };

        let path = entry.path();

        if target.should_skip_broad_scan_children(root_path, path)
            || target.should_skip_children(path)
        {
            walker.skip_current_dir();
            continue;
        }

        if path.is_dir()
            && path
                .file_name()
                .is_some_and(|name| target.folder_names().iter().any(|folder| name == *folder))
        {
            if mark_emitted(&mut emitted, path) {
                if !on_task(CleanupTask::new(
                    path.to_path_buf(),
                    target,
                    directory_size(path),
                )) {
                    return;
                }
            }
            walker.skip_current_dir();
        }
    }
}

fn mark_emitted(emitted: &mut HashSet<PathBuf>, path: &Path) -> bool {
    if emitted
        .iter()
        .any(|emitted_path| path.starts_with(emitted_path) || emitted_path.starts_with(path))
    {
        return false;
    }

    emitted.insert(path.to_path_buf())
}

fn directory_size(path: &Path) -> u64 {
    WalkDir::new(path)
        .into_iter()
        .filter_map(Result::ok)
        .filter_map(|entry| entry.metadata().ok())
        .filter(|metadata| metadata.is_file())
        .map(|metadata| metadata.len())
        .sum()
}
