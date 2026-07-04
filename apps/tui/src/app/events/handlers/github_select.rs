use std::process::Command;

use crossterm::event::{KeyCode, KeyEvent};
use rust_i18n::t;

use crate::app::App;
use crate::models::ActiveModal;

pub(super) fn handle_github_quick_view(app: &mut App, key: &KeyEvent) {
    if app.github_quickview_searching {
        handle_github_quick_view_search(app, key);
    } else {
        handle_github_quick_view_browse(app, key);
    }
}

pub(super) fn handle_github_branch_select(app: &mut App, key: &KeyEvent) {
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') => {
            app.active_modal = ActiveModal::GithubDownloadTree;
        }
        KeyCode::Up | KeyCode::Char('k') => {
            if !app.github_branches.is_empty() {
                if app.selected_github_branch_index > 0 {
                    app.selected_github_branch_index -= 1;
                } else {
                    app.selected_github_branch_index = app.github_branches.len() - 1;
                }
            }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            if !app.github_branches.is_empty() {
                if app.selected_github_branch_index < app.github_branches.len() - 1 {
                    app.selected_github_branch_index += 1;
                } else {
                    app.selected_github_branch_index = 0;
                }
            }
        }
        KeyCode::Enter => {
            switch_github_branch(app);
        }
        _ => {}
    }
}

fn handle_github_quick_view_search(app: &mut App, key: &KeyEvent) {
    match key.code {
        KeyCode::Esc | KeyCode::Enter => {
            app.github_quickview_searching = false;
        }
        KeyCode::Backspace => {
            app.github_quickview_search.pop();
        }
        KeyCode::Char(c) => {
            app.github_quickview_search.push(c);
        }
        _ => {}
    }
}

fn handle_github_quick_view_browse(app: &mut App, key: &KeyEvent) {
    match key.code {
        KeyCode::Esc | KeyCode::Enter => {
            app.github_quickview_search.clear();
            app.active_modal = ActiveModal::GithubDownloadTree;
        }
        KeyCode::Up | KeyCode::Char('k') => {
            app.github_quickview_scroll = app.github_quickview_scroll.saturating_sub(1);
        }
        KeyCode::Down | KeyCode::Char('j') => {
            app.github_quickview_scroll = app.github_quickview_scroll.saturating_add(1);
        }
        KeyCode::PageUp => {
            app.github_quickview_scroll = app.github_quickview_scroll.saturating_sub(20);
        }
        KeyCode::PageDown => {
            app.github_quickview_scroll = app.github_quickview_scroll.saturating_add(20);
        }
        KeyCode::Char('/') => {
            app.github_quickview_searching = true;
            app.github_quickview_search.clear();
        }
        KeyCode::Char('c') | KeyCode::Char('C') => {
            app.github_quickview_search.clear();
        }
        _ => {}
    }
}

fn switch_github_branch(app: &mut App) {
    if app.github_branches.is_empty()
        || app.selected_github_branch_index >= app.github_branches.len()
    {
        return;
    }

    let selected_branch = app.github_branches[app.selected_github_branch_index].clone();
    app.status_message = t!("github_branch_switching", name = selected_branch.clone()).to_string();
    if let Some(dir) = app
        .github_temp_dir
        .as_ref()
        .map(|dir| dir.path().to_path_buf())
    {
        let fetch_out = Command::new("git")
            .args(["fetch", "--depth", "1", "origin", &selected_branch])
            .current_dir(&dir)
            .output();
        if let Ok(out) = fetch_out {
            if out.status.success() {
                checkout_github_fetch_head(app, selected_branch, &dir);
            } else {
                let stderr = String::from_utf8_lossy(&out.stderr).trim().to_string();
                app.status_message = t!("github_branch_fetch_err", err = stderr).to_string();
            }
        } else {
            app.status_message = t!("github_branch_fetch_cmd_err").to_string();
        }
    }
}

fn checkout_github_fetch_head(app: &mut App, selected_branch: String, dir: &std::path::Path) {
    let checkout_out = Command::new("git")
        .args(["checkout", "FETCH_HEAD"])
        .current_dir(dir)
        .output();
    if let Ok(c_out) = checkout_out {
        if c_out.status.success() {
            if let Err(e) = app.visit_repo_dir() {
                app.status_message = t!("github_repo_visit_err", err = e.to_string()).to_string();
            } else {
                app.current_github_branch = selected_branch;
                app.selected_github_tree_index = 0;
                app.active_modal = ActiveModal::GithubDownloadTree;
                app.status_message = t!("github_branch_ok").to_string();
            }
        } else {
            app.status_message = t!("github_branch_checkout_err").to_string();
        }
    } else {
        app.status_message = t!("github_branch_checkout_cmd_err").to_string();
    }
}
