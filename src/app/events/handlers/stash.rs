use crossterm::event::{KeyCode, KeyEvent};

use crate::app::App;
use crate::models::{StashAction, StashStep};

pub fn handle_stash(app: &mut App, key: &KeyEvent) {
    match &app.stash_step.clone() {
        StashStep::List => match key.code {
            KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('s') => {
                app.active_modal = crate::models::ActiveModal::None;
            }
            KeyCode::Up | KeyCode::Char('k') => {
                if app.selected_stash_index > 0 {
                    app.selected_stash_index -= 1;
                } else if !app.stash_entries.is_empty() {
                    app.selected_stash_index = app.stash_entries.len() - 1;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if !app.stash_entries.is_empty() {
                    if app.selected_stash_index < app.stash_entries.len() - 1 {
                        app.selected_stash_index += 1;
                    } else {
                        app.selected_stash_index = 0;
                    }
                }
            }
            KeyCode::Char('n') | KeyCode::Char('N') => {
                app.status_message = match crate::git::stash::stash_push() {
                    Ok(_) => {
                        if app.current_lang == "vi" {
                            "✅ Đã stash thay đổi!".to_string()
                        } else {
                            "✅ Changes stashed!".to_string()
                        }
                    }
                    Err(_) => "❌ Stash failed.".to_string(),
                };
                app.fetch_stash();
                app.refresh_git_status();
            }
            KeyCode::Enter | KeyCode::Char('p') => {
                if !app.stash_entries.is_empty() {
                    app.stash_step = StashStep::Confirm(app.selected_stash_index, StashAction::Pop);
                }
            }
            KeyCode::Char('a') => {
                if !app.stash_entries.is_empty() {
                    app.stash_step =
                        StashStep::Confirm(app.selected_stash_index, StashAction::Apply);
                }
            }
            KeyCode::Char('x') | KeyCode::Delete => {
                if !app.stash_entries.is_empty() {
                    app.stash_step =
                        StashStep::Confirm(app.selected_stash_index, StashAction::Drop);
                }
            }
            _ => {}
        },
        StashStep::Confirm(idx, action) => {
            let idx = *idx;
            let action = action.clone();
            match key.code {
                KeyCode::Char('y') | KeyCode::Char('Y') | KeyCode::Enter => {
                    let ref_str = format!("stash@{{{}}}", idx);
                    let result = match action {
                        StashAction::Pop => crate::git::stash::stash_pop(&ref_str),
                        StashAction::Apply => crate::git::stash::stash_apply(&ref_str),
                        StashAction::Drop => crate::git::stash::stash_drop(&ref_str),
                    };
                    let is_vi = app.current_lang == "vi";
                    app.status_message = match result {
                        Ok(_) => match action {
                            StashAction::Pop => {
                                if is_vi {
                                    "✅ Đã pop stash!".to_string()
                                } else {
                                    "✅ Stash popped!".to_string()
                                }
                            }
                            StashAction::Apply => {
                                if is_vi {
                                    "✅ Đã apply stash!".to_string()
                                } else {
                                    "✅ Stash applied!".to_string()
                                }
                            }
                            StashAction::Drop => {
                                if is_vi {
                                    "🗑️ Đã xóa stash!".to_string()
                                } else {
                                    "🗑️ Stash dropped!".to_string()
                                }
                            }
                        },
                        Err(_) => "❌ Stash operation failed.".to_string(),
                    };
                    app.fetch_stash();
                    app.refresh_git_status();
                    app.active_modal = crate::models::ActiveModal::None;
                }
                KeyCode::Esc | KeyCode::Char('n') => {
                    app.stash_step = StashStep::List;
                }
                _ => {}
            }
        }
    }
}
