use crossterm::event::{KeyCode, KeyEvent};

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
                    app.status_message = if app.current_lang == "vi" {
                        "❌ Commit message không được để trống!".to_string()
                    } else {
                        "❌ Commit message cannot be empty!".to_string()
                    };
                } else if app.staged_count == 0 {
                    app.status_message = if app.current_lang == "vi" {
                        "⚠️ Chưa có file nào staged! Hãy nhấn [Space] để stage trước.".to_string()
                    } else {
                        "⚠️ No files staged! Press [Space] to stage first.".to_string()
                    };
                } else {
                    match crate::git::commit::commit(&msg) {
                        Ok(_) => {
                            app.status_message = if app.current_lang == "vi" {
                                format!("✅ Đã commit thủ công: {}", msg)
                            } else {
                                format!("✅ Manual commit successful: {}", msg)
                            };
                            app.active_modal = crate::models::ActiveModal::None;
                            app.manual_commit_message.clear();
                            app.refresh_git_status();
                        }
                        Err(e) => {
                            app.status_message = if app.current_lang == "vi" {
                                format!("❌ Commit thất bại: {}", e)
                            } else {
                                format!("❌ Commit failed: {}", e)
                            };
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
                    app.status_message = if app.current_lang == "vi" {
                        format!("⚡ Đang tạo chi nhánh {}...", branch_name)
                    } else {
                        format!("⚡ Creating branch {}...", branch_name)
                    };
                    match crate::git::branch::create_and_checkout_branch(&branch_name) {
                        Ok(_) => {
                            app.status_message = if app.current_lang == "vi" {
                                format!("🌿 Đã tạo và chuyển sang chi nhánh mới: {}", branch_name)
                            } else {
                                format!("🌿 Created and checked out new branch: {}", branch_name)
                            };
                            app.active_modal = crate::models::ActiveModal::None;
                        }
                        Err(err) => {
                            app.status_message = if app.current_lang == "vi" {
                                format!("❌ Lỗi tạo chi nhánh: {}", err)
                            } else {
                                format!("❌ Failed to create branch: {}", err)
                            };
                            app.active_modal = crate::models::ActiveModal::BranchSelect;
                        }
                    }
                    app.refresh_git_status();
                } else {
                    app.status_message = if app.current_lang == "vi" {
                        "❌ Tên chi nhánh không được để trống!".to_string()
                    } else {
                        "❌ Branch name cannot be empty!".to_string()
                    };
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
                    app.status_message = if app.current_lang == "vi" {
                        "⏳ Đang tải thông tin repository từ GitHub...".to_string()
                    } else {
                        "⏳ Fetching repository metadata from GitHub...".to_string()
                    };
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
                    let is_vi = app.current_lang == "vi";
                    let current = std::path::Path::new(&app.current_dir);
                    let target = std::path::Path::new(&app.github_download_target_path);
                    let current_canon = current
                        .canonicalize()
                        .unwrap_or_else(|_| current.to_path_buf());
                    let target_canon = target
                        .canonicalize()
                        .unwrap_or_else(|_| target.to_path_buf());
                    if target_canon.starts_with(&current_canon) || target.starts_with(&current) {
                        app.status_message = if is_vi {
                            "⚠️ Không thể tải vào dự án hiện tại để tránh xung đột file!"
                                .to_string()
                        } else {
                            "⚠️ Cannot download into the current project to avoid file conflicts!"
                                .to_string()
                        };
                        return;
                    }
                    match {
                        app.status_message = if is_vi {
                            "⏳ Đang sao chép tập tin từ GitHub...".to_string()
                        } else {
                            "⏳ Copying files from GitHub...".to_string()
                        };
                        app.copy_github_download_item()
                    } {
                        Ok(_) => {
                            let visible = app.get_visible_github_tree_entries();
                            let selected_name = visible
                                .get(app.selected_github_tree_index)
                                .map(|e| e.name.clone())
                                .unwrap_or_default();
                            app.status_message = if is_vi {
                                format!("✅ Đã tải thành công: {}", selected_name)
                            } else {
                                format!("✅ Downloaded successfully: {}", selected_name)
                            };
                            app.github_temp_dir = None;
                            app.active_modal = crate::models::ActiveModal::None;
                            app.refresh_git_status();
                        }
                        Err(err) => {
                            app.status_message = if is_vi {
                                format!("❌ Lỗi lưu tập tin: {}", err)
                            } else {
                                format!("❌ Save error: {}", err)
                            };
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
                let is_vi = app.current_lang == "vi";
                let selected_path = app.workspace_path_input.trim().to_string();
                if !selected_path.is_empty() {
                    if std::env::set_current_dir(&selected_path).is_ok() {
                        app.current_dir = selected_path.clone();
                        app.add_to_workspace_history(&selected_path);
                        app.refresh_git_status();
                        app.status_message = if is_vi {
                            format!("🔄 Đã chuyển sang Project: {}", selected_path)
                        } else {
                            format!("🔄 Switched to project: {}", selected_path)
                        };
                        app.active_modal = crate::models::ActiveModal::None;
                        app.workspace_path_input.clear();
                    } else {
                        app.status_message = if is_vi {
                            "❌ Lỗi: Không thể truy cập thư mục này.".to_string()
                        } else {
                            "❌ Error: Cannot access this folder.".to_string()
                        };
                    }
                } else {
                    app.status_message = if is_vi {
                        "❌ Đường dẫn không được để trống!".to_string()
                    } else {
                        "❌ Path cannot be empty!".to_string()
                    };
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
