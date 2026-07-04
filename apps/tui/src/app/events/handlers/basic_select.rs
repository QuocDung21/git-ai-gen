use std::process::Command;

use crossterm::event::{KeyCode, KeyEvent};
use rust_i18n::t;

use crate::app::App;
use crate::models::ActiveModal;

pub(super) fn handle_help(app: &mut App, key: &KeyEvent) {
    match key.code {
        KeyCode::Esc
        | KeyCode::Char(' ')
        | KeyCode::Enter
        | KeyCode::Char('q')
        | KeyCode::Char('?')
        | KeyCode::Char('h') => {
            app.active_modal = ActiveModal::None;
        }
        _ => {}
    }
}

pub(super) fn handle_language_select(app: &mut App, key: &KeyEvent) {
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') => {
            app.active_modal = ActiveModal::None;
        }
        KeyCode::Char('v') | KeyCode::Char('V') => {
            apply_language(app, "vi");
        }
        KeyCode::Char('e') | KeyCode::Char('E') => {
            apply_language(app, "en");
        }
        KeyCode::Char('a') | KeyCode::Char('A') => {
            apply_language(app, "auto");
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
            apply_language(app, selection);
        }
        _ => {}
    }
}

pub(super) fn handle_settings(app: &mut App, key: &KeyEvent) {
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') => {
            app.active_modal = ActiveModal::None;
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
                persist_bool("git-ai.auto-push", app.auto_push);
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
                persist_bool("git-ai.auto-stage-all", app.auto_stage_all);
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
                persist_bool("git-ai.kilo-ai", app.kilo_ai_enabled);
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
                persist_bool("git-ai.splash", app.splash_enabled);
                let state = if app.splash_enabled {
                    t!("select_on")
                } else {
                    t!("select_off")
                };
                app.status_message =
                    t!("select_settings_splash", state = state.as_ref()).to_string();
            }
            4 => {
                app.active_modal = ActiveModal::EditorSelect;
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
    }
}

pub(super) fn handle_editor_select(app: &mut App, key: &KeyEvent) {
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') => {
            app.active_modal = ActiveModal::Settings;
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
            app.active_modal = ActiveModal::Settings;
        }
        _ => {}
    }
}

pub(super) fn handle_project_languages(app: &mut App, key: &KeyEvent) {
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => {
            app.active_modal = ActiveModal::None;
        }
        _ => {}
    }
}

pub(super) fn handle_handle_test(app: &mut App, key: &KeyEvent) {
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => {
            app.active_modal = ActiveModal::None;
        }
        KeyCode::Char('1') => {
            app.is_light_theme = false;
            app.theme_id = "midnight".to_string();
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
            app.status_message = app
                .tr(
                    "⚡ [DEV]: Đã quét thử nghiệm mã nguồn, đã tối ưu hóa tệp tin lockfile!",
                    "⚡ [DEV]: Executed mock diff scan, lockfiles successfully optimized!",
                )
                .to_string();
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

pub(super) fn handle_remote_info(app: &mut App, key: &KeyEvent) {
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('i') | KeyCode::Enter => {
            app.active_modal = ActiveModal::None;
        }
        _ => {}
    }
}

pub(super) fn handle_view_prompt(app: &mut App, key: &KeyEvent) {
    match key.code {
        KeyCode::Esc
        | KeyCode::Char('q')
        | KeyCode::Char('x')
        | KeyCode::Char('X')
        | KeyCode::Enter => {
            app.active_modal = ActiveModal::None;
        }
        _ => {}
    }
}

fn apply_language(app: &mut App, selection: &str) {
    let locales = crate::cli::Locales::new(&app.current_lang);
    if let Ok(msg) = crate::cli::system::handle_lang(selection, &locales) {
        app.status_message = msg;
        app.current_lang = crate::helper::Helper::get_ai_language();
        app.refresh_locales();
    }
    app.active_modal = ActiveModal::None;
}

fn persist_bool(key: &str, value: bool) {
    let val = value.to_string();
    let _ = Command::new("git")
        .args(["config", "--global", key, &val])
        .output();
}
