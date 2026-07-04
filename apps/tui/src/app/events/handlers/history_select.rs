use crossterm::event::{KeyCode, KeyEvent};
use rust_i18n::t;

use crate::app::App;
use crate::models::ActiveModal;

pub(super) fn handle_git_log(app: &mut App, key: &KeyEvent) {
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('v') | KeyCode::Char('V') => {
            app.active_modal = ActiveModal::None;
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
        KeyCode::Enter
            if !app.commit_logs.is_empty() && app.selected_log_index < app.commit_logs.len() =>
        {
            let hash = app.commit_logs[app.selected_log_index].hash.clone();
            app.fetch_commit_diff(&hash);
            app.active_modal = ActiveModal::CommitDiff(hash);
        }
        _ => {}
    }
}

pub(super) fn handle_commit_diff(app: &mut App, key: &KeyEvent) {
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') => {
            app.active_modal = ActiveModal::GitLog;
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
    }
}

pub(super) fn handle_commit_tree(app: &mut App, key: &KeyEvent) {
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('t') | KeyCode::Char('T') => {
            app.active_modal = ActiveModal::None;
        }
        KeyCode::Up | KeyCode::Char('k') => {
            if app.selected_log_index > 0 {
                app.selected_log_index -= 1;
            } else if !app.commit_logs.is_empty() {
                app.selected_log_index = app.commit_logs.len() - 1;
            }
            fetch_selected_commit_diff(app);
        }
        KeyCode::Down | KeyCode::Char('j') => {
            if !app.commit_logs.is_empty() {
                app.selected_log_index = (app.selected_log_index + 1) % app.commit_logs.len();
            }
            fetch_selected_commit_diff(app);
        }
        _ => {}
    }
}

pub(super) fn handle_feature_commit(app: &mut App, key: &KeyEvent) {
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('e') | KeyCode::Char('E') => {
            app.active_modal = ActiveModal::None;
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
            stage_selected_feature_group(app);
        }
        _ => {}
    }
}

fn fetch_selected_commit_diff(app: &mut App) {
    if !app.commit_logs.is_empty() {
        let hash = app.commit_logs[app.selected_log_index].hash.clone();
        app.fetch_commit_diff(&hash);
    }
}

fn stage_selected_feature_group(app: &mut App) {
    if app.feature_groups.is_empty() || app.selected_feature_index >= app.feature_groups.len() {
        return;
    }

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

    app.active_modal = ActiveModal::None;
}
