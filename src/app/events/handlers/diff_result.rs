use crate::app::App;
use crossterm::event::{KeyCode, KeyEvent};

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
    match key.code {
        KeyCode::Esc
        | KeyCode::Enter
        | KeyCode::Char('q')
        | KeyCode::Char('d')
        | KeyCode::Backspace => {
            close_modal(app);
        }
        KeyCode::Char('g') => {
            app.selected_git_action = 0;
            app.active_modal = crate::models::ActiveModal::GitMenu;
        }
        _ => {}
    }
}

pub fn close_modal(app: &mut App) {
    app.active_modal = crate::models::ActiveModal::None;
}
