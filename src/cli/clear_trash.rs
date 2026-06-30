use crate::cli::{ask_confirm_default_no, logger};
use anyhow::Result;
use console::{style, Key, Term};
use rust_i18n::t;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

const NODE_MODULES_FOLDER_NAMES: &[&str] = &["node_modules"];
const BUILD_FOLDER_NAMES: &[&str] = &[
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
];
const BUILD_SCAN_SKIP_FOLDER_NAMES: &[&str] = &[
    "node_modules",
    "vendor",
    ".git",
    ".svn",
    ".hg",
    ".pnpm-store",
    ".yarn",
    "Pods",
];

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
        let confirmed = show_macos_confirm_dialog(
            "git-ai-clean",
            &t!("clear_trash_confirm").to_string(),
            "Empty Trash",
            "Cancel",
        )?;

        if !confirmed {
            logger::success(&t!("clear_trash_cancelled").to_string());
            return Ok(());
        }

        empty_macos_trash()?;
        logger::success(&t!("clear_trash_success").to_string());
    }

    Ok(())
}

#[cfg(target_os = "macos")]
fn show_macos_confirm_dialog(
    title: &str,
    message: &str,
    confirm_button: &str,
    cancel_button: &str,
) -> Result<bool> {
    use anyhow::Context;
    use std::process::Command;

    let title = escape_applescript_string(title);
    let message = escape_applescript_string(message);
    let confirm_button = escape_applescript_string(confirm_button);
    let cancel_button = escape_applescript_string(cancel_button);

    let script = format!(
        r#"
        display dialog "{}" ¬
        with title "{}" ¬
        buttons {{"{}", "{}"}} ¬
        default button "{}" ¬
        cancel button "{}" ¬
        with icon caution
        "#,
        message, title, cancel_button, confirm_button, cancel_button, cancel_button
    );

    let output = Command::new("osascript")
        .arg("-e")
        .arg(script)
        .output()
        .context("failed to show macOS confirmation dialog")?;

    Ok(output.status.success())
}

#[cfg(target_os = "macos")]
fn escape_applescript_string(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}

#[derive(Clone, Copy)]
pub enum CleanupTarget {
    NodeModules,
    BuildFolders,
}

impl CleanupTarget {
    fn folder_names(self) -> &'static [&'static str] {
        match self {
            CleanupTarget::NodeModules => NODE_MODULES_FOLDER_NAMES,
            CleanupTarget::BuildFolders => BUILD_FOLDER_NAMES,
        }
    }

    fn heading_key(self) -> &'static str {
        match self {
            CleanupTarget::NodeModules => "node_modules_search_heading",
            CleanupTarget::BuildFolders => "build_folders_search_heading",
        }
    }

    fn root_key(self) -> &'static str {
        match self {
            CleanupTarget::NodeModules => "node_modules_search_root",
            CleanupTarget::BuildFolders => "build_folders_search_root",
        }
    }

    fn read_failed_key(self) -> &'static str {
        match self {
            CleanupTarget::NodeModules => "node_modules_search_read_failed",
            CleanupTarget::BuildFolders => "build_folders_search_read_failed",
        }
    }

    fn empty_key(self) -> &'static str {
        match self {
            CleanupTarget::NodeModules => "node_modules_search_empty",
            CleanupTarget::BuildFolders => "build_folders_search_empty",
        }
    }

    fn found_key(self) -> &'static str {
        match self {
            CleanupTarget::NodeModules => "node_modules_search_done",
            CleanupTarget::BuildFolders => "build_folders_search_done",
        }
    }

    fn confirm_key(self) -> &'static str {
        match self {
            CleanupTarget::NodeModules => "node_modules_delete_confirm",
            CleanupTarget::BuildFolders => "build_folders_delete_confirm",
        }
    }

    fn cancelled_key(self) -> &'static str {
        match self {
            CleanupTarget::NodeModules => "node_modules_delete_cancelled",
            CleanupTarget::BuildFolders => "build_folders_delete_cancelled",
        }
    }

    fn success_key(self) -> &'static str {
        match self {
            CleanupTarget::NodeModules => "node_modules_delete_success",
            CleanupTarget::BuildFolders => "build_folders_delete_success",
        }
    }

    fn failed_key(self) -> &'static str {
        match self {
            CleanupTarget::NodeModules => "node_modules_delete_failed",
            CleanupTarget::BuildFolders => "build_folders_delete_failed",
        }
    }

    fn done_key(self) -> &'static str {
        match self {
            CleanupTarget::NodeModules => "node_modules_delete_done",
            CleanupTarget::BuildFolders => "build_folders_delete_done",
        }
    }

    fn select_title_key(self) -> &'static str {
        match self {
            CleanupTarget::NodeModules => "node_modules_select_title",
            CleanupTarget::BuildFolders => "build_folders_select_title",
        }
    }
}

