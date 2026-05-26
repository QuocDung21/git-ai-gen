use crate::app::App;
use crate::models::GoStep;
use crossterm::event::{KeyCode, KeyEvent};
use rust_i18n::t;

pub fn handle_go_confirm(app: &mut App, key: &KeyEvent) {
    match app.go_step.clone() {
        GoStep::Confirm => match key.code {
            KeyCode::Tab => {
                app.commit_input_mode = !app.commit_input_mode;
                if app.commit_input_mode && app.commit_input_text.is_empty() {
                    app.commit_input_text = app.commit_message_preview.clone();
                }
            }
            KeyCode::Enter => {
                if app.staged_count == 0 {
                    app.status_message = t!("go_no_stage").to_string();
                    app.active_modal = crate::models::ActiveModal::None;
                } else {
                    let msg = if app.commit_input_mode {
                        app.commit_input_text.trim().to_string()
                    } else {
                        app.commit_message_preview.trim().to_string()
                    };
                    if !msg.is_empty() {
                        app.commit_message_preview = msg;
                        app.go_step = GoStep::Pushing;
                    }
                }
            }
            KeyCode::Char('y') | KeyCode::Char('Y') if !app.commit_input_mode => {
                if app.staged_count > 0 {
                    let msg = app.commit_message_preview.trim().to_string();
                    if !msg.is_empty() {
                        app.go_step = GoStep::Pushing;
                    }
                }
            }
            KeyCode::Backspace if app.commit_input_mode => {
                app.commit_input_text.pop();
            }
            KeyCode::Char(c) if app.commit_input_mode => {
                app.commit_input_text.push(c);
            }
            KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc if !app.commit_input_mode => {
                app.active_modal = crate::models::ActiveModal::None;
                app.commit_input_mode = false;
            }
            KeyCode::Esc if app.commit_input_mode => {
                app.commit_input_mode = false;
            }
            _ => {}
        },
        GoStep::Done(_) => match key.code {
            KeyCode::Esc | KeyCode::Enter | KeyCode::Char('q') => {
                app.active_modal = crate::models::ActiveModal::None;
                app.go_step = GoStep::Confirm;
                app.refresh_git_status();
            }
            _ => {}
        },
        GoStep::Pushing => {}
    }
}

pub fn handle_go_pushing(app: &mut App) -> bool {
    if app.active_modal != crate::models::ActiveModal::GoConfirm {
        return false;
    }
    if let GoStep::Pushing = &app.go_step {
        let msg = app.commit_message_preview.clone();

        let commit_ok = crate::git::commit::commit(&msg).is_ok();

        if !commit_ok {
            app.go_step = GoStep::Done(t!("go_commit_fail").to_string());
        } else {
            if app.auto_push {
                match crate::git::remote::git_push() {
                    Ok(_) => {
                        app.go_step = GoStep::Done(t!("go_push_ok").to_string());
                    }
                    Err(err) => {
                        app.go_step = GoStep::Done(t!("go_push_fail", err = err.to_string()).to_string());
                    }
                }
            } else {
                app.go_step = GoStep::Done(t!("go_commit_only_ok").to_string());
            }
        }
        return true;
    }
    false
}

pub fn handle_amend_edit(app: &mut App, key: &KeyEvent) {
    use crate::models::AmendStep;
    match app.amend_step.clone() {
        AmendStep::Edit => match key.code {
            KeyCode::Esc | KeyCode::Char('q') => {
                app.active_modal = crate::models::ActiveModal::None;
            }
            KeyCode::Enter => {
                if !app.amend_message.trim().is_empty() {
                    app.amend_step = AmendStep::Pushing;
                }
            }
            KeyCode::Backspace => {
                app.amend_message.pop();
            }
            KeyCode::Char(c) => {
                app.amend_message.push(c);
            }
            _ => {}
        },
        AmendStep::Done(_) => match key.code {
            KeyCode::Esc | KeyCode::Enter | KeyCode::Char('q') => {
                app.active_modal = crate::models::ActiveModal::None;
                app.amend_step = AmendStep::Edit;
                app.refresh_git_status();
            }
            _ => {}
        },
        AmendStep::Pushing => {}
    }
}

pub fn handle_amend_pushing(app: &mut App) -> bool {
    use crate::models::AmendStep;
    if app.active_modal != crate::models::ActiveModal::AmendCommit {
        return false;
    }
    if let AmendStep::Pushing = &app.amend_step {
        let msg = app.amend_message.clone();
        app.amend_step = match crate::git::commit::amend_commit(&msg) {
            Ok(_) => AmendStep::Done(t!("go_amend_ok").to_string()),
            Err(err) => AmendStep::Done(format!("❌ Amend failed: {}", err)),
        };
        return true;
    }
    false
}
