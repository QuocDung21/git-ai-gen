use std::process::Command;

use crossterm::event::{KeyCode, KeyEvent};
use rust_i18n::t;

use crate::app::App;

pub fn handle_download_tree(app: &mut App, key: &KeyEvent) {
    match key.code {
        KeyCode::Esc => {
            app.github_temp_dir = None;
            app.active_modal = crate::models::ActiveModal::GithubDownloadUrlInput;
        }
        KeyCode::Up | KeyCode::Char('k') => {
            let len = app.get_visible_github_tree_entries().len();
            if len > 0 {
                if app.selected_github_tree_index > 0 {
                    app.selected_github_tree_index -= 1;
                } else {
                    app.selected_github_tree_index = len - 1;
                }
            }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            let len = app.get_visible_github_tree_entries().len();
            if len > 0 {
                if app.selected_github_tree_index < len - 1 {
                    app.selected_github_tree_index += 1;
                } else {
                    app.selected_github_tree_index = 0;
                }
            }
        }
        KeyCode::Char(' ') => {
            app.toggle_github_tree_selection(app.selected_github_tree_index);
        }
        KeyCode::Char('b') | KeyCode::Char('B') => {
            app.status_message = t!("github_branch_fetching").to_string();
            match app.fetch_github_branches() {
                Ok(_) => {
                    app.selected_github_branch_index = app
                        .github_branches
                        .iter()
                        .position(|b| b == &app.current_github_branch)
                        .unwrap_or(0);
                    app.active_modal = crate::models::ActiveModal::GithubBranchSelect;
                    app.status_message = t!("github_branch_ok").to_string();
                }
                Err(e) => {
                    app.status_message =
                        t!("github_branch_fetch_err", err = e.to_string()).to_string();
                }
            }
        }
        KeyCode::Char('v') | KeyCode::Char('V') => {
            let visible = app.get_visible_github_tree_entries();
            if !visible.is_empty() && app.selected_github_tree_index < visible.len() {
                let entry = visible[app.selected_github_tree_index].clone();
                if !entry.is_dir {
                    if let Some(ref dir) = app.github_temp_dir {
                        let _ = Command::new("git")
                            .args(["checkout", "HEAD", &entry.path])
                            .current_dir(dir.path())
                            .output();
                    }
                    app.github_quickview_scroll = 0;
                    app.active_modal = crate::models::ActiveModal::GithubQuickView {
                        path: entry.path.clone(),
                        name: entry.name.clone(),
                    };
                }
            }
        }
        KeyCode::Char('r') => {
            app.github_expanded_dirs.clear();
            let locales = crate::cli::Locales::new(&app.current_lang);
            app.status_message = locales.github_close_all_folders.clone();
        }
        KeyCode::Right => {
            let entry = {
                let visible = app.get_visible_github_tree_entries();
                if app.selected_github_tree_index < visible.len() {
                    Some(visible[app.selected_github_tree_index].clone())
                } else {
                    None
                }
            };
            if let Some(entry) = entry {
                if entry.is_dir {
                    app.github_expanded_dirs.insert(entry.path.clone());
                    let next_len = app.get_visible_github_tree_entries().len();
                    if app.selected_github_tree_index >= next_len {
                        app.selected_github_tree_index = next_len.saturating_sub(1);
                    }
                }
            }
        }
        KeyCode::Left => {
            let entry = {
                let visible = app.get_visible_github_tree_entries();
                if app.selected_github_tree_index < visible.len() {
                    Some(visible[app.selected_github_tree_index].clone())
                } else {
                    None
                }
            };
            if let Some(entry) = entry {
                if entry.is_dir {
                    if app.github_expanded_dirs.contains(&entry.path) {
                        app.github_expanded_dirs.remove(&entry.path);
                        let prefix = format!("{}/", entry.path);
                        app.github_expanded_dirs.retain(|k| !k.starts_with(&prefix));
                    }
                    let next_len = app.get_visible_github_tree_entries().len();
                    if app.selected_github_tree_index >= next_len {
                        app.selected_github_tree_index = next_len.saturating_sub(1);
                    }
                }
            }
        }
        KeyCode::Enter => {
            let len = app.get_visible_github_tree_entries().len();
            if len > 0 && app.selected_github_tree_index < len {
                app.github_download_target_path = app.current_dir.clone();
                app.active_modal = ActiveModal::GithubDownloadTargetInput;
            }
        }
        _ => {}
    }
}

/// Handle GitHub cloning in the pre-poll phase.
/// Returns true if cloning was in progress.
pub fn handle_github_cloning(app: &mut App) -> bool {
    if !app.github_cloning {
        return false;
    }

    let temp_dir = match tempfile::Builder::new()
        .prefix("git_ai_download_")
        .tempdir()
    {
        Ok(dir) => dir,
        Err(e) => {
            app.github_cloning_error =
                Some(t!("github_temp_dir_err", err = e.to_string()).to_string());
            app.github_cloning = false;
            return true;
        }
    };
    let temp_path = temp_dir.path().to_path_buf();
    let output = Command::new("git")
        .args([
            "clone",
            "--depth",
            "1",
            "--filter=blob:none",
            "--no-checkout",
            &app.github_download_url,
            temp_path.to_str().unwrap_or_default(),
        ])
        .output();
    match output {
        Ok(out) if out.status.success() => {
            app.github_temp_dir = Some(temp_dir);
            if let Some(ref dir) = app.github_temp_dir {
                if let Ok(output) = Command::new("git")
                    .args(["symbolic-ref", "--short", "HEAD"])
                    .current_dir(dir.path())
                    .output()
                {
                    if output.status.success() {
                        app.current_github_branch =
                            String::from_utf8_lossy(&output.stdout).trim().to_string();
                    } else {
                        app.current_github_branch = "main".to_string();
                    }
                } else {
                    app.current_github_branch = "main".to_string();
                }
            }
            if let Err(e) = app.visit_repo_dir() {
                app.github_cloning_error = Some(format!("{}", e));
                app.github_cloning = false;
                app.github_temp_dir = None;
                app.active_modal = crate::models::ActiveModal::GithubDownloadUrlInput;
            } else {
                let url = app.github_download_url.trim().to_string();
                app.add_to_github_history(&url);
                app.selected_github_history_index = None;
                app.github_cloning = false;
                app.selected_github_tree_index = 0;
                app.active_modal = crate::models::ActiveModal::GithubDownloadTree;
            }
        }
        Ok(out) => {
            let stderr = String::from_utf8_lossy(&out.stderr).trim().to_string();
            app.github_cloning_error = Some(stderr);
            app.github_cloning = false;
            app.github_temp_dir = None;
            app.active_modal = crate::models::ActiveModal::GithubDownloadUrlInput;
        }
        Err(err) => {
            app.github_cloning_error = Some(format!("{}", err));
            app.github_cloning = false;
            app.github_temp_dir = None;
            app.active_modal = crate::models::ActiveModal::GithubDownloadUrlInput;
        }
    }
    true
}

/// Helper to access ActiveModal variants without full path
use crate::models::ActiveModal;
