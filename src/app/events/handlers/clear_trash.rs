use crate::app::App;
use crossterm::event::{KeyCode, KeyEvent};
use rust_i18n::t;

pub fn handle_clear_trash_confirm(app: &mut App, key: &KeyEvent) {
    match key.code {
        KeyCode::Char('y') | KeyCode::Char('Y') | KeyCode::Enter => {
            #[cfg(target_os = "macos")]
            {
                match crate::cli::clear_trash::empty_macos_trash() {
                    Ok(_) => {
                        app.status_message = t!("clear_trash_success").to_string();
                    }
                    Err(e) => {
                        app.status_message = format!("{} {}", t!("clear_trash_failed"), e);
                    }
                }
            }

            #[cfg(not(target_os = "macos"))]
            {
                app.status_message = t!("clear_trash_unsupported").to_string();
            }

            app.active_modal = crate::models::ActiveModal::None;
        }
        KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
            app.active_modal = crate::models::ActiveModal::None;
            app.status_message = t!("clear_trash_cancelled").to_string();
        }
        _ => {}
    }
}
