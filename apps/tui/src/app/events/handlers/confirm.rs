use crossterm::event::{KeyCode, KeyEvent};
use rust_i18n::t;

use crate::app::App;

pub fn handle_revert_confirm(app: &mut App, key: &KeyEvent) {
    match key.code {
        KeyCode::Char('y') | KeyCode::Char('Y') | KeyCode::Enter => {
            let path_to_revert = match &app.active_modal {
                crate::models::ActiveModal::RevertConfirm(path) => path.clone(),
                _ => return,
            };
            let is_untracked = app.files.iter().any(|f| {
                f.path == path_to_revert && (f.status.starts_with("??") || f.status.contains("??"))
            });

            if is_untracked {
                let p = std::path::Path::new(&path_to_revert);
                if p.is_dir() {
                    let _ = std::fs::remove_dir_all(p);
                } else {
                    let _ = std::fs::remove_file(p);
                }
                app.status_message =
                    t!("revert_deleted_untracked", path = path_to_revert).to_string();
            } else {
                let _ = crate::git::status::unstage_file(&path_to_revert);
                let success = crate::git::status::revert_file(&path_to_revert).is_ok();
                if success {
                    app.status_message = t!("revert_success", path = path_to_revert).to_string();
                } else {
                    app.status_message = t!("revert_failed", path = path_to_revert).to_string();
                }
            }
            app.active_modal = crate::models::ActiveModal::None;
            app.refresh_git_status();
        }
        KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
            app.active_modal = crate::models::ActiveModal::None;
            app.status_message = t!("revert_cancelled").to_string();
        }
        _ => {}
    }
}
