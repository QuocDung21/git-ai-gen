use crossterm::event::{KeyCode, KeyEvent};
use rust_i18n::t;

use crate::app::App;

pub fn handle_diff_result(app: &mut App, key: &KeyEvent) {
    match key.code {
        KeyCode::Up => {
            app.diff_snapshot_scroll = app.diff_snapshot_scroll.saturating_sub(1);
            return;
        }
        KeyCode::Down => {
            app.diff_snapshot_scroll = app.diff_snapshot_scroll.saturating_add(1);
            return;
        }
        KeyCode::PageUp => {
            app.diff_snapshot_scroll = app.diff_snapshot_scroll.saturating_sub(10);
            return;
        }
        KeyCode::PageDown => {
            app.diff_snapshot_scroll = app.diff_snapshot_scroll.saturating_add(10);
            return;
        }
        _ => {}
    }

    if !app.diff_kilo_generated.is_empty() {
        match key.code {
            KeyCode::Char('c') | KeyCode::Char('C') => {
                if let Ok(mut cb) = arboard::Clipboard::new() {
                    let _ = cb.set_text(app.diff_kilo_generated.clone());
                }
                app.status_message = t!("diff_kilo_clipboard_copied").to_string();
            }
            KeyCode::Enter | KeyCode::Char('g') | KeyCode::Char('G') => {
                app.commit_message_preview = app.diff_kilo_generated.clone();
                app.active_modal = crate::models::ActiveModal::None;
                app.status_message = t!("diff_kilo_using_message").to_string();
            }
            KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('d') => {
                app.diff_kilo_generated.clear();
                app.active_modal = crate::models::ActiveModal::None;
            }
            _ => {}
        }
    } else {
        match key.code {
            KeyCode::Char('k') | KeyCode::Char('K') => {
                if !app.kilo_ai_enabled {
                    app.kilo_generation_status = t!("kilo_disabled").to_string();
                } else if app.last_staged_diff.trim().is_empty() {
                    app.kilo_generation_status = t!("kilo_no_staged_diff").to_string();
                } else {
                    app.kilo_generating = true;
                        app.kilo_generation_status = t!("kilo_asking").to_string();
                    app.diff_kilo_generated.clear();
                }
            }
            KeyCode::Char('m') | KeyCode::Char('M') => {
                app.status_message = t!("kilo_fetching_models").to_string();
                app.fetch_kilo_models();
                app.kilo_model_filter.clear();
                app.kilo_model_search_mode = false;
                app.selected_kilo_model_index = 0;
                if !app.current_kilo_model.is_empty() {
                    if let Some(idx) = app
                        .kilo_models
                        .iter()
                        .position(|m| m == &app.current_kilo_model)
                    {
                        app.selected_kilo_model_index = idx;
                    }
                }
                app.active_modal = crate::models::ActiveModal::KiloModelSelect;
            }
            KeyCode::Esc | KeyCode::Enter | KeyCode::Char('q') | KeyCode::Char('d') => {
                app.active_modal = crate::models::ActiveModal::None;
            }
            _ => {}
        }
    }
}

/// Handle Kilo AI generation in the pre-poll phase (long-running).
/// Returns true if generation was in progress.
pub fn handle_kilo_generation(app: &mut App) -> bool {
    if app.active_modal != crate::models::ActiveModal::DiffResult || !app.kilo_generating {
        return false;
    }
    let diff = app.last_staged_diff.clone();
    if diff.trim().is_empty() {
        app.kilo_generation_status = t!("kilo_no_staged_diff").to_string();
        app.kilo_generating = false;
    } else {
        match app.try_generate_with_kilo(&diff) {
            Ok(msg) => {
                app.diff_kilo_generated = msg;
                app.kilo_generation_status = t!("kilo_finished").to_string();
            }
            Err(e) => {
                app.kilo_generation_status = format!("❌ {}", e);
            }
        }
        app.kilo_generating = false;
    }
    true
}
