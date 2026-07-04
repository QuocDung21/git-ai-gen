#![allow(clippy::collapsible_match)]

use crate::app::App;
use crate::models::{AiTemp, GoStep};
use crossterm::event::{KeyCode, KeyEvent};
use rust_i18n::t;

pub fn handle_git_menu(app: &mut App, key: &KeyEvent) {
    let max = 14;

    match key.code {
        KeyCode::Char('g') | KeyCode::Char('G') => {
            if !app.kilo_ai_enabled {
                app.status_message = t!("kilo_disabled").to_string();
                return;
            }
            open_ai_commit_confirm(app);
        }
        KeyCode::Char('n') | KeyCode::Char('N') => {
            app.github_download_url.clear();
            app.github_cloning_error = None;
            app.github_cloning = false;
            app.active_modal = crate::models::ActiveModal::GithubDownloadUrlInput;
        }
        KeyCode::Char('c') | KeyCode::Char('C') => {
            app.manual_commit_message.clear();
            app.auto_stage_all_if_enabled();
            app.active_modal = crate::models::ActiveModal::ManualCommit;
        }
        KeyCode::Char('m') | KeyCode::Char('M') => {
            app.fetch_amend_msg();
            app.active_modal = crate::models::ActiveModal::AmendCommit;
        }
        KeyCode::Char('f') | KeyCode::Char('F') => {
            app.status_message = t!("nav_fetch_start").to_string();
            let _ = crate::git::remote::git_fetch();
            app.status_message = t!("nav_fetch_ok").to_string();
            app.active_modal = crate::models::ActiveModal::None;
            app.refresh_git_status();
        }
        KeyCode::Char('p') | KeyCode::Char('P') => {
            app.status_message = t!("nav_pull_start").to_string();
            let _ = crate::git::remote::git_pull();
            app.status_message = t!("nav_pull_ok").to_string();
            app.active_modal = crate::models::ActiveModal::None;
            app.refresh_git_status();
        }
        KeyCode::Char('u') | KeyCode::Char('U') => {
            app.status_message = t!("nav_push_start").to_string();
            match crate::git::remote::git_push() {
                Ok(_) => {
                    app.status_message = t!("nav_push_ok").to_string();
                }
                Err(e) => {
                    app.status_message = format!("❌ Push failed: {}", e);
                }
            }
            app.active_modal = crate::models::ActiveModal::None;
            app.refresh_git_status();
        }
        KeyCode::Char('i') | KeyCode::Char('I') => {
            app.fetch_remote_info();
            app.active_modal = crate::models::ActiveModal::RemoteInfo;
        }
        KeyCode::Char('b') | KeyCode::Char('B') => {
            app.fetch_branches();
            app.active_modal = crate::models::ActiveModal::BranchSelect;
        }
        KeyCode::Char('s') | KeyCode::Char('S') => {
            app.fetch_stash();
            app.active_modal = crate::models::ActiveModal::StashList;
        }
        KeyCode::Char('v') | KeyCode::Char('V') => {
            app.fetch_commit_logs();
            app.active_modal = crate::models::ActiveModal::GitLog;
        }
        KeyCode::Char('t') | KeyCode::Char('T') => {
            app.fetch_commit_tree();
            app.selected_log_index = 0;
            if !app.commit_logs.is_empty() {
                let hash = app.commit_logs[0].hash.clone();
                app.fetch_commit_diff(&hash);
            }
            app.active_modal = crate::models::ActiveModal::CommitTree;
        }
        KeyCode::Char('e') | KeyCode::Char('E') => {
            app.compute_feature_groups();
            app.active_modal = crate::models::ActiveModal::FeatureCommit;
        }
        KeyCode::Char('y') | KeyCode::Char('Y') => {
            app.selected_setting_index = 0;
            app.active_modal = crate::models::ActiveModal::Settings;
        }
        KeyCode::Char('x') | KeyCode::Char('X') => {
            app.active_modal = crate::models::ActiveModal::ClearTrashConfirm;
        }
        _ => {}
    }

    // Handle navigation keys (Esc, Enter, arrows) for the menu list
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') => {
            app.active_modal = crate::models::ActiveModal::None;
        }
        KeyCode::Enter => {
            match app.selected_git_action {
                0 => {
                    // AI Commit & Push
                    if !app.kilo_ai_enabled {
                        app.status_message = t!("kilo_disabled").to_string();
                    } else {
                        open_ai_commit_confirm(app);
                    }
                }
                1 => {
                    // Manual Commit
                    app.manual_commit_message.clear();
                    app.auto_stage_all_if_enabled();
                    app.active_modal = crate::models::ActiveModal::ManualCommit;
                }
                2 => {
                    app.fetch_amend_msg();
                    app.active_modal = crate::models::ActiveModal::AmendCommit;
                }
                3 => {
                    app.status_message = t!("nav_fetch_start").to_string();
                    let _ = crate::git::remote::git_fetch();
                    app.status_message = t!("nav_fetch_ok").to_string();
                    app.active_modal = crate::models::ActiveModal::None;
                    app.refresh_git_status();
                }
                4 => {
                    app.status_message = t!("nav_pull_start").to_string();
                    let _ = crate::git::remote::git_pull();
                    app.status_message = t!("nav_pull_ok").to_string();
                    app.active_modal = crate::models::ActiveModal::None;
                    app.refresh_git_status();
                }
                5 => {
                    app.status_message = t!("nav_push_start").to_string();
                    match crate::git::remote::git_push() {
                        Ok(_) => {
                            app.status_message = t!("nav_push_ok").to_string();
                        }
                        Err(e) => {
                            app.status_message = format!("❌ Push failed: {}", e);
                        }
                    }
                    app.active_modal = crate::models::ActiveModal::None;
                    app.refresh_git_status();
                }
                6 => {
                    app.fetch_remote_info();
                    app.active_modal = crate::models::ActiveModal::RemoteInfo;
                }
                7 => {
                    app.fetch_branches();
                    app.active_modal = crate::models::ActiveModal::BranchSelect;
                }
                8 => {
                    app.fetch_stash();
                    app.active_modal = crate::models::ActiveModal::StashList;
                }
                9 => {
                    app.fetch_commit_tree();
                    app.selected_log_index = 0;
                    if !app.commit_logs.is_empty() {
                        let hash = app.commit_logs[0].hash.clone();
                        app.fetch_commit_diff(&hash);
                    }
                    app.active_modal = crate::models::ActiveModal::CommitTree;
                }
                10 => {
                    app.fetch_commit_logs();
                    app.active_modal = crate::models::ActiveModal::GitLog;
                }
                11 => {
                    app.compute_feature_groups();
                    app.active_modal = crate::models::ActiveModal::FeatureCommit;
                }
                12 => {
                    app.github_download_url.clear();
                    app.github_cloning_error = None;
                    app.github_cloning = false;
                    app.active_modal = crate::models::ActiveModal::GithubDownloadUrlInput;
                }
                13 => {
                    app.active_modal = crate::models::ActiveModal::ClearTrashConfirm;
                }
                14 => {
                    app.selected_setting_index = 0;
                    app.active_modal = crate::models::ActiveModal::Settings;
                }
                _ => {}
            }
        }
        KeyCode::Up | KeyCode::Char('k') => {
            if app.selected_git_action > 0 {
                app.selected_git_action -= 1;
            }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            if app.selected_git_action < max {
                app.selected_git_action += 1;
            }
        }
        _ => {}
    }
}

fn open_ai_commit_confirm(app: &mut App) {
    let clipboard_msg = if let Ok(mut cb) = arboard::Clipboard::new() {
        cb.get_text().unwrap_or_default()
    } else {
        String::new()
    };

    app.commit_message_preview = if clipboard_msg.trim().is_empty() {
        t!("no_commit_in_clipboard").to_string()
    } else {
        let message = clipboard_msg.trim().to_string();
        app.ai_temp = AiTemp::GeneratedMessage(message.clone());
        message
    };
    app.go_step = GoStep::Confirm;
    app.auto_stage_all_if_enabled();
    app.active_modal = crate::models::ActiveModal::GoConfirm;
}
