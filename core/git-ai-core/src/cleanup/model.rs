use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub enum CleanupTarget {
    NodeModules,
    BuildFolders,
    DevCleaner,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct CleanupTask {
    pub path: PathBuf,
    pub target: CleanupTarget,
    pub size_bytes: u64,
}

impl CleanupTask {
    pub fn new(path: PathBuf, target: CleanupTarget, size_bytes: u64) -> Self {
        Self {
            path,
            target,
            size_bytes,
        }
    }
}

pub fn format_size_bytes(size_bytes: u64) -> String {
    let units = ["B", "KB", "MB", "GB", "TB"];
    let mut size = size_bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index + 1 < units.len() {
        size /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{} {}", size_bytes, units[unit_index])
    } else {
        format!("{:.1} {}", size, units[unit_index])
    }
}

impl CleanupTarget {
    pub fn folder_names(self) -> &'static [&'static str] {
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
            CleanupTarget::DevCleaner => &[
                "node_modules",
                "target",
                "build",
                "dist",
                "out",
                ".next",
                ".nuxt",
                ".svelte-kit",
                ".vite",
                ".turbo",
                ".expo",
                ".serverless",
                ".parcel-cache",
                ".dart_tool",
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

    pub fn skip_folder_names(self) -> &'static [&'static str] {
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
            CleanupTarget::DevCleaner => &[".git", ".svn", ".hg", "vendor", "Pods"],
        }
    }

    pub fn broad_scan_skip_folder_names(self) -> &'static [&'static str] {
        match self {
            CleanupTarget::DevCleaner => &[
                "Applications",
                "Desktop",
                "Downloads",
                "Library",
                "Movies",
                "Music",
                "Pictures",
                "Public",
                ".Trash",
                ".cargo",
                ".rustup",
                ".local",
            ],
            _ => &[],
        }
    }

    pub fn broad_scan_allow_folder_names(self) -> &'static [&'static str] {
        match self {
            CleanupTarget::DevCleaner => &[
                "Documents",
                "Developer",
                "Projects",
                "Workspace",
                "Workspaces",
                "Wordspace",
                "Code",
                ".cache",
                ".codex",
                ".npm",
                ".pnpm-store",
                ".gradle",
            ],
            _ => &[],
        }
    }

    pub fn should_skip_children(self, path: &Path) -> bool {
        path.file_name().is_some_and(|name| {
            self.skip_folder_names()
                .iter()
                .any(|folder| name == *folder)
        })
    }

    pub fn should_skip_broad_scan_children(self, root_path: &Path, path: &Path) -> bool {
        if self.broad_scan_skip_folder_names().is_empty()
            && self.broad_scan_allow_folder_names().is_empty()
        {
            return false;
        }

        if !is_broad_root(root_path) {
            return false;
        }

        let Ok(relative_path) = path.strip_prefix(root_path) else {
            return false;
        };

        if relative_path.components().count() != 1 {
            return false;
        }

        path.file_name().is_some_and(|name| {
            self.broad_scan_skip_folder_names()
                .iter()
                .any(|folder| name == *folder)
                || !self
                    .broad_scan_allow_folder_names()
                    .iter()
                    .any(|folder| name == *folder)
        })
    }

    pub fn known_relative_paths(self) -> &'static [&'static str] {
        match self {
            CleanupTarget::DevCleaner => &[
                "Library/Developer/Xcode/DerivedData",
                "Library/Developer/Xcode/Archives",
                "Library/Developer/Xcode/iOS DeviceSupport",
                "Library/Developer/Xcode/watchOS DeviceSupport",
                "Library/Developer/Xcode/tvOS DeviceSupport",
                "Library/Developer/Xcode/UserData/Previews",
                "Library/Developer/CoreSimulator/Caches",
                "Library/Caches/com.apple.dt.Xcode",
                "Library/Caches/org.swift.swiftpm",
                "Library/org.swift.swiftpm",
                ".cache/clang",
                ".gradle/caches",
                ".npm",
                ".pnpm-store",
            ],
            _ => &[],
        }
    }

    pub fn heading_key(self) -> &'static str {
        match self {
            CleanupTarget::NodeModules => "node_modules_search_heading",
            CleanupTarget::BuildFolders => "build_folders_search_heading",
            CleanupTarget::DevCleaner => "devcleaner_search_heading",
        }
    }

    pub fn root_key(self) -> &'static str {
        match self {
            CleanupTarget::NodeModules => "node_modules_search_root",
            CleanupTarget::BuildFolders => "build_folders_search_root",
            CleanupTarget::DevCleaner => "devcleaner_search_root",
        }
    }

    pub fn empty_key(self) -> &'static str {
        match self {
            CleanupTarget::NodeModules => "node_modules_search_empty",
            CleanupTarget::BuildFolders => "build_folders_search_empty",
            CleanupTarget::DevCleaner => "devcleaner_search_empty",
        }
    }

    pub fn found_key(self) -> &'static str {
        match self {
            CleanupTarget::NodeModules => "node_modules_search_done",
            CleanupTarget::BuildFolders => "build_folders_search_done",
            CleanupTarget::DevCleaner => "devcleaner_search_done",
        }
    }

    pub fn confirm_key(self) -> &'static str {
        match self {
            CleanupTarget::NodeModules => "node_modules_delete_confirm",
            CleanupTarget::BuildFolders => "build_folders_delete_confirm",
            CleanupTarget::DevCleaner => "devcleaner_delete_confirm",
        }
    }

    pub fn cancelled_key(self) -> &'static str {
        match self {
            CleanupTarget::NodeModules => "node_modules_delete_cancelled",
            CleanupTarget::BuildFolders => "build_folders_delete_cancelled",
            CleanupTarget::DevCleaner => "devcleaner_delete_cancelled",
        }
    }

    pub fn success_key(self) -> &'static str {
        match self {
            CleanupTarget::NodeModules => "node_modules_delete_success",
            CleanupTarget::BuildFolders => "build_folders_delete_success",
            CleanupTarget::DevCleaner => "devcleaner_delete_success",
        }
    }

    pub fn failed_key(self) -> &'static str {
        match self {
            CleanupTarget::NodeModules => "node_modules_delete_failed",
            CleanupTarget::BuildFolders => "build_folders_delete_failed",
            CleanupTarget::DevCleaner => "devcleaner_delete_failed",
        }
    }

    pub fn done_key(self) -> &'static str {
        match self {
            CleanupTarget::NodeModules => "node_modules_delete_done",
            CleanupTarget::BuildFolders => "build_folders_delete_done",
            CleanupTarget::DevCleaner => "devcleaner_delete_done",
        }
    }

    pub fn select_title_key(self) -> &'static str {
        match self {
            CleanupTarget::NodeModules => "node_modules_select_title",
            CleanupTarget::BuildFolders => "build_folders_select_title",
            CleanupTarget::DevCleaner => "devcleaner_select_title",
        }
    }
}

fn is_broad_root(path: &Path) -> bool {
    let home_path = std::env::var("HOME").ok().map(PathBuf::from);

    path == Path::new("/")
        || path == Path::new("/Users")
        || home_path.as_deref().is_some_and(|home| path == home)
}
