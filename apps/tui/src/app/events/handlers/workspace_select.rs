use crossterm::event::{KeyCode, KeyEvent};
use rust_i18n::t;

use crate::app::App;
use crate::models::ActiveModal;

pub(super) fn handle_workspace_history(app: &mut App, key: &KeyEvent) {
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') => {
            app.active_modal = ActiveModal::None;
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
            select_current_workspace(app);
        }
        KeyCode::Char('n') | KeyCode::Char('N') => {
            pick_new_workspace(app);
        }
        KeyCode::Char('p') | KeyCode::Char('P') => {
            app.workspace_path_input.clear();
            app.active_modal = ActiveModal::WorkspacePathInput;
        }
        KeyCode::Char('x') | KeyCode::Char('X') => {
            remove_workspace(app);
        }
        _ => {}
    }
}

fn select_current_workspace(app: &mut App) {
    if app.workspace_history.is_empty() {
        return;
    }

    let selected_path = app.workspace_history[app.selected_workspace_index].clone();
    if std::env::set_current_dir(&selected_path).is_ok() {
        app.current_dir = selected_path.clone();
        app.add_to_workspace_history(&selected_path);
        app.refresh_git_status();
        app.status_message = t!("select_workspace_ok", path = selected_path.clone()).to_string();
        app.active_modal = ActiveModal::None;
    } else {
        app.status_message = t!("select_workspace_err").to_string();
    }
}

fn pick_new_workspace(app: &mut App) {
    #[cfg(target_os = "linux")]
    let is_headless =
        std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err();
    #[cfg(not(target_os = "linux"))]
    let is_headless = false;

    if is_headless {
        app.workspace_path_input.clear();
        app.active_modal = ActiveModal::WorkspacePathInput;
        return;
    }

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
            app.active_modal = ActiveModal::None;
        } else {
            app.status_message = t!("select_workspace_err").to_string();
        }
    } else {
        app.status_message = t!("select_workspace_cancel").to_string();
    }
}

fn remove_workspace(app: &mut App) {
    if app.workspace_history.is_empty() {
        return;
    }

    let removed_path = app.workspace_history[app.selected_workspace_index].clone();
    if removed_path == app.current_dir {
        app.status_message = t!("select_workspace_active_err").to_string();
    } else {
        app.remove_from_workspace_history(app.selected_workspace_index);
        app.status_message =
            t!("select_workspace_removed", path = removed_path.clone()).to_string();
    }
}
