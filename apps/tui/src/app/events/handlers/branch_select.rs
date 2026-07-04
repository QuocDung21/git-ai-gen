use crossterm::event::{KeyCode, KeyEvent};
use rust_i18n::t;

use crate::app::App;
use crate::models::ActiveModal;

pub(super) fn handle_branch_select(app: &mut App, key: &KeyEvent) {
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('b') | KeyCode::Char('B') => {
            app.active_modal = ActiveModal::None;
        }
        KeyCode::Up | KeyCode::Char('k') => {
            if app.selected_branch_index > 0 {
                app.selected_branch_index -= 1;
            } else if !app.branches.is_empty() {
                app.selected_branch_index = app.branches.len() - 1;
            }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            if !app.branches.is_empty() {
                if app.selected_branch_index < app.branches.len() - 1 {
                    app.selected_branch_index += 1;
                } else {
                    app.selected_branch_index = 0;
                }
            }
        }
        KeyCode::Char('c') | KeyCode::Char('C') => {
            app.new_branch_name.clear();
            app.active_modal = ActiveModal::NewBranchInput;
        }
        KeyCode::Char('d') | KeyCode::Char('D') | KeyCode::Char('x') | KeyCode::Char('X') => {
            confirm_delete_selected_branch(app);
        }
        KeyCode::Char('m') | KeyCode::Char('M') => {
            confirm_merge_selected_branch(app);
        }
        KeyCode::Enter => {
            checkout_selected_branch(app);
        }
        _ => {}
    }
}

pub(super) fn handle_merge_confirm(app: &mut App, key: &KeyEvent, branch_name: &str) {
    match key.code {
        KeyCode::Esc | KeyCode::Char('n') | KeyCode::Char('N') => {
            app.active_modal = ActiveModal::BranchSelect;
        }
        KeyCode::Enter | KeyCode::Char('y') | KeyCode::Char('Y') => {
            app.status_message = t!("select_branch_merging", name = branch_name).to_string();
            match crate::git::branch::git_merge(branch_name) {
                Ok(out) => {
                    app.status_message =
                        t!("select_branch_merge_ok", out = out.clone()).to_string();
                }
                Err(err) => {
                    app.status_message =
                        t!("select_branch_merge_err", err = err.to_string()).to_string();
                }
            }
            app.active_modal = ActiveModal::None;
            app.refresh_git_status();
        }
        _ => {}
    }
}

pub(super) fn handle_branch_delete_confirm(app: &mut App, key: &KeyEvent, branch_name: &str) {
    match key.code {
        KeyCode::Esc | KeyCode::Char('n') | KeyCode::Char('N') => {
            app.active_modal = ActiveModal::BranchSelect;
        }
        KeyCode::Enter | KeyCode::Char('y') | KeyCode::Char('Y') => {
            app.status_message = t!("select_branch_deleting", name = branch_name).to_string();
            match crate::git::branch::delete_branch(
                branch_name,
                crate::git::branch::DeleteBranchOptions::default(),
            ) {
                Ok(out) => {
                    app.status_message =
                        t!("select_branch_delete_ok", out = out.clone()).to_string();
                }
                Err(err) => {
                    app.status_message =
                        t!("select_branch_delete_fail", err = err.to_string()).to_string();
                }
            }
            app.fetch_branches();
            if app.selected_branch_index >= app.branches.len() {
                app.selected_branch_index = app.branches.len().saturating_sub(1);
            }
            app.active_modal = ActiveModal::BranchSelect;
            app.refresh_git_status();
        }
        _ => {}
    }
}

fn confirm_delete_selected_branch(app: &mut App) {
    if app.branches.is_empty() || app.selected_branch_index >= app.branches.len() {
        return;
    }

    let branch = &app.branches[app.selected_branch_index];
    if branch.name == app.current_branch && !branch.is_remote {
        app.status_message = t!("select_branch_delete_err").to_string();
    } else {
        app.active_modal = ActiveModal::BranchDeleteConfirm(branch.name.clone());
    }
}

fn confirm_merge_selected_branch(app: &mut App) {
    if app.branches.is_empty() || app.selected_branch_index >= app.branches.len() {
        return;
    }

    let branch_name = app.branches[app.selected_branch_index].name.clone();
    if branch_name != app.current_branch {
        app.active_modal = ActiveModal::MergeConfirm(branch_name);
    } else {
        app.status_message = t!("select_branch_merge_self_err").to_string();
    }
}

fn checkout_selected_branch(app: &mut App) {
    if app.branches.is_empty() || app.selected_branch_index >= app.branches.len() {
        return;
    }

    let branch_name = app.branches[app.selected_branch_index].name.clone();
    match crate::git::branch::checkout_branch(&branch_name) {
        Ok(_) => {
            app.status_message =
                t!("select_branch_checkout_ok", name = branch_name.clone()).to_string();
        }
        Err(err) => {
            app.status_message =
                t!("select_branch_checkout_err", err = err.to_string()).to_string();
        }
    }
    app.active_modal = ActiveModal::None;
    app.refresh_git_status();
}
