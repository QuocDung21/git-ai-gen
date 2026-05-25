use std::process::Command;

use crossterm::event::{KeyCode, KeyEvent};

use crate::app::App;

pub fn handle_select_modal_keys(app: &mut App, key: &KeyEvent) {
    match &app.active_modal.clone() {
        crate::models::ActiveModal::Help => match key.code {
            KeyCode::Esc
            | KeyCode::Char(' ')
            | KeyCode::Enter
            | KeyCode::Char('q')
            | KeyCode::Char('?')
            | KeyCode::Char('h') => {
                app.active_modal = crate::models::ActiveModal::None;
            }
            _ => {}
        },
        crate::models::ActiveModal::LanguageSelect => match key.code {
            KeyCode::Esc | KeyCode::Char('q') => {
                app.active_modal = crate::models::ActiveModal::None;
            }
            KeyCode::Char('v') | KeyCode::Char('V') => {
                let locales = crate::cli::Locales::new(&app.current_lang);
                if let Ok(msg) = crate::cli::system::handle_lang("vi", &locales) {
                    app.status_message = msg;
                    app.current_lang = crate::helper::Helper::get_ai_language();
                }
                app.active_modal = crate::models::ActiveModal::None;
            }
            KeyCode::Char('e') | KeyCode::Char('E') => {
                let locales = crate::cli::Locales::new(&app.current_lang);
                if let Ok(msg) = crate::cli::system::handle_lang("en", &locales) {
                    app.status_message = msg;
                    app.current_lang = crate::helper::Helper::get_ai_language();
                }
                app.active_modal = crate::models::ActiveModal::None;
            }
            KeyCode::Char('a') | KeyCode::Char('A') => {
                let locales = crate::cli::Locales::new(&app.current_lang);
                if let Ok(msg) = crate::cli::system::handle_lang("auto", &locales) {
                    app.status_message = msg;
                    app.current_lang = crate::helper::Helper::get_ai_language();
                }
                app.active_modal = crate::models::ActiveModal::None;
            }
            KeyCode::Up | KeyCode::Char('k') => {
                if app.selected_lang_index > 0 {
                    app.selected_lang_index -= 1;
                } else {
                    app.selected_lang_index = 2;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if app.selected_lang_index < 2 {
                    app.selected_lang_index += 1;
                } else {
                    app.selected_lang_index = 0;
                }
            }
            KeyCode::Enter => {
                let selection = match app.selected_lang_index {
                    0 => "vi",
                    1 => "en",
                    _ => "auto",
                };
                let locales = crate::cli::Locales::new(&app.current_lang);
                if let Ok(msg) = crate::cli::system::handle_lang(selection, &locales) {
                    app.status_message = msg;
                    app.current_lang = crate::helper::Helper::get_ai_language();
                }
                app.active_modal = crate::models::ActiveModal::None;
            }
            _ => {}
        },
        crate::models::ActiveModal::ThemeSelect => {
            let themes = crate::theme::get_all_themes();
            let themes_len = themes.len();
            match key.code {
                KeyCode::Esc => {
                    app.active_modal = crate::models::ActiveModal::None;
                }
                KeyCode::Char(c) => {
                    let lower_c = c.to_lowercase().to_string();
                    if lower_c == "q" {
                        app.active_modal = crate::models::ActiveModal::None;
                    } else if lower_c == "j" {
                        if app.selected_theme_index < themes_len - 1 {
                            app.selected_theme_index += 1;
                        } else {
                            app.selected_theme_index = 0;
                        }
                    } else if lower_c == "k" {
                        if app.selected_theme_index > 0 {
                            app.selected_theme_index -= 1;
                        } else {
                            app.selected_theme_index = themes_len - 1;
                        }
                    } else {
                        if let Some((idx, t_info)) = themes
                            .iter()
                            .enumerate()
                            .find(|(_, t)| t.hotkey.to_lowercase().to_string() == lower_c)
                        {
                            apply_theme(app, idx, t_info);
                        }
                    }
                }
                KeyCode::Up => {
                    if app.selected_theme_index > 0 {
                        app.selected_theme_index -= 1;
                    } else {
                        app.selected_theme_index = themes_len - 1;
                    }
                }
                KeyCode::Down => {
                    if app.selected_theme_index < themes_len - 1 {
                        app.selected_theme_index += 1;
                    } else {
                        app.selected_theme_index = 0;
                    }
                }
                KeyCode::Enter => {
                    if app.selected_theme_index < themes_len {
                        let t_info = &themes[app.selected_theme_index];
                        apply_theme(app, app.selected_theme_index, t_info);
                    }
                }
                _ => {}
            }
        }
        crate::models::ActiveModal::Settings => {
            let is_vi = app.current_lang == "vi";
            match key.code {
                KeyCode::Esc | KeyCode::Char('q') => {
                    app.active_modal = crate::models::ActiveModal::None;
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    if app.selected_setting_index > 0 {
                        app.selected_setting_index -= 1;
                    } else {
                        app.selected_setting_index = 3;
                    }
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    if app.selected_setting_index < 3 {
                        app.selected_setting_index += 1;
                    } else {
                        app.selected_setting_index = 0;
                    }
                }
                KeyCode::Char(' ') | KeyCode::Enter => match app.selected_setting_index {
                    0 => {
                        app.auto_push = !app.auto_push;
                        let val = app.auto_push.to_string();
                        let _ = Command::new("git")
                            .args(["config", "--global", "git-ai.auto-push", &val])
                            .output();
                        app.status_message = if is_vi {
                            format!(
                                "⚙️ Tự động Push: {}",
                                if app.auto_push { "BẬT" } else { "TẮT" }
                            )
                        } else {
                            format!("⚙️ Auto Push: {}", if app.auto_push { "ON" } else { "OFF" })
                        };
                    }
                    1 => {
                        app.auto_stage_all = !app.auto_stage_all;
                        let val = app.auto_stage_all.to_string();
                        let _ = Command::new("git")
                            .args(["config", "--global", "git-ai.auto-stage-all", &val])
                            .output();
                        app.status_message = if is_vi {
                            format!(
                                "⚙️ Tự động Stage tất cả: {}",
                                if app.auto_stage_all { "BẬT" } else { "TẮT" }
                            )
                        } else {
                            format!(
                                "⚙️ Auto Stage All: {}",
                                if app.auto_stage_all { "ON" } else { "OFF" }
                            )
                        };
                    }
                    2 => {
                        app.kilo_ai_enabled = !app.kilo_ai_enabled;
                        let val = app.kilo_ai_enabled.to_string();
                        let _ = Command::new("git")
                            .args(["config", "--global", "git-ai.kilo-ai", &val])
                            .output();
                        app.status_message = if is_vi {
                            format!(
                                "⚙️ Kilo AI: {}",
                                if app.kilo_ai_enabled {
                                    "BẬT"
                                } else {
                                    "TẮT"
                                }
                            )
                        } else {
                            format!(
                                "⚙️ Kilo AI Generation: {}",
                                if app.kilo_ai_enabled { "ON" } else { "OFF" }
                            )
                        };
                    }
                    3 => {
                        app.splash_enabled = !app.splash_enabled;
                        let val = app.splash_enabled.to_string();
                        let _ = Command::new("git")
                            .args(["config", "--global", "git-ai.splash", &val])
                            .output();
                        app.status_message = if is_vi {
                            format!(
                                "⚙️ Hiển thị Splash: {}",
                                if app.splash_enabled {
                                    "BẬT"
                                } else {
                                    "TẮT"
                                }
                            )
                        } else {
                            format!(
                                "⚙️ Show Splash Screen: {}",
                                if app.splash_enabled { "ON" } else { "OFF" }
                            )
                        };
                    }
                    _ => {}
                },
                _ => {}
            }
        }
        crate::models::ActiveModal::WorkspaceHistory => {
            let is_vi = app.current_lang == "vi";
            match key.code {
                KeyCode::Esc | KeyCode::Char('q') => {
                    app.active_modal = crate::models::ActiveModal::None;
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    if !app.workspace_history.is_empty() && app.selected_workspace_index > 0 {
                        app.selected_workspace_index -= 1;
                    }
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    if !app.workspace_history.is_empty()
                        && app.selected_workspace_index < app.workspace_history.len() - 1
                    {
                        app.selected_workspace_index += 1;
                    }
                }
                KeyCode::Enter => {
                    if !app.workspace_history.is_empty() {
                        let selected_path =
                            app.workspace_history[app.selected_workspace_index].clone();
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
                        } else {
                            app.status_message = if is_vi {
                                "❌ Lỗi: Không thể truy cập thư mục này.".to_string()
                            } else {
                                "❌ Error: Cannot access this folder.".to_string()
                            };
                        }
                    }
                }
                KeyCode::Char('n') | KeyCode::Char('N') => {
                    let dialog_title = if is_vi {
                        "Chọn thư mục Project mới"
                    } else {
                        "Select New Project Folder"
                    };
                    if let Some(folder) =
                        rfd::FileDialog::new().set_title(dialog_title).pick_folder()
                    {
                        if std::env::set_current_dir(&folder).is_ok() {
                            let folder_str = folder.display().to_string();
                            app.current_dir = folder_str.clone();
                            app.add_to_workspace_history(&folder_str);
                            app.refresh_git_status();
                            app.status_message = if is_vi {
                                "🔄 Đã tải Project mới thành công!".to_string()
                            } else {
                                "🔄 Loaded new Project successfully!".to_string()
                            };
                            app.active_modal = crate::models::ActiveModal::None;
                        } else {
                            app.status_message = if is_vi {
                                "❌ Lỗi: Không thể truy cập thư mục này.".to_string()
                            } else {
                                "❌ Error: Cannot access this folder.".to_string()
                            };
                        }
                    } else {
                        app.status_message = if is_vi {
                            "ℹ️ Đã hủy chọn Project.".to_string()
                        } else {
                            "ℹ️ Project selection cancelled.".to_string()
                        };
                    }
                }
                KeyCode::Char('x') | KeyCode::Char('X') => {
                    if !app.workspace_history.is_empty() {
                        let removed_path =
                            app.workspace_history[app.selected_workspace_index].clone();
                        if removed_path == app.current_dir {
                            app.status_message = if is_vi {
                                "⚠️ Không thể xóa workspace đang hoạt động!".to_string()
                            } else {
                                "⚠️ Cannot remove the currently active workspace!".to_string()
                            };
                        } else {
                            app.remove_from_workspace_history(app.selected_workspace_index);
                            app.status_message = if is_vi {
                                format!("🗑️ Đã xóa khỏi lịch sử: {}", removed_path)
                            } else {
                                format!("🗑️ Removed from history: {}", removed_path)
                            };
                        }
                    }
                }
                _ => {}
            }
        }
        crate::models::ActiveModal::GitLog => match key.code {
            KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('v') | KeyCode::Char('V') => {
                app.active_modal = crate::models::ActiveModal::None;
            }
            KeyCode::Up | KeyCode::Char('k') => {
                if app.selected_log_index > 0 {
                    app.selected_log_index -= 1;
                } else if !app.commit_logs.is_empty() {
                    app.selected_log_index = app.commit_logs.len() - 1;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if !app.commit_logs.is_empty() {
                    if app.selected_log_index < app.commit_logs.len() - 1 {
                        app.selected_log_index += 1;
                    } else {
                        app.selected_log_index = 0;
                    }
                }
            }
            KeyCode::Enter => {
                if !app.commit_logs.is_empty() && app.selected_log_index < app.commit_logs.len() {
                    let hash = app.commit_logs[app.selected_log_index].hash.clone();
                    app.fetch_commit_diff(&hash);
                    app.active_modal = crate::models::ActiveModal::CommitDiff(hash);
                }
            }
            _ => {}
        },
        crate::models::ActiveModal::CommitDiff(_) => match key.code {
            KeyCode::Esc | KeyCode::Char('q') => {
                app.active_modal = crate::models::ActiveModal::GitLog;
            }
            KeyCode::Up | KeyCode::Char('k') => {
                app.commit_diff_scroll = app.commit_diff_scroll.saturating_sub(3);
            }
            KeyCode::Down | KeyCode::Char('j') => {
                app.commit_diff_scroll += 3;
            }
            KeyCode::PageUp => {
                app.commit_diff_scroll = app.commit_diff_scroll.saturating_sub(15);
            }
            KeyCode::PageDown => {
                app.commit_diff_scroll += 15;
            }
            _ => {}
        },
        crate::models::ActiveModal::BranchSelect => {
            let _branches_len = app.branches.len();
            match key.code {
                KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('b') | KeyCode::Char('B') => {
                    app.active_modal = crate::models::ActiveModal::None;
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    if app.selected_branch_index > 0 {
                        app.selected_branch_index -= 1;
                    } else if !app.branches.is_empty() {
                        app.selected_branch_index = app.branches.len() - 1;
                    }
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    if !app.branches.is_empty() {
                        if app.selected_branch_index < app.branches.len() - 1 {
                            app.selected_branch_index += 1;
                        } else {
                            app.selected_branch_index = 0;
                        }
                    }
                }
                KeyCode::Char('c') | KeyCode::Char('C') => {
                    app.new_branch_name.clear();
                    app.active_modal = crate::models::ActiveModal::NewBranchInput;
                }
                KeyCode::Char('d')
                | KeyCode::Char('D')
                | KeyCode::Char('x')
                | KeyCode::Char('X') => {
                    if !app.branches.is_empty() && app.selected_branch_index < app.branches.len() {
                        let branch = &app.branches[app.selected_branch_index];
                        if branch.name == app.current_branch && !branch.is_remote {
                            app.status_message = if app.current_lang == "vi" {
                                "❌ Không thể xóa chi nhánh đang hoạt động!".to_string()
                            } else {
                                "❌ Cannot delete the active branch!".to_string()
                            };
                        } else {
                            app.active_modal = crate::models::ActiveModal::BranchDeleteConfirm(
                                branch.name.clone(),
                            );
                        }
                    }
                }
                KeyCode::Char('m') | KeyCode::Char('M') => {
                    if !app.branches.is_empty() && app.selected_branch_index < app.branches.len() {
                        let branch_name = app.branches[app.selected_branch_index].name.clone();
                        if branch_name != app.current_branch {
                            app.active_modal =
                                crate::models::ActiveModal::MergeConfirm(branch_name);
                        } else {
                            app.status_message = if app.current_lang == "vi" {
                                "❌ Không thể merge chi nhánh hiện tại vào chính nó!".to_string()
                            } else {
                                "❌ Cannot merge current branch into itself!".to_string()
                            };
                        }
                    }
                }
                KeyCode::Enter => {
                    if !app.branches.is_empty() && app.selected_branch_index < app.branches.len() {
                        let branch_name = app.branches[app.selected_branch_index].name.clone();
                        match crate::git::branch::checkout_branch(&branch_name) {
                            Ok(_) => {
                                app.status_message = if app.current_lang == "vi" {
                                    format!("🌿 Đã chuyển sang chi nhánh: {}", branch_name)
                                } else {
                                    format!("🌿 Checked out branch: {}", branch_name)
                                };
                            }
                            Err(err) => {
                                app.status_message = if app.current_lang == "vi" {
                                    format!("❌ Lỗi chuyển chi nhánh: {}", err)
                                } else {
                                    format!("❌ Checkout failed: {}", err)
                                };
                            }
                        }
                        app.active_modal = crate::models::ActiveModal::None;
                        app.refresh_git_status();
                    }
                }
                _ => {}
            }
        }
        crate::models::ActiveModal::MergeConfirm(ref branch_name) => {
            let bname = branch_name.clone();
            match key.code {
                KeyCode::Esc | KeyCode::Char('n') | KeyCode::Char('N') => {
                    app.active_modal = crate::models::ActiveModal::BranchSelect;
                }
                KeyCode::Enter | KeyCode::Char('y') | KeyCode::Char('Y') => {
                    app.status_message = if app.current_lang == "vi" {
                        format!("⚡ Đang merge chi nhánh {}...", bname)
                    } else {
                        format!("⚡ Merging branch {}...", bname)
                    };
                    match crate::git::branch::git_merge(&bname) {
                        Ok(out) => {
                            app.status_message = if app.current_lang == "vi" {
                                format!("✅ Đã merge thành công: {}", out)
                            } else {
                                format!("✅ Merge successful: {}", out)
                            };
                        }
                        Err(err) => {
                            app.status_message = if app.current_lang == "vi" {
                                format!("❌ Lỗi merge: {}", err)
                            } else {
                                format!("❌ Merge failed: {}", err)
                            };
                        }
                    }
                    app.active_modal = crate::models::ActiveModal::None;
                    app.refresh_git_status();
                }
                _ => {}
            }
        }
        crate::models::ActiveModal::BranchDeleteConfirm(ref branch_name) => {
            let bname = branch_name.clone();
            match key.code {
                KeyCode::Esc | KeyCode::Char('n') | KeyCode::Char('N') => {
                    app.active_modal = crate::models::ActiveModal::BranchSelect;
                }
                KeyCode::Enter | KeyCode::Char('y') | KeyCode::Char('Y') => {
                    app.status_message = if app.current_lang == "vi" {
                        format!("🗑️ Đang xóa chi nhánh {}...", bname)
                    } else {
                        format!("🗑️ Deleting branch {}...", bname)
                    };
                    match crate::git::branch::delete_branch(
                        &bname,
                        crate::git::branch::DeleteBranchOptions::default(),
                    ) {
                        Ok(out) => {
                            app.status_message = if app.current_lang == "vi" {
                                format!("✅ Đã xóa thành công: {}", out)
                            } else {
                                format!("✅ Delete successful: {}", out)
                            };
                        }
                        Err(err) => {
                            app.status_message = if app.current_lang == "vi" {
                                format!("❌ Lỗi xóa chi nhánh: {}", err)
                            } else {
                                format!("❌ Delete failed: {}", err)
                            };
                        }
                    }
                    app.fetch_branches();
                    if app.selected_branch_index >= app.branches.len() {
                        app.selected_branch_index = app.branches.len().saturating_sub(1);
                    }
                    app.active_modal = crate::models::ActiveModal::BranchSelect;
                    app.refresh_git_status();
                }
                _ => {}
            }
        }
        crate::models::ActiveModal::KiloModelSelect => {
            let filtered: Vec<String> = if app.kilo_model_filter.is_empty() {
                app.kilo_models.clone()
            } else {
                let f = app.kilo_model_filter.to_lowercase();
                app.kilo_models
                    .iter()
                    .filter(|m| m.to_lowercase().contains(&f))
                    .cloned()
                    .collect()
            };

            if !filtered.is_empty() && app.selected_kilo_model_index >= filtered.len() {
                app.selected_kilo_model_index = filtered.len() - 1;
            }

            match key.code {
                KeyCode::Esc | KeyCode::Char('q') => {
                    if app.kilo_model_search_mode || !app.kilo_model_filter.is_empty() {
                        app.kilo_model_filter.clear();
                        app.kilo_model_search_mode = false;
                        app.selected_kilo_model_index = 0;
                    } else {
                        app.active_modal = crate::models::ActiveModal::DiffResult;
                    }
                }
                KeyCode::Enter => {
                    if !filtered.is_empty() {
                        let idx = app.selected_kilo_model_index.min(filtered.len() - 1);
                        app.current_kilo_model = filtered[idx].clone();
                    }
                    app.kilo_model_filter.clear();
                    app.kilo_model_search_mode = false;
                    app.active_modal = crate::models::ActiveModal::DiffResult;
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    if app.selected_kilo_model_index > 0 {
                        app.selected_kilo_model_index -= 1;
                    }
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    if !filtered.is_empty() && app.selected_kilo_model_index + 1 < filtered.len() {
                        app.selected_kilo_model_index += 1;
                    }
                }
                KeyCode::Char('/') => {
                    app.kilo_model_search_mode = true;
                }
                KeyCode::Backspace => {
                    if app.kilo_model_search_mode && !app.kilo_model_filter.is_empty() {
                        app.kilo_model_filter.pop();
                        app.selected_kilo_model_index = 0;
                    }
                }
                KeyCode::Char(c) => {
                    if c.is_ascii_alphanumeric() || c == '-' || c == '.' {
                        if !app.kilo_model_search_mode {
                            app.kilo_model_search_mode = true;
                        }
                        app.kilo_model_filter.push(c);
                        app.selected_kilo_model_index = 0;
                    }
                }
                _ => {}
            }
        }
        crate::models::ActiveModal::RemoteInfo => match key.code {
            KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('i') | KeyCode::Enter => {
                app.active_modal = crate::models::ActiveModal::None;
            }
            _ => {}
        },
        crate::models::ActiveModal::ViewPrompt => match key.code {
            KeyCode::Esc
            | KeyCode::Char('q')
            | KeyCode::Char('x')
            | KeyCode::Char('X')
            | KeyCode::Enter => {
                app.active_modal = crate::models::ActiveModal::None;
            }
            _ => {}
        },
        crate::models::ActiveModal::CommitTree => match key.code {
            KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('t') | KeyCode::Char('T') => {
                app.active_modal = crate::models::ActiveModal::None;
            }
            KeyCode::Up | KeyCode::Char('k') => {
                if app.selected_log_index > 0 {
                    app.selected_log_index -= 1;
                } else if !app.commit_logs.is_empty() {
                    app.selected_log_index = app.commit_logs.len() - 1;
                }
                if !app.commit_logs.is_empty() {
                    let hash = app.commit_logs[app.selected_log_index].hash.clone();
                    app.fetch_commit_diff(&hash);
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if !app.commit_logs.is_empty() {
                    app.selected_log_index = (app.selected_log_index + 1) % app.commit_logs.len();
                }
                if !app.commit_logs.is_empty() {
                    let hash = app.commit_logs[app.selected_log_index].hash.clone();
                    app.fetch_commit_diff(&hash);
                }
            }
            _ => {}
        },
        crate::models::ActiveModal::FeatureCommit => {
            match key.code {
                KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('e') | KeyCode::Char('E') => {
                    app.active_modal = crate::models::ActiveModal::None;
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    if app.selected_feature_index > 0 {
                        app.selected_feature_index -= 1;
                    } else if !app.feature_groups.is_empty() {
                        app.selected_feature_index = app.feature_groups.len() - 1;
                    }
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    if !app.feature_groups.is_empty() {
                        if app.selected_feature_index + 1 < app.feature_groups.len() {
                            app.selected_feature_index += 1;
                        } else {
                            app.selected_feature_index = 0;
                        }
                    }
                }
                KeyCode::Enter => {
                    if !app.feature_groups.is_empty()
                        && app.selected_feature_index < app.feature_groups.len()
                    {
                        let group = &app.feature_groups[app.selected_feature_index];
                        let files_to_stage = group.files.clone();
                        let feature_name = group.name.clone();
                        let file_count = files_to_stage.len();

                        let _ = crate::git::status::unstage_all();

                        for path in &files_to_stage {
                            let _ = crate::git::status::stage_file(path);
                        }

                        app.refresh_git_status();

                        app.status_message = if app.current_lang == "vi" {
                            format!("✅ Đã stage feature '{}': {} file(s). Nhấn [g] hoặc [K] để commit.", feature_name, file_count)
                        } else {
                            format!(
                                "✅ Staged feature '{}': {} file(s). Press [g] hoặc [K] để commit.",
                                feature_name, file_count
                            )
                        };

                        app.active_modal = crate::models::ActiveModal::None;
                    }
                }
                _ => {}
            }
        }
        crate::models::ActiveModal::GithubQuickView { .. } => {
            if app.github_quickview_searching {
                match key.code {
                    KeyCode::Esc | KeyCode::Enter => {
                        app.github_quickview_searching = false;
                    }
                    KeyCode::Backspace => {
                        app.github_quickview_search.pop();
                    }
                    KeyCode::Char(c) => {
                        app.github_quickview_search.push(c);
                    }
                    _ => {}
                }
            } else {
                match key.code {
                    KeyCode::Esc | KeyCode::Enter => {
                        app.github_quickview_search.clear();
                        app.active_modal = crate::models::ActiveModal::GithubDownloadTree;
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        app.github_quickview_scroll = app.github_quickview_scroll.saturating_sub(1);
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        app.github_quickview_scroll = app.github_quickview_scroll.saturating_add(1);
                    }
                    KeyCode::PageUp => {
                        app.github_quickview_scroll =
                            app.github_quickview_scroll.saturating_sub(20);
                    }
                    KeyCode::PageDown => {
                        app.github_quickview_scroll =
                            app.github_quickview_scroll.saturating_add(20);
                    }
                    KeyCode::Char('/') => {
                        app.github_quickview_searching = true;
                        app.github_quickview_search.clear();
                    }
                    KeyCode::Char('c') | KeyCode::Char('C') => {
                        app.github_quickview_search.clear();
                    }
                    _ => {}
                }
            }
        }
        crate::models::ActiveModal::GithubBranchSelect => match key.code {
            KeyCode::Esc | KeyCode::Char('q') => {
                app.active_modal = crate::models::ActiveModal::GithubDownloadTree;
            }
            KeyCode::Up | KeyCode::Char('k') => {
                if !app.github_branches.is_empty() {
                    if app.selected_github_branch_index > 0 {
                        app.selected_github_branch_index -= 1;
                    } else {
                        app.selected_github_branch_index = app.github_branches.len() - 1;
                    }
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if !app.github_branches.is_empty() {
                    if app.selected_github_branch_index < app.github_branches.len() - 1 {
                        app.selected_github_branch_index += 1;
                    } else {
                        app.selected_github_branch_index = 0;
                    }
                }
            }
            KeyCode::Enter => {
                if !app.github_branches.is_empty()
                    && app.selected_github_branch_index < app.github_branches.len()
                {
                    let selected_branch =
                        app.github_branches[app.selected_github_branch_index].clone();
                    app.status_message = if app.current_lang == "vi" {
                        format!("⏳ Đang chuyển sang nhánh {}...", selected_branch)
                    } else {
                        format!("⏳ Switching to branch {}...", selected_branch)
                    };
                    if let Some(ref dir) = app.github_temp_dir {
                        let fetch_out = Command::new("git")
                            .args(["fetch", "--depth", "1", "origin", &selected_branch])
                            .current_dir(dir.path())
                            .output();
                        if let Ok(out) = fetch_out {
                            if out.status.success() {
                                let checkout_out = Command::new("git")
                                    .args(["checkout", "FETCH_HEAD"])
                                    .current_dir(dir.path())
                                    .output();
                                if let Ok(c_out) = checkout_out {
                                    if c_out.status.success() {
                                        if let Err(e) = app.visit_repo_dir() {
                                            let _ = e;
                                            app.status_message = format!("❌ Lỗi: {}", e);
                                        } else {
                                            app.current_github_branch = selected_branch;
                                            app.selected_github_tree_index = 0;
                                            app.active_modal =
                                                crate::models::ActiveModal::GithubDownloadTree;
                                            app.status_message = if app.current_lang == "vi" {
                                                "✅ Đã chuyển nhánh thành công".to_string()
                                            } else {
                                                "✅ Successfully switched branch".to_string()
                                            };
                                        }
                                    } else {
                                        app.status_message = if app.current_lang == "vi" {
                                            "❌ Lỗi checkout chi nhánh".to_string()
                                        } else {
                                            "❌ Error checking out branch".to_string()
                                        };
                                    }
                                } else {
                                    app.status_message = if app.current_lang == "vi" {
                                        "❌ Không thể chạy lệnh checkout".to_string()
                                    } else {
                                        "❌ Cannot execute checkout command".to_string()
                                    };
                                }
                            } else {
                                let stderr =
                                    String::from_utf8_lossy(&out.stderr).trim().to_string();
                                app.status_message = format!("❌ Fetch failed: {}", stderr);
                            }
                        } else {
                            app.status_message = if app.current_lang == "vi" {
                                "❌ Không thể chạy lệnh fetch".to_string()
                            } else {
                                "❌ Cannot execute fetch command".to_string()
                            };
                        }
                    }
                }
            }
            _ => {}
        },
        _ => {}
    }
}

fn apply_theme(app: &mut App, index: usize, t_info: &crate::theme::ThemeInfo) {
    app.theme_id = t_info.id.to_string();
    app.is_light_theme = t_info.id == "light";
    app.selected_theme_index = index;
    let _ = Command::new("git")
        .args(["config", "--global", "git-ai.theme", t_info.id])
        .output();
    let label = if app.current_lang == "vi" {
        t_info.name_vi
    } else {
        t_info.name_en
    };
    app.status_message = if app.current_lang == "vi" {
        format!("🎨 Đã chuyển sang giao diện {}", label)
    } else {
        format!("🎨 Switched to {} theme", label)
    };
    app.active_modal = crate::models::ActiveModal::None;
}
