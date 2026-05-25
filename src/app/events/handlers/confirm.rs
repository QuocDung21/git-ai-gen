use crossterm::event::{KeyCode, KeyEvent};

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
                app.status_message = if app.current_lang == "vi" {
                    format!("🗑️ Đã xóa file chưa theo dõi: {}", path_to_revert)
                } else {
                    format!("🗑️ Deleted untracked file: {}", path_to_revert)
                };
            } else {
                let _ = crate::git::status::unstage_file(&path_to_revert);
                let success = crate::git::status::revert_file(&path_to_revert).is_ok();
                if success {
                    app.status_message = if app.current_lang == "vi" {
                        format!("🔄 Đã khôi phục file: {}", path_to_revert)
                    } else {
                        format!("🔄 Reverted changes in file: {}", path_to_revert)
                    };
                } else {
                    app.status_message = if app.current_lang == "vi" {
                        format!("❌ Lỗi khi khôi phục file: {}", path_to_revert)
                    } else {
                        format!("❌ Failed to revert file: {}", path_to_revert)
                    };
                }
            }
            app.active_modal = crate::models::ActiveModal::None;
            app.refresh_git_status();
        }
        KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
            app.active_modal = crate::models::ActiveModal::None;
            app.status_message = if app.current_lang == "vi" {
                "ℹ️ Đã hủy khôi phục file.".to_string()
            } else {
                "ℹ️ Revert cancelled.".to_string()
            };
        }
        _ => {}
    }
}
