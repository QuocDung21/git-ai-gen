use crate::app::App;
use crate::models::GoStep;
use crossterm::event::{KeyCode, KeyEvent};

pub fn handle_git_menu(app: &mut App, key: &KeyEvent) {
    let max = 13;

    match key.code {
        KeyCode::Char('g') | KeyCode::Char('G') => {
            if !app.kilo_ai_enabled {
                app.status_message = if app.current_lang == "vi" {
                    "⚠️ Tính năng Kilo AI đã bị tắt trong Cài đặt!".to_string()
                } else {
                    "⚠️ Kilo AI Generation is disabled in Settings!".to_string()
                };
                return;
            }
            let clipboard_msg = if let Ok(mut cb) = arboard::Clipboard::new() {
                cb.get_text().unwrap_or_default()
            } else {
                String::new()
            };
            app.commit_message_preview = if clipboard_msg.trim().is_empty() {
                if app.current_lang == "vi" {
                    "(Chưa có commit message trong clipboard)".to_string()
                } else {
                    "(No commit message in clipboard)".to_string()
                }
            } else {
                clipboard_msg.trim().to_string()
            };
            app.go_step = GoStep::Confirm;
            app.auto_stage_all_if_enabled();
            app.active_modal = crate::models::ActiveModal::GoConfirm;
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
            app.status_message = if app.current_lang == "vi" {
                "⏳ Đang tải thông tin mới từ Remote (Fetch)...".to_string()
            } else {
                "⏳ Fetching new updates from Remote...".to_string()
            };
            let _ = crate::git::remote::git_fetch();
            app.status_message = if app.current_lang == "vi" {
                "✅ Fetch hoàn tất".to_string()
            } else {
                "✅ Fetch completed".to_string()
            };
            app.active_modal = crate::models::ActiveModal::None;
            app.refresh_git_status();
        }
        KeyCode::Char('p') | KeyCode::Char('P') => {
            app.status_message = if app.current_lang == "vi" {
                "⏳ Đang cập nhật thay đổi từ Remote (Pull)...".to_string()
            } else {
                "⏳ Pulling changes from Remote...".to_string()
            };
            let _ = crate::git::remote::git_pull();
            app.status_message = if app.current_lang == "vi" {
                "✅ Pull hoàn tất".to_string()
            } else {
                "✅ Pull completed".to_string()
            };
            app.active_modal = crate::models::ActiveModal::None;
            app.refresh_git_status();
        }
        KeyCode::Char('u') | KeyCode::Char('U') => {
            app.status_message = if app.current_lang == "vi" {
                "⏳ Đang đẩy các thay đổi lên Remote (Push)...".to_string()
            } else {
                "⏳ Pushing committed changes to Remote...".to_string()
            };
            match crate::git::remote::git_push() {
                Ok(_) => {
                    app.status_message = if app.current_lang == "vi" {
                        "✅ Push thành công".to_string()
                    } else {
                        "✅ Push successful".to_string()
                    };
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
                        app.status_message = if app.current_lang == "vi" {
                            "⚠️ Tính năng Kilo AI đã bị tắt trong Cài đặt!".to_string()
                        } else {
                            "⚠️ Kilo AI Generation is disabled in Settings!".to_string()
                        };
                    } else {
                        let clipboard_msg = if let Ok(mut cb) = arboard::Clipboard::new() {
                            cb.get_text().unwrap_or_default()
                        } else {
                            String::new()
                        };
                        app.commit_message_preview = if clipboard_msg.trim().is_empty() {
                            if app.current_lang == "vi" {
                                "(Chưa có commit message trong clipboard)".to_string()
                            } else {
                                "(No commit message in clipboard)".to_string()
                            }
                        } else {
                            clipboard_msg.trim().to_string()
                        };
                        app.go_step = GoStep::Confirm;
                        app.auto_stage_all_if_enabled();
                        app.active_modal = crate::models::ActiveModal::GoConfirm;
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
                    app.status_message = if app.current_lang == "vi" {
                        "⏳ Đang tải thông tin mới từ Remote (Fetch)...".to_string()
                    } else {
                        "⏳ Fetching new updates from Remote...".to_string()
                    };
                    let _ = crate::git::remote::git_fetch();
                    app.status_message = if app.current_lang == "vi" {
                        "✅ Fetch hoàn tất".to_string()
                    } else {
                        "✅ Fetch completed".to_string()
                    };
                    app.active_modal = crate::models::ActiveModal::None;
                    app.refresh_git_status();
                }
                4 => {
                    app.status_message = if app.current_lang == "vi" {
                        "⏳ Đang cập nhật thay đổi từ Remote (Pull)...".to_string()
                    } else {
                        "⏳ Pulling changes from Remote...".to_string()
                    };
                    let _ = crate::git::remote::git_pull();
                    app.status_message = if app.current_lang == "vi" {
                        "✅ Pull hoàn tất".to_string()
                    } else {
                        "✅ Pull completed".to_string()
                    };
                    app.active_modal = crate::models::ActiveModal::None;
                    app.refresh_git_status();
                }
                5 => {
                    app.status_message = if app.current_lang == "vi" {
                        "⏳ Đang đẩy các thay đổi lên Remote (Push)...".to_string()
                    } else {
                        "⏳ Pushing committed changes to Remote...".to_string()
                    };
                    match crate::git::remote::git_push() {
                        Ok(_) => {
                            app.status_message = if app.current_lang == "vi" {
                                "✅ Push thành công".to_string()
                            } else {
                                "✅ Push successful".to_string()
                            };
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
