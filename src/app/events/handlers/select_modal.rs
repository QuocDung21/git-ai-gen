#![allow(clippy::collapsible_match)]

use std::process::Command;

use crossterm::event::{KeyCode, KeyEvent};
use rust_i18n::t;

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
                    app.refresh_locales();
                }
                app.active_modal = crate::models::ActiveModal::None;
            }
            KeyCode::Char('e') | KeyCode::Char('E') => {
                let locales = crate::cli::Locales::new(&app.current_lang);
                if let Ok(msg) = crate::cli::system::handle_lang("en", &locales) {
                    app.status_message = msg;
                    app.current_lang = crate::helper::Helper::get_ai_language();
                    app.refresh_locales();
                }
                app.active_modal = crate::models::ActiveModal::None;
            }
            KeyCode::Char('a') | KeyCode::Char('A') => {
                let locales = crate::cli::Locales::new(&app.current_lang);
                if let Ok(msg) = crate::cli::system::handle_lang("auto", &locales) {
                    app.status_message = msg;
                    app.current_lang = crate::helper::Helper::get_ai_language();
                    app.refresh_locales();
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
                    app.refresh_locales();
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
        crate::models::ActiveModal::Settings => match key.code {
            KeyCode::Esc | KeyCode::Char('q') => {
                app.active_modal = crate::models::ActiveModal::None;
            }
            KeyCode::Up | KeyCode::Char('k') => {
                if app.selected_setting_index > 0 {
                    app.selected_setting_index -= 1;
                } else {
                    app.selected_setting_index = 4;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if app.selected_setting_index < 4 {
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
                    let state = if app.auto_push {
                        t!("select_on")
                    } else {
                        t!("select_off")
                    };
                    app.status_message =
                        t!("select_settings_auto_push", state = state.as_ref()).to_string();
                }
                1 => {
                    app.auto_stage_all = !app.auto_stage_all;
                    let val = app.auto_stage_all.to_string();
                    let _ = Command::new("git")
                        .args(["config", "--global", "git-ai.auto-stage-all", &val])
                        .output();
                    let state = if app.auto_stage_all {
                        t!("select_on")
                    } else {
                        t!("select_off")
                    };
                    app.status_message =
                        t!("select_settings_auto_stage", state = state.as_ref()).to_string();
                }
                2 => {
                    app.kilo_ai_enabled = !app.kilo_ai_enabled;
                    let val = app.kilo_ai_enabled.to_string();
                    let _ = Command::new("git")
                        .args(["config", "--global", "git-ai.kilo-ai", &val])
                        .output();
                    let state = if app.kilo_ai_enabled {
                        t!("select_on")
                    } else {
                        t!("select_off")
                    };
                    app.status_message =
                        t!("select_settings_kilo_ai", state = state.as_ref()).to_string();
                }
                3 => {
                    app.splash_enabled = !app.splash_enabled;
                    let val = app.splash_enabled.to_string();
                    let _ = Command::new("git")
                        .args(["config", "--global", "git-ai.splash", &val])
                        .output();
                    let state = if app.splash_enabled {
                        t!("select_on")
                    } else {
                        t!("select_off")
                    };
                    app.status_message =
                        t!("select_settings_splash", state = state.as_ref()).to_string();
                }
                4 => {
                    app.active_modal = crate::models::ActiveModal::EditorSelect;
                    app.selected_editor_index = match app.editor.as_str() {
                        "code" => 0,
                        "cursor" => 1,
                        "zed" => 2,
                        "subl" => 3,
                        _ => 4,
                    };
                }
                _ => {}
            },
            _ => {}
        },
        crate::models::ActiveModal::EditorSelect => match key.code {
            KeyCode::Esc | KeyCode::Char('q') => {
                app.active_modal = crate::models::ActiveModal::Settings;
            }
            KeyCode::Up | KeyCode::Char('k') => {
                if app.selected_editor_index > 0 {
                    app.selected_editor_index -= 1;
                } else {
                    app.selected_editor_index = 4;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if app.selected_editor_index < 4 {
                    app.selected_editor_index += 1;
                } else {
                    app.selected_editor_index = 0;
                }
            }
            KeyCode::Char('1') => {
                app.selected_editor_index = 0;
            }
            KeyCode::Char('2') => {
                app.selected_editor_index = 1;
            }
            KeyCode::Char('3') => {
                app.selected_editor_index = 2;
            }
            KeyCode::Char('4') => {
                app.selected_editor_index = 3;
            }
            KeyCode::Char('5') => {
                app.selected_editor_index = 4;
            }
            KeyCode::Char(' ') | KeyCode::Enter => {
                let selection = match app.selected_editor_index {
                    0 => "code",
                    1 => "cursor",
                    2 => "zed",
                    3 => "subl",
                    _ => crate::app::DEFAULT_OPEN_CMD,
                };
                app.editor = selection.to_string();
                let _ = Command::new("git")
                    .args(["config", "--global", "git-ai.editor", selection])
                    .output();

                let friendly_name = match selection {
                    "code" => "VS Code",
                    "cursor" => "Cursor",
                    "zed" => "Zed",
                    "subl" => "Sublime Text",
                    _ => "System Default",
                };

                app.status_message = t!("select_settings_editor", name = friendly_name).to_string();
                app.active_modal = crate::models::ActiveModal::Settings;
            }
            _ => {}
        },
        crate::models::ActiveModal::WorkspaceHistory => match key.code {
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
                    let selected_path = app.workspace_history[app.selected_workspace_index].clone();
                    if std::env::set_current_dir(&selected_path).is_ok() {
                        app.current_dir = selected_path.clone();
                        app.add_to_workspace_history(&selected_path);
                        app.refresh_git_status();
                        app.status_message =
                            t!("select_workspace_ok", path = selected_path.clone()).to_string();
                        app.active_modal = crate::models::ActiveModal::None;
                    } else {
                        app.status_message = t!("select_workspace_err").to_string();
                    }
                }
            }
            KeyCode::Char('n') | KeyCode::Char('N') => {
                #[cfg(target_os = "linux")]
                let is_headless =
                    std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err();
                #[cfg(not(target_os = "linux"))]
                let is_headless = false;

                if is_headless {
                    app.workspace_path_input.clear();
                    app.active_modal = crate::models::ActiveModal::WorkspacePathInput;
                } else {
                    let dialog_title = t!("select_workspace_new_folder");
                    if let Some(folder) = rfd::FileDialog::new()
                        .set_title(dialog_title.as_ref())
                        .pick_folder()
                    {
                        if std::env::set_current_dir(&folder).is_ok() {
                            let folder_str = folder.display().to_string();
                            app.current_dir = folder_str.clone();
                            app.add_to_workspace_history(&folder_str);
                            app.refresh_git_status();
                            app.status_message = t!("select_workspace_new_ok").to_string();
                            app.active_modal = crate::models::ActiveModal::None;
                        } else {
                            app.status_message = t!("select_workspace_err").to_string();
                        }
                    } else {
                        app.status_message = t!("select_workspace_cancel").to_string();
                    }
                }
            }
            KeyCode::Char('p') | KeyCode::Char('P') => {
                app.workspace_path_input.clear();
                app.active_modal = crate::models::ActiveModal::WorkspacePathInput;
            }
            KeyCode::Char('x') | KeyCode::Char('X') => {
                if !app.workspace_history.is_empty() {
                    let removed_path = app.workspace_history[app.selected_workspace_index].clone();
                    if removed_path == app.current_dir {
                        app.status_message = t!("select_workspace_active_err").to_string();
                    } else {
                        app.remove_from_workspace_history(app.selected_workspace_index);
                        app.status_message =
                            t!("select_workspace_removed", path = removed_path.clone()).to_string();
                    }
                }
            }
            _ => {}
        },
        crate::models::ActiveModal::ProjectLanguages => match key.code {
            KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => {
                app.active_modal = crate::models::ActiveModal::None;
            }
            _ => {}
        },
        crate::models::ActiveModal::HandleTest => {
            match key.code {
                KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => {
                    app.active_modal = crate::models::ActiveModal::None;
                }
                KeyCode::Char('1') => {
                    app.is_light_theme = !app.is_light_theme;
                    app.theme_id = if app.is_light_theme {
                        "light".to_string()
                    } else {
                        "vscode".to_string()
                    };
                }
                KeyCode::Char('2') => {
                    app.status_message = app
                        .tr(
                            "🔔 [DEV]: Đã kích hoạt cảnh báo thử nghiệm thành công!",
                            "🔔 [DEV]: Mock alert status triggered successfully!",
                        )
                        .to_string();
                }
                KeyCode::Char('3') => {
                    app.language_stats.clear();
                    app.status_message = app
                        .tr(
                            "🧹 [DEV]: Đã dọn dẹp dữ liệu thống kê ngôn ngữ!",
                            "🧹 [DEV]: Cleared language stats data successfully!",
                        )
                        .to_string();
                }
                KeyCode::Char('4') => {
                    app.status_message = app.tr(
                    "⚡ [DEV]: Đã quét thử nghiệm mã nguồn, đã tối ưu hóa tệp tin lockfile!",
                    "⚡ [DEV]: Executed mock diff scan, lockfiles successfully optimized!"
                ).to_string();
                }
                KeyCode::Char('5') => {
                    let history = crate::helper::Helper::load_history_file("workspace_history.txt");
                    app.status_message = format!("JSON: {:?}", history);
                }
                KeyCode::Char('6') => {
                    app.has_conflicts = !app.has_conflicts;
                    app.conflict_count = if app.has_conflicts { 3 } else { 0 };
                    app.status_message = app
                        .tr(
                            "⚡ [DEV]: Đã bật/tắt cảnh báo xung đột (3 xung đột)!",
                            "⚡ [DEV]: Toggled mock merge conflicts (3 conflicts)!",
                        )
                        .to_string();
                }
                KeyCode::Char('7') => {
                    app.current_lang = if app.current_lang == "vi" {
                        "en".to_string()
                    } else {
                        "vi".to_string()
                    };
                    app.refresh_locales();
                    app.status_message = app
                        .tr(
                            "🌐 [DEV]: Đã chuyển ngôn ngữ giao diện sang Tiếng Việt!",
                            "🌐 [DEV]: Switched interface language to English!",
                        )
                        .to_string();
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
                            app.status_message = t!("select_branch_delete_err").to_string();
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
                            app.status_message = t!("select_branch_merge_self_err").to_string();
                        }
                    }
                }
                KeyCode::Enter => {
                    if !app.branches.is_empty() && app.selected_branch_index < app.branches.len() {
                        let branch_name = app.branches[app.selected_branch_index].name.clone();
                        match crate::git::branch::checkout_branch(&branch_name) {
                            Ok(_) => {
                                app.status_message =
                                    t!("select_branch_checkout_ok", name = branch_name.clone())
                                        .to_string();
                            }
                            Err(err) => {
                                app.status_message =
                                    t!("select_branch_checkout_err", err = err.to_string())
                                        .to_string();
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
                    app.status_message =
                        t!("select_branch_merging", name = bname.clone()).to_string();
                    match crate::git::branch::git_merge(&bname) {
                        Ok(out) => {
                            app.status_message =
                                t!("select_branch_merge_ok", out = out.clone()).to_string();
                        }
                        Err(err) => {
                            app.status_message =
                                t!("select_branch_merge_err", err = err.to_string()).to_string();
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
                    app.status_message =
                        t!("select_branch_deleting", name = bname.clone()).to_string();
                    match crate::git::branch::delete_branch(
                        &bname,
                        crate::git::branch::DeleteBranchOptions::default(),
                    ) {
                        Ok(out) => {
                            app.status_message =
                                t!("select_branch_delete_ok", out = out.clone()).to_string();
                        }
                        Err(err) => {
                            app.status_message =
                                t!("select_branch_delete_fail", err = err.to_string()).to_string();
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
        crate::models::ActiveModal::FeatureCommit => match key.code {
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

                    app.status_message = t!(
                        "feature_staged",
                        name = feature_name.clone(),
                        count = file_count
                    )
                    .to_string();

                    app.active_modal = crate::models::ActiveModal::None;
                }
            }
            _ => {}
        },
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
                    app.status_message =
                        t!("github_branch_switching", name = selected_branch.clone()).to_string();
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
                                            app.status_message = t!("github_branch_ok").to_string();
                                        }
                                    } else {
                                        app.status_message =
                                            t!("github_branch_checkout_err").to_string();
                                    }
                                } else {
                                    app.status_message =
                                        t!("github_branch_checkout_cmd_err").to_string();
                                }
                            } else {
                                let stderr =
                                    String::from_utf8_lossy(&out.stderr).trim().to_string();
                                app.status_message = format!("❌ Fetch failed: {}", stderr);
                            }
                        } else {
                            app.status_message = t!("github_branch_fetch_cmd_err").to_string();
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
    let label = if app.current_lang == "vi" { t_info.name_vi } else { t_info.name_en };
    app.status_message = t!("theme_switched", name = label).to_string();
    app.active_modal = crate::models::ActiveModal::None;
}
