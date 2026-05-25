use std::process::Command;

use crossterm::event::KeyCode;
use ratatui::backend::Backend;
use ratatui::Terminal;

use crate::app::App;
use crate::git::remote::{git_fetch, git_pull};
use crate::git::status::{stage_all, stage_file, unstage_all, unstage_file};

use crate::app::events::run_cli_command;

pub fn handle_standard_keys<B: Backend + std::io::Write>(
    app: &mut App,
    terminal: &mut Terminal<B>,
    key: &crossterm::event::KeyEvent,
) -> Result<(), anyhow::Error> {
    // Handle diff-focus mode keys first
    if app.focus_diff {
        match key.code {
            KeyCode::Char('q') => {
                // Quit always works regardless of focus
                std::process::exit(0);
            }
            KeyCode::Down | KeyCode::Char('j') => {
                app.diff_scroll_offset = app.diff_scroll_offset.saturating_add(1);
            }
            KeyCode::Up | KeyCode::Char('k') => {
                if app.diff_scroll_offset > 0 {
                    app.diff_scroll_offset = app.diff_scroll_offset.saturating_sub(1);
                }
            }
            KeyCode::PageDown | KeyCode::Char('d') => {
                app.diff_scroll_offset = app.diff_scroll_offset.saturating_add(10);
            }
            KeyCode::PageUp | KeyCode::Char('u') => {
                if app.diff_scroll_offset > 0 {
                    app.diff_scroll_offset = app.diff_scroll_offset.saturating_sub(10);
                }
            }
            KeyCode::Char(' ') => {
                if !app.files.is_empty() && app.selected_index < app.files.len() {
                    let file = &app.files[app.selected_index];
                    let is_staged = !file.status.starts_with(' ') && !file.status.starts_with('?');
                    let path = file.path.clone();

                    if is_staged {
                        let _ = unstage_file(&path);
                        app.status_message = if app.current_lang == "vi" {
                            format!("➖ Đã unstage: {}", path)
                        } else {
                            format!("➖ Unstaged: {}", path)
                        };
                    } else {
                        let _ = stage_file(&path);
                        app.status_message = if app.current_lang == "vi" {
                            format!("➕ Đã stage: {}", path)
                        } else {
                            format!("➕ Staged: {}", path)
                        };
                    }
                    app.refresh_git_status();
                }
            }
            KeyCode::Tab | KeyCode::Esc | KeyCode::Left => {
                app.focus_diff = false;
                app.status_message = if app.current_lang == "vi" {
                    "📂 Đã quay lại Danh sách tập tin".to_string()
                } else {
                    "📂 Returned to Files list".to_string()
                };
            }
            _ => {}
        }
        return Ok(());
    }

    match key.code {
        KeyCode::Char('q') => {
            // Graceful quit: return to caller
            std::process::exit(0);
        }
        KeyCode::Up | KeyCode::Char('k') => {
            if !app.files.is_empty() {
                if app.selected_index > 0 {
                    app.selected_index -= 1;
                } else {
                    app.selected_index = app.files.len() - 1;
                }
                app.diff_scroll_offset = 0;
                app.update_selected_diff();
            }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            if !app.files.is_empty() {
                if app.selected_index < app.files.len() - 1 {
                    app.selected_index += 1;
                } else {
                    app.selected_index = 0;
                }
                app.diff_scroll_offset = 0;
                app.update_selected_diff();
            }
        }
        KeyCode::Char(' ') => {
            if !app.files.is_empty() && app.selected_index < app.files.len() {
                let file = &app.files[app.selected_index];
                let is_staged = !file.status.starts_with(' ') && !file.status.starts_with('?');
                let path = file.path.clone();

                if is_staged {
                    let _ = unstage_file(&path);
                    app.status_message = if app.current_lang == "vi" {
                        format!("➖ Đã unstage: {}", path)
                    } else {
                        format!("➖ Unstaged: {}", path)
                    };
                } else {
                    let _ = stage_file(&path);
                    app.status_message = if app.current_lang == "vi" {
                        format!("➕ Đã stage: {}", path)
                    } else {
                        format!("➕ Staged: {}", path)
                    };
                }
                app.refresh_git_status();
            }
        }
        KeyCode::Backspace => {
            if !app.files.is_empty() && app.selected_index < app.files.len() {
                let file = &app.files[app.selected_index];
                app.active_modal = crate::models::ActiveModal::RevertConfirm(file.path.clone());
            }
        }
        KeyCode::PageUp => {
            if app.diff_scroll_offset > 0 {
                app.diff_scroll_offset = app.diff_scroll_offset.saturating_sub(5);
            }
        }
        KeyCode::PageDown => {
            app.diff_scroll_offset = app.diff_scroll_offset.saturating_add(5);
        }
        KeyCode::Tab | KeyCode::Right => {
            if !app.files.is_empty() {
                app.focus_diff = true;
                app.status_message = if app.current_lang == "vi" {
                    "📄 Đã chuyển focus sang Diff. Nhấn j/k để cuộn dòng, d/u để cuộn trang, Tab/Esc để quay lại.".to_string()
                } else {
                    "📄 Focused Diff view. Press j/k to line scroll, d/u to page scroll, Tab/Esc to return.".to_string()
                };
            }
        }
        KeyCode::Char('[') => {
            if app.diff_scroll_offset > 0 {
                app.diff_scroll_offset = app.diff_scroll_offset.saturating_sub(5);
            }
        }
        KeyCode::Char(']') => {
            app.diff_scroll_offset = app.diff_scroll_offset.saturating_add(5);
        }
        KeyCode::Char('d') => {
            handle_diff_capture(app);
        }
        KeyCode::Char('o') => match Command::new("code").arg(".").spawn() {
            Ok(_) => {
                app.status_message = if app.current_lang == "vi" {
                    "🚀 Đã mở dự án bằng VS Code!".to_string()
                } else {
                    "🚀 Opened project in VS Code!".to_string()
                };
            }
            Err(_) => {
                app.status_message = if app.current_lang == "vi" {
                    "❌ Lỗi: Không tìm thấy lệnh 'code'.".to_string()
                } else {
                    "❌ Error: 'code' command not found.".to_string()
                };
            }
        },
        KeyCode::Char('g') => {
            app.selected_git_action = 0;
            app.active_modal = crate::models::ActiveModal::GitMenu;
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
        KeyCode::Char('r') => {
            let locales = crate::cli::Locales::new(&app.current_lang);
            run_cli_command(terminal, || crate::cli::system::handle_restore(&locales))?;
            app.refresh_git_status();
            app.status_message = if app.current_lang == "vi" {
                "🔄 Đã reset cấu hình hệ thống.".to_string()
            } else {
                "🔄 System configuration reset.".to_string()
            };
        }
        KeyCode::Char('l') => {
            let raw_lang = if let Ok(output) = Command::new("git")
                .args(["config", "--global", "--get", "git-ai.lang"])
                .output()
            {
                let stdout = String::from_utf8_lossy(&output.stdout)
                    .trim()
                    .to_lowercase();
                if stdout == "vi" || stdout == "en" {
                    stdout
                } else {
                    "auto".to_string()
                }
            } else {
                "auto".to_string()
            };
            app.selected_lang_index = match raw_lang.as_str() {
                "vi" => 0,
                "en" => 1,
                _ => 2,
            };
            app.active_modal = crate::models::ActiveModal::LanguageSelect;
        }
        KeyCode::Char('a') | KeyCode::Char('A') => {
            let success = stage_all().is_ok();
            if success {
                app.status_message = if app.current_lang == "vi" {
                    "➕ Đã stage toàn bộ thay đổi!".to_string()
                } else {
                    "➕ Staged all changes!".to_string()
                };
            } else {
                app.status_message = if app.current_lang == "vi" {
                    "❌ Lỗi: Không thể stage toàn bộ.".to_string()
                } else {
                    "❌ Error: Failed to stage all.".to_string()
                };
            }
            app.refresh_git_status();
        }
        KeyCode::Char('u') | KeyCode::Char('U') => {
            let success = unstage_all().is_ok();
            if success {
                app.status_message = if app.current_lang == "vi" {
                    "➖ Đã unstage toàn bộ thay đổi!".to_string()
                } else {
                    "➖ Unstaged all changes!".to_string()
                };
            } else {
                app.status_message = if app.current_lang == "vi" {
                    "❌ Lỗi: Không thể unstage toàn bộ.".to_string()
                } else {
                    "❌ Error: Failed to unstage all.".to_string()
                };
            }
            app.refresh_git_status();
        }
        KeyCode::Char('v') | KeyCode::Char('V') => {
            app.active_modal = crate::models::ActiveModal::GitLog;
            app.selected_log_index = 0;
            app.fetch_commit_logs();
        }
        KeyCode::Char('b') | KeyCode::Char('B') => {
            app.active_modal = crate::models::ActiveModal::BranchSelect;
            app.fetch_branches();
        }
        KeyCode::Char('f') | KeyCode::Char('F') => {
            let is_vi = app.current_lang == "vi";
            app.status_message = if is_vi {
                "⏳ Đang tải thông tin mới từ Remote (Fetch)...".to_string()
            } else {
                "⏳ Fetching new updates from Remote...".to_string()
            };
            terminal.draw(|f| crate::ui::ui(f, app))?;
            match git_fetch() {
                Ok(_) => {
                    app.status_message = if is_vi {
                        "✅ Đã tìm nạp (git fetch) thành công!".to_string()
                    } else {
                        "✅ Git fetch completed successfully!".to_string()
                    };
                }
                Err(err) => {
                    app.status_message = if is_vi {
                        format!("❌ Lỗi git fetch: {}", err)
                    } else {
                        format!("❌ git fetch failed: {}", err)
                    };
                }
            }
            app.refresh_git_status();
        }
        KeyCode::Char('p') | KeyCode::Char('P') => {
            let is_vi = app.current_lang == "vi";
            app.status_message = if is_vi {
                "⏳ Đang cập nhật thay đổi từ Remote (Pull)...".to_string()
            } else {
                "⏳ Pulling changes from Remote...".to_string()
            };
            terminal.draw(|f| crate::ui::ui(f, app))?;
            match git_pull() {
                Ok(_) => {
                    app.status_message = if is_vi {
                        "✅ Đã cập nhật (git pull) thành công!".to_string()
                    } else {
                        "✅ Git pull completed successfully!".to_string()
                    };
                }
                Err(err) => {
                    app.status_message = if is_vi {
                        format!("❌ Lỗi git pull: {}", err)
                    } else {
                        format!("❌ git pull failed: {}", err)
                    };
                }
            }
            app.refresh_git_status();
        }
        KeyCode::Char('?') | KeyCode::Char('h') => {
            app.active_modal = crate::models::ActiveModal::Help;
        }
        KeyCode::Char('s') | KeyCode::Char('S') => {
            app.active_modal = crate::models::ActiveModal::StashList;
            app.fetch_stash();
        }
        KeyCode::Char('i') | KeyCode::Char('I') => {
            app.fetch_remote_info();
            app.active_modal = crate::models::ActiveModal::RemoteInfo;
        }
        KeyCode::Char('x') | KeyCode::Char('X') => {
            app.fetch_prompt();
            app.active_modal = crate::models::ActiveModal::ViewPrompt;
        }
        KeyCode::Char('m') | KeyCode::Char('M') => {
            app.fetch_amend_msg();
            app.active_modal = crate::models::ActiveModal::AmendCommit;
        }
        KeyCode::Char('w') => {
            app.load_workspace_history();
            app.selected_workspace_index = 0;
            app.active_modal = crate::models::ActiveModal::WorkspaceHistory;
        }
        KeyCode::Char('t') | KeyCode::Char('T') => {
            app.selected_theme_index = if app.is_light_theme { 1 } else { 0 };
            app.active_modal = crate::models::ActiveModal::ThemeSelect;
        }
        KeyCode::Char(',') => {
            app.selected_setting_index = 0;
            app.active_modal = crate::models::ActiveModal::Settings;
        }
        _ => {}
    }
    Ok(())
}

fn handle_diff_capture(app: &mut App) {
    let diff_output = Command::new("git").args(["diff", "--cached"]).output();

    match diff_output {
        Ok(out) => {
            let diff_str = String::from_utf8_lossy(&out.stdout).to_string();
            if diff_str.trim().is_empty() {
                app.status_message = if app.current_lang == "vi" {
                    "⚠️ Bạn chưa chọn (stage) file nào! Hãy nhấn [Space] để chọn file trước khi bấm 'd'.".to_string()
                } else {
                    "⚠️ No files staged! Please press [Space] to select files before pressing 'd'."
                        .to_string()
                };
            } else {
                app.diff_added_lines = diff_str
                    .lines()
                    .filter(|l| l.starts_with('+') && !l.starts_with("++"))
                    .count();
                app.diff_removed_lines = diff_str
                    .lines()
                    .filter(|l| l.starts_with('-') && !l.starts_with("--"))
                    .count();

                let preview: String = diff_str.lines().take(40).collect::<Vec<_>>().join("\n");
                app.diff_snapshot = preview;
                app.last_staged_diff = diff_str.clone();
                app.diff_kilo_generated.clear();

                let ai_lang = crate::helper::Helper::get_ai_language_name();
                let prompt = format!(
                    "{} {}.\n\nDiff:\n\n{}",
                    crate::constant::Constant::PROMPT_EXPERT,
                    ai_lang,
                    diff_str
                );
                if let Ok(mut cb) = arboard::Clipboard::new() {
                    let _ = cb.set_text(prompt);
                }
                app.active_modal = crate::models::ActiveModal::DiffResult;
            }
        }
        Err(e) => {
            app.status_message = format!("❌ Error capturing diff: {}", e);
        }
    }
}
