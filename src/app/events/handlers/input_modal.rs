use crossterm::event::{KeyCode, KeyEvent};
use rust_i18n::t;

use crate::app::App;

pub fn handle_input_modal_keys(app: &mut App, key: &KeyEvent) {
    match &app.active_modal {
        crate::models::ActiveModal::ManualCommit => match key.code {
            KeyCode::Esc => {
                app.active_modal = crate::models::ActiveModal::None;
            }
            KeyCode::Enter => {
                let msg = app.manual_commit_message.trim().to_string();
                if msg.is_empty() {
                    app.status_message = t!("input_commit_empty").to_string();
                } else if app.staged_count == 0 {
                    app.status_message = t!("input_no_staged").to_string();
                } else {
                    match crate::git::commit::commit(&msg) {
                        Ok(_) => {
                            app.status_message = t!("input_commit_ok", msg = msg.clone()).to_string();
                            app.active_modal = crate::models::ActiveModal::None;
                            app.manual_commit_message.clear();
                            app.refresh_git_status();
                        }
                        Err(e) => {
                            app.status_message = t!("input_commit_err", err = e.to_string()).to_string();
                        }
                    }
                }
            }
            KeyCode::Backspace => {
                app.manual_commit_message.pop();
            }
            KeyCode::Char('v') | KeyCode::Char('V')
                if key
                    .modifiers
                    .contains(crossterm::event::KeyModifiers::CONTROL) =>
            {
                if let Ok(mut cb) = arboard::Clipboard::new() {
                    if let Ok(text) = cb.get_text() {
                        app.manual_commit_message.push_str(&text);
                    }
                }
            }
            KeyCode::Char(c) => {
                app.manual_commit_message.push(c);
            }
            _ => {}
        },
        crate::models::ActiveModal::NewBranchInput => match key.code {
            KeyCode::Esc => {
                app.active_modal = crate::models::ActiveModal::BranchSelect;
            }
            KeyCode::Enter => {
                let branch_name = app.new_branch_name.trim().to_string();
                if !branch_name.is_empty() {
                    app.status_message = t!("input_branch_creating", name = branch_name.clone()).to_string();
                    match crate::git::branch::create_and_checkout_branch(&branch_name) {
                        Ok(_) => {
                            app.status_message = t!("input_branch_ok", name = branch_name.clone()).to_string();
                            app.active_modal = crate::models::ActiveModal::None;
                        }
                        Err(err) => {
                            app.status_message = t!("input_branch_err", err = err.to_string()).to_string();
                            app.active_modal = crate::models::ActiveModal::BranchSelect;
                        }
                    }
                    app.refresh_git_status();
                } else {
                    app.status_message = t!("input_branch_empty").to_string();
                }
            }
            KeyCode::Backspace => {
                app.new_branch_name.pop();
            }
            KeyCode::Char(c) => {
                app.new_branch_name.push(c);
            }
            _ => {}
        },
        crate::models::ActiveModal::GithubDownloadUrlInput => match key.code {
            KeyCode::Esc => {
                app.active_modal = crate::models::ActiveModal::None;
            }
            KeyCode::Enter => {
                let url = app.github_download_url.trim().to_string();
                if !url.is_empty() {
                    app.status_message = t!("input_github_fetching").to_string();
                    app.github_cloning = true;
                    app.github_cloning_error = None;
                }
            }
            KeyCode::Up | KeyCode::Char('k') | KeyCode::Char('K') => {
                if !app.github_history.is_empty() {
                    if app.selected_github_history_index.is_none() {
                        app.github_download_url_temp = app.github_download_url.clone();
                        app.selected_github_history_index = Some(app.github_history.len() - 1);
                        app.github_download_url =
                            app.github_history[app.github_history.len() - 1].clone();
                    } else if let Some(idx) = app.selected_github_history_index {
                        if idx > 0 {
                            app.selected_github_history_index = Some(idx - 1);
                            app.github_download_url = app.github_history[idx - 1].clone();
                        } else {
                            app.selected_github_history_index = None;
                            app.github_download_url = app.github_download_url_temp.clone();
                        }
                    }
                }
            }
            KeyCode::Down | KeyCode::Char('j') | KeyCode::Char('J') => {
                if !app.github_history.is_empty() {
                    if app.selected_github_history_index.is_none() {
                        app.github_download_url_temp = app.github_download_url.clone();
                        app.selected_github_history_index = Some(0);
                        app.github_download_url = app.github_history[0].clone();
                    } else if let Some(idx) = app.selected_github_history_index {
                        if idx < app.github_history.len() - 1 {
                            app.selected_github_history_index = Some(idx + 1);
                            app.github_download_url = app.github_history[idx + 1].clone();
                        } else {
                            app.selected_github_history_index = None;
                            app.github_download_url = app.github_download_url_temp.clone();
                        }
                    }
                }
            }
            KeyCode::Backspace => {
                if let Some(idx) = app.selected_github_history_index {
                    app.remove_from_github_history(idx);
                    if app.github_history.is_empty() {
                        app.selected_github_history_index = None;
                        app.github_download_url = app.github_download_url_temp.clone();
                    } else if let Some(new_idx) = app.selected_github_history_index {
                        app.github_download_url = app.github_history[new_idx].clone();
                    }
                } else {
                    app.github_download_url.pop();
                    app.github_download_url_temp = app.github_download_url.clone();
                }
            }
            KeyCode::Char(c) => {
                app.selected_github_history_index = None;
                app.github_download_url.push(c);
                app.github_download_url_temp = app.github_download_url.clone();
            }
            _ => {}
        },
        crate::models::ActiveModal::GithubDownloadTargetInput => {
            match key.code {
                KeyCode::Esc => {
                    app.active_modal = crate::models::ActiveModal::GithubDownloadTree;
                }
                KeyCode::Tab => {
                    if let Ok(output) = Command::new("osascript")
                        .args(["-e", "POSIX path of (choose folder with prompt \"Select Destination Folder:\")"])
                        .output()
                    {
                        if output.status.success() {
                            let path_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
                            if !path_str.is_empty() {
                                app.github_download_target_path = path_str;
                            }
                        }
                    }
                }
                KeyCode::Enter => {
                    let current = std::path::Path::new(&app.current_dir);
                    let target = std::path::Path::new(&app.github_download_target_path);
                    let current_canon = current
                        .canonicalize()
                        .unwrap_or_else(|_| current.to_path_buf());
                    let target_canon = target
                        .canonicalize()
                        .unwrap_or_else(|_| target.to_path_buf());
                    if target_canon.starts_with(&current_canon) || target.starts_with(&current) {
                        app.status_message = t!("input_download_conflict").to_string();
                        return;
                    }
                    match {
                        app.status_message = t!("input_download_copying").to_string();
                        app.copy_github_download_item()
                    } {
                        Ok(_) => {
                            let visible = app.get_visible_github_tree_entries();
                            let selected_name = visible
                                .get(app.selected_github_tree_index)
                                .map(|e| e.name.clone())
                                .unwrap_or_default();
                            app.status_message = t!("input_download_ok", name = selected_name).to_string();
                            app.github_temp_dir = None;
                            app.active_modal = crate::models::ActiveModal::None;
                            app.refresh_git_status();
                        }
                        Err(err) => {
                            app.status_message = t!("input_download_err", err = err.to_string()).to_string();
                            app.github_temp_dir = None;
                        }
                    }
                }
                KeyCode::Backspace => {
                    app.github_download_target_path.pop();
                }
                KeyCode::Char(c) => {
                    app.github_download_target_path.push(c);
                }
                _ => {}
            }
        }
        crate::models::ActiveModal::WorkspacePathInput => match key.code {
            KeyCode::Esc => {
                app.active_modal = crate::models::ActiveModal::WorkspaceHistory;
            }
            KeyCode::Enter => {
                let selected_path = app.workspace_path_input.trim().to_string();
                if !selected_path.is_empty() {
                    if std::env::set_current_dir(&selected_path).is_ok() {
                        app.current_dir = selected_path.clone();
                        app.add_to_workspace_history(&selected_path);
                        app.refresh_git_status();
                        app.status_message = t!("input_workspace_ok", path = selected_path.clone()).to_string();
                        app.active_modal = crate::models::ActiveModal::None;
                        app.workspace_path_input.clear();
                    } else {
                        app.status_message = t!("input_workspace_err").to_string();
                    }
                } else {
                    app.status_message = t!("input_workspace_empty").to_string();
                }
            }
            KeyCode::Backspace => {
                app.workspace_path_input.pop();
            }
            KeyCode::Char('v') | KeyCode::Char('V')
                if key
                    .modifiers
                    .contains(crossterm::event::KeyModifiers::CONTROL) =>
            {
                if let Ok(mut cb) = arboard::Clipboard::new() {
                    if let Ok(text) = cb.get_text() {
                        app.workspace_path_input.push_str(&text);
                    }
                }
            }
            KeyCode::Char(c) => {
                app.workspace_path_input.push(c);
            }
            _ => {}
        }
        _ => {}
    }
}

use std::process::Command;