pub fn handle_folder_cleanup(root_path: &Path, target: CleanupTarget, select: bool) -> Result<()> {
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

    Ok(())
}

pub fn select_cleanup_folders(
    target: CleanupTarget,
    folders: &[PathBuf],
) -> Result<Option<Vec<PathBuf>>> {
    let term = Term::stdout();
    let mut cursor = 0;
    let mut offset = 0;
    let mut selected = vec![false; folders.len()];

    loop {
        render_select_cleanup_folders(&term, target, folders, &selected, cursor, offset)?;

        match term.read_key()? {
            Key::ArrowUp => {
                cursor = cursor.saturating_sub(1);
                if cursor < offset {
                    offset = cursor;
                }
            }
            Key::ArrowDown => {
                if cursor + 1 < folders.len() {
                    cursor += 1;
                }
                let page_size = cleanup_select_page_size();
                if cursor >= offset + page_size {
                    offset = cursor + 1 - page_size;
                }
            }
            Key::Char(' ') => {
                selected[cursor] = !selected[cursor];
            }
            Key::Char('a') | Key::Char('A') => {
                let should_select = selected.iter().any(|selected| !selected);
                selected.fill(should_select);
            }
            Key::Enter => {
                let selected_paths = folders
                    .iter()
                    .zip(selected.iter())
                    .filter_map(|(path, selected)| selected.then(|| path.clone()))
                    .collect::<Vec<_>>();

                if selected_paths.is_empty() {
                    logger::warn(&t!("cleanup_select_empty").to_string());
                    continue;
                }

                term.clear_screen()?;
                return Ok(Some(selected_paths));
            }
            Key::Escape | Key::Char('q') | Key::Char('Q') => {
                term.clear_screen()?;
                return Ok(None);
            }
            _ => {}
        }
    }
}

fn render_select_cleanup_folders(
    term: &Term,
    target: CleanupTarget,
    folders: &[PathBuf],
    selected: &[bool],
    cursor: usize,
    offset: usize,
) -> Result<()> {
    term.clear_screen()?;
    logger::heading(&t!(target.select_title_key()).to_string());
    logger::info(&t!("cleanup_select_help").to_string());

    let page_size = cleanup_select_page_size();
    let end = folders.len().min(offset + page_size);

    for index in offset..end {
        let cursor_marker = if index == cursor { ">" } else { " " };
        let check_marker = if selected[index] { "[x]" } else { "[ ]" };
        println!(
            "{} {} {}",
            style(cursor_marker).cyan().bold(),
            style(check_marker).green(),
            folders[index].display()
        );
    }

    println!(
        "{}",
        style(format!(
            "{} {} / {}",
            t!("cleanup_select_counter"),
            selected.iter().filter(|selected| **selected).count(),
            folders.len()
        ))
        .yellow()
    );
    io::stdout().flush()?;

    Ok(())
}

fn cleanup_select_page_size() -> usize {
    15
}

pub fn print_cleanup_folders(root_path: &Path, target: CleanupTarget, folders: &[PathBuf]) {
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

pub fn find_matching_folders(root_path: &Path, target: CleanupTarget) -> Vec<PathBuf> {
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

impl CleanupTarget {
    fn should_skip_children(self, path: &Path) -> bool {
        matches!(self, CleanupTarget::BuildFolders)
            && path.file_name().is_some_and(|name| {
                BUILD_SCAN_SKIP_FOLDER_NAMES
                    .iter()
                    .any(|folder| name == *folder)
            })
    }
}

#[cfg(target_os = "macos")]
pub fn empty_macos_trash() -> Result<()> {
    use anyhow::{bail, Context};
    use std::process::Command;

    let output = Command::new("osascript")
        .arg("-e")
        .arg(r#"tell application "Finder" to empty trash"#)
        .output()
        .context("failed to run osascript")?;

    if output.status.success() {
        return Ok(());
    }

    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_owned();

    if stderr.contains("is in use") {
        bail!(
            "{}\n{}",
            stderr,
            "Trash contains an item currently in use. Close the related app/process, or use force clean."
        );
    }

    if stderr.is_empty() {
        bail!("{}", t!("clear_trash_failed"));
    }

    bail!("{} {}", t!("clear_trash_failed"), stderr);
}

#[cfg(not(target_os = "macos"))]
pub fn empty_macos_trash() -> Result<()> {
    Ok(())
}
