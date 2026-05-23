use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use std::process::Command;
use std::{io, time::Duration};

use crate::app::models::{ActiveModal, AmendStep, GoStep, StashAction, StashStep};
use crate::app::App;
use crate::git::branch::{checkout_branch, create_and_checkout_branch, git_merge};
use crate::git::commit::{amend_commit, commit};
use crate::git::remote::{git_fetch, git_pull, git_push};
use crate::git::stash::{stash_apply, stash_drop, stash_pop, stash_push};
use crate::git::status::{revert_file, stage_all, stage_file, unstage_all, unstage_file};
use crate::helper::Helper;

pub fn run_dashboard() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    res?;
    Ok(())
}

fn run_app<B: Backend + std::io::Write>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()> {
    loop {
        terminal.draw(|f| crate::ui::ui(f, app))?;

        // Handle GoStep::Pushing: run git commands outside of event poll
        if app.active_modal == ActiveModal::GoConfirm {
            if let GoStep::Pushing = &app.go_step {
                let is_vi = app.current_lang == "vi";
                let msg = app.commit_message_preview.clone();

                let commit_ok = commit(&msg).is_ok();

                if !commit_ok {
                    app.go_step = GoStep::Done(if is_vi {
                        "❌ Lỗi: git commit thất bại. Hãy chắc chắn bạn đã chọn file cần commit."
                            .to_string()
                    } else {
                        "❌ Error: git commit failed. Make sure you have staged files to commit."
                            .to_string()
                    });
                } else {
                    if app.auto_push {
                        match git_push() {
                            Ok(_) => {
                                app.go_step = GoStep::Done(if is_vi {
                                    "✅ Commit & Push thành công! Code đã lên mây ☁️".to_string()
                                } else {
                                    "✅ Commit & Push successful! Code is in the cloud ☁️"
                                        .to_string()
                                });
                            }
                            Err(err) => {
                                app.go_step = GoStep::Done(format!(
                                    "{} {}",
                                    if is_vi {
                                        "❌ Push thất bại:"
                                    } else {
                                        "❌ Push failed:"
                                    },
                                    err
                                ));
                            }
                        }
                    } else {
                        app.go_step = GoStep::Done(if is_vi {
                            "✅ Commit thành công (Đã bỏ qua Tự động Push)!".to_string()
                        } else {
                            "✅ Commit successful (Auto Push disabled)!".to_string()
                        });
                    }
                }
                app.refresh_git_status();
                continue;
            }
        }

        // Handle AmendStep::Pushing: run git amend outside of event poll
        if app.active_modal == ActiveModal::AmendCommit {
            if let AmendStep::Pushing = &app.amend_step {
                let is_vi = app.current_lang == "vi";
                let msg = app.amend_message.clone();
                app.amend_step = match amend_commit(&msg) {
                    Ok(_) => AmendStep::Done(if is_vi {
                        "✅ Đã sửa commit cuối! (Amend thành công)".to_string()
                    } else {
                        "✅ Last commit amended successfully!".to_string()
                    }),
                    Err(err) => AmendStep::Done(format!("❌ Amend failed: {}", err)),
                };
                app.refresh_git_status();
                continue;
            }
        }

        // Handle KILO commit generation (long-running CLI call)
        if app.active_modal == ActiveModal::DiffResult && app.kilo_generating {
            let diff = app.last_staged_diff.clone();
            if diff.trim().is_empty() {
                app.kilo_generation_status = if app.current_lang == "vi" {
                    "❌ Không có diff staged.".to_string()
                } else {
                    "❌ No staged diff.".to_string()
                };
                app.kilo_generating = false;
            } else {
                match app.try_generate_with_kilo(&diff) {
                    Ok(msg) => {
                        app.diff_kilo_generated = msg;
                        app.kilo_generation_status = if app.current_lang == "vi" {
                            "✅ KILO đã sinh message xong!".to_string()
                        } else {
                            "✅ KILO finished generating!".to_string()
                        };
                    }
                    Err(e) => {
                        app.kilo_generation_status = format!("❌ {}", e);
                    }
                }
                app.kilo_generating = false;
            }
            continue;
        }

        if app.github_cloning {
            let temp_dir = match tempfile::Builder::new()
                .prefix("git_ai_download_")
                .tempdir()
            {
                Ok(dir) => dir,
                Err(e) => {
                    app.github_cloning_error = Some(format!("Không thể tạo thư mục tạm: {}", e));
                    app.github_cloning = false;
                    continue;
                }
            };
            let temp_path = temp_dir.path().to_path_buf();
            let output = Command::new("git")
                .args([
                    "clone",
                    "--depth",
                    "1",
                    "--filter=blob:none",
                    "--no-checkout",
                    &app.github_download_url,
                    temp_path.to_str().unwrap_or_default(),
                ])
                .output();
            match output {
                Ok(out) if out.status.success() => {
                    app.github_temp_dir = Some(temp_dir);
                    if let Some(ref dir) = app.github_temp_dir {
                        if let Ok(output) = Command::new("git")
                            .args(["symbolic-ref", "--short", "HEAD"])
                            .current_dir(dir.path())
                            .output()
                        {
                            if output.status.success() {
                                app.current_github_branch = String::from_utf8_lossy(&output.stdout).trim().to_string();
                            } else {
                                app.current_github_branch = "main".to_string();
                            }
                        } else {
                            app.current_github_branch = "main".to_string();
                        }
                    }
                    if let Err(e) = app.visit_repo_dir() {
                        app.github_cloning_error = Some(format!("{}", e));
                        app.github_cloning = false;
                        app.github_temp_dir = None;
                        app.active_modal = ActiveModal::GithubDownloadUrlInput;
                    } else {
                        let url = app.github_download_url.trim().to_string();
                        app.add_to_github_history(&url);
                        app.selected_github_history_index = None;
                        app.github_cloning = false;
                        app.selected_github_tree_index = 0;
                        app.active_modal = ActiveModal::GithubDownloadTree;
                    }
                }
                Ok(out) => {
                    let stderr = String::from_utf8_lossy(&out.stderr).trim().to_string();
                    app.github_cloning_error = Some(stderr);
                    app.github_cloning = false;
                    app.github_temp_dir = None;
                    app.active_modal = ActiveModal::GithubDownloadUrlInput;
                }
                Err(err) => {
                    app.github_cloning_error = Some(format!("{}", err));
                    app.github_cloning = false;
                    app.github_temp_dir = None;
                    app.active_modal = ActiveModal::GithubDownloadUrlInput;
                }
            }
            continue;
        }

        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                // Intercept keys if a modal is active
                match &app.active_modal {
                    ActiveModal::Help => {
                        match key.code {
                            KeyCode::Esc
                            | KeyCode::Char(' ')
                            | KeyCode::Enter
                            | KeyCode::Char('q')
                            | KeyCode::Char('?')
                            | KeyCode::Char('h') => {
                                app.active_modal = ActiveModal::None;
                            }
                            _ => {}
                        }
                        continue;
                    }
                    ActiveModal::LanguageSelect => {
                        match key.code {
                            KeyCode::Esc | KeyCode::Char('q') => {
                                app.active_modal = ActiveModal::None;
                            }
                            KeyCode::Char('v') | KeyCode::Char('V') => {
                                let locales = crate::cli::Locales::new(&app.current_lang);
                                if let Ok(msg) = crate::cli::system::handle_lang("vi", &locales) {
                                    app.status_message = msg;
                                    app.current_lang = Helper::get_ai_language();
                                }
                                app.active_modal = ActiveModal::None;
                            }
                            KeyCode::Char('e') | KeyCode::Char('E') => {
                                let locales = crate::cli::Locales::new(&app.current_lang);
                                if let Ok(msg) = crate::cli::system::handle_lang("en", &locales) {
                                    app.status_message = msg;
                                    app.current_lang = Helper::get_ai_language();
                                }
                                app.active_modal = ActiveModal::None;
                            }
                            KeyCode::Char('a') | KeyCode::Char('A') => {
                                let locales = crate::cli::Locales::new(&app.current_lang);
                                if let Ok(msg) = crate::cli::system::handle_lang("auto", &locales) {
                                    app.status_message = msg;
                                    app.current_lang = Helper::get_ai_language();
                                }
                                app.active_modal = ActiveModal::None;
                            }
                            KeyCode::Up | KeyCode::Char('k') => {
                                if app.selected_lang_index > 0 {
                                    app.selected_lang_index -= 1;
                                } else {
                                    app.selected_lang_index = 2;
                                }
                            }
                            KeyCode::Down | KeyCode::Char('j') => {
                                if app.selected_lang_index < 2 {
                                    app.selected_lang_index += 1;
                                } else {
                                    app.selected_lang_index = 0;
                                }
                            }
                            KeyCode::Enter => {
                                let selection = match app.selected_lang_index {
                                    0 => "vi",
                                    1 => "en",
                                    _ => "auto",
                                };
                                let locales = crate::cli::Locales::new(&app.current_lang);
                                if let Ok(msg) =
                                    crate::cli::system::handle_lang(selection, &locales)
                                {
                                    app.status_message = msg;
                                    app.current_lang = Helper::get_ai_language();
                                }
                                app.active_modal = ActiveModal::None;
                            }
                            _ => {}
                        }
                        continue;
                    }
                    ActiveModal::ThemeSelect => {
                        match key.code {
                            KeyCode::Esc | KeyCode::Char('q') => {
                                app.active_modal = ActiveModal::None;
                            }
                            KeyCode::Char('d') | KeyCode::Char('D') => {
                                app.is_light_theme = false;
                                let _ = Command::new("git")
                                    .args(["config", "--global", "git-ai.theme", "dark"])
                                    .output();
                                app.refresh_git_status();
                                app.status_message = if app.current_lang == "vi" {
                                    "🎨 Đã chuyển sang giao diện Tối (Dracula)".to_string()
                                } else {
                                    "🎨 Switched to Dracula (Dark) theme".to_string()
                                };
                                app.active_modal = ActiveModal::None;
                            }
                            KeyCode::Char('l') | KeyCode::Char('L') => {
                                app.is_light_theme = true;
                                let _ = Command::new("git")
                                    .args(["config", "--global", "git-ai.theme", "light"])
                                    .output();
                                app.refresh_git_status();
                                app.status_message = if app.current_lang == "vi" {
                                    "🎨 Đã chuyển sang giao diện Sáng (Premium Light)".to_string()
                                } else {
                                    "🎨 Switched to Premium Light theme".to_string()
                                };
                                app.active_modal = ActiveModal::None;
                            }
                            KeyCode::Up
                            | KeyCode::Char('k')
                            | KeyCode::Down
                            | KeyCode::Char('j') => {
                                app.selected_theme_index = 1 - app.selected_theme_index;
                            }
                            KeyCode::Enter => {
                                let selection = match app.selected_theme_index {
                                    0 => "dark",
                                    _ => "light",
                                };
                                app.is_light_theme = selection == "light";
                                let _ = Command::new("git")
                                    .args(["config", "--global", "git-ai.theme", selection])
                                    .output();
                                app.refresh_git_status();
                                app.status_message = if app.current_lang == "vi" {
                                    format!(
                                        "🎨 Đã chuyển sang giao diện {}",
                                        if app.is_light_theme {
                                            "Sáng (Premium Light)"
                                        } else {
                                            "Tối (Dracula)"
                                        }
                                    )
                                } else {
                                    format!(
                                        "🎨 Switched to {} theme",
                                        if app.is_light_theme {
                                            "Premium Light"
                                        } else {
                                            "Dracula (Dark)"
                                        }
                                    )
                                };
                                app.active_modal = ActiveModal::None;
                            }
                            _ => {}
                        }
                        continue;
                    }
                    ActiveModal::Settings => {
                        let is_vi = app.current_lang == "vi";
                        match key.code {
                            KeyCode::Esc | KeyCode::Char('q') => {
                                app.active_modal = ActiveModal::None;
                            }
                            KeyCode::Up | KeyCode::Char('k') => {
                                if app.selected_setting_index > 0 {
                                    app.selected_setting_index -= 1;
                                } else {
                                    app.selected_setting_index = 2;
                                }
                            }
                            KeyCode::Down | KeyCode::Char('j') => {
                                if app.selected_setting_index < 2 {
                                    app.selected_setting_index += 1;
                                } else {
                                    app.selected_setting_index = 0;
                                }
                            }
                            KeyCode::Char(' ') | KeyCode::Enter => {
                                match app.selected_setting_index {
                                    0 => {
                                        app.auto_push = !app.auto_push;
                                        let val = app.auto_push.to_string();
                                        let _ = Command::new("git")
                                            .args(["config", "--global", "git-ai.auto-push", &val])
                                            .output();
                                        app.status_message = if is_vi {
                                            format!(
                                                "⚙️ Tự động Push: {}",
                                                if app.auto_push { "BẬT" } else { "TẮT" }
                                            )
                                        } else {
                                            format!(
                                                "⚙️ Auto Push: {}",
                                                if app.auto_push { "ON" } else { "OFF" }
                                            )
                                        };
                                    }
                                    1 => {
                                        app.auto_stage_all = !app.auto_stage_all;
                                        let val = app.auto_stage_all.to_string();
                                        let _ = Command::new("git")
                                            .args([
                                                "config",
                                                "--global",
                                                "git-ai.auto-stage-all",
                                                &val,
                                            ])
                                            .output();
                                        app.status_message = if is_vi {
                                            format!(
                                                "⚙️ Tự động Stage tất cả: {}",
                                                if app.auto_stage_all { "BẬT" } else { "TẮT" }
                                            )
                                        } else {
                                            format!(
                                                "⚙️ Auto Stage All: {}",
                                                if app.auto_stage_all { "ON" } else { "OFF" }
                                            )
                                        };
                                    }
                                    2 => {
                                        app.kilo_ai_enabled = !app.kilo_ai_enabled;
                                        let val = app.kilo_ai_enabled.to_string();
                                        let _ = Command::new("git")
                                            .args(["config", "--global", "git-ai.kilo-ai", &val])
                                            .output();
                                        app.status_message = if is_vi {
                                            format!(
                                                "⚙️ Kilo AI: {}",
                                                if app.kilo_ai_enabled {
                                                    "BẬT"
                                                } else {
                                                    "TẮT"
                                                }
                                            )
                                        } else {
                                            format!(
                                                "⚙️ Kilo AI Generation: {}",
                                                if app.kilo_ai_enabled { "ON" } else { "OFF" }
                                            )
                                        };
                                    }
                                    _ => {}
                                }
                            }
                            _ => {}
                        }
                        continue;
                    }
                    ActiveModal::WorkspaceHistory => {
                        let is_vi = app.current_lang == "vi";
                        match key.code {
                            KeyCode::Esc | KeyCode::Char('q') => {
                                app.active_modal = ActiveModal::None;
                            }
                            KeyCode::Up | KeyCode::Char('k') => {
                                if !app.workspace_history.is_empty()
                                    && app.selected_workspace_index > 0
                                {
                                    app.selected_workspace_index -= 1;
                                }
                            }
                            KeyCode::Down | KeyCode::Char('j') => {
                                if !app.workspace_history.is_empty()
                                    && app.selected_workspace_index
                                        < app.workspace_history.len() - 1
                                {
                                    app.selected_workspace_index += 1;
                                }
                            }
                            KeyCode::Enter => {
                                if !app.workspace_history.is_empty() {
                                    let selected_path =
                                        app.workspace_history[app.selected_workspace_index].clone();
                                    if std::env::set_current_dir(&selected_path).is_ok() {
                                        app.current_dir = selected_path.clone();
                                        app.add_to_workspace_history(&selected_path);
                                        app.refresh_git_status();
                                        app.status_message = if is_vi {
                                            format!("🔄 Đã chuyển sang Project: {}", selected_path)
                                        } else {
                                            format!("🔄 Switched to project: {}", selected_path)
                                        };
                                        app.active_modal = ActiveModal::None;
                                    } else {
                                        app.status_message = if is_vi {
                                            "❌ Lỗi: Không thể truy cập thư mục này.".to_string()
                                        } else {
                                            "❌ Error: Cannot access this folder.".to_string()
                                        };
                                    }
                                }
                            }
                            KeyCode::Char('n') | KeyCode::Char('N') => {
                                let dialog_title = if is_vi {
                                    "Chọn thư mục Project mới"
                                } else {
                                    "Select New Project Folder"
                                };
                                if let Some(folder) =
                                    rfd::FileDialog::new().set_title(dialog_title).pick_folder()
                                {
                                    if std::env::set_current_dir(&folder).is_ok() {
                                        let folder_str = folder.display().to_string();
                                        app.current_dir = folder_str.clone();
                                        app.add_to_workspace_history(&folder_str);
                                        app.refresh_git_status();
                                        app.status_message = if is_vi {
                                            "🔄 Đã tải Project mới thành công!".to_string()
                                        } else {
                                            "🔄 Loaded new Project successfully!".to_string()
                                        };
                                        app.active_modal = ActiveModal::None;
                                    } else {
                                        app.status_message = if is_vi {
                                            "❌ Lỗi: Không thể truy cập thư mục này.".to_string()
                                        } else {
                                            "❌ Error: Cannot access this folder.".to_string()
                                        };
                                    }
                                } else {
                                    app.status_message = if is_vi {
                                        "ℹ️ Đã hủy chọn Project.".to_string()
                                    } else {
                                        "ℹ️ Project selection cancelled.".to_string()
                                    };
                                }
                            }
                            KeyCode::Char('x') | KeyCode::Char('X') => {
                                if !app.workspace_history.is_empty() {
                                    let removed_path =
                                        app.workspace_history[app.selected_workspace_index].clone();
                                    // Don't allow removing the currently active workspace
                                    if removed_path == app.current_dir {
                                        app.status_message = if is_vi {
                                            "⚠️ Không thể xóa workspace đang hoạt động!".to_string()
                                        } else {
                                            "⚠️ Cannot remove the currently active workspace!"
                                                .to_string()
                                        };
                                    } else {
                                        app.remove_from_workspace_history(
                                            app.selected_workspace_index,
                                        );
                                        app.status_message = if is_vi {
                                            format!("🗑️ Đã xóa khỏi lịch sử: {}", removed_path)
                                        } else {
                                            format!("🗑️ Removed from history: {}", removed_path)
                                        };
                                    }
                                }
                            }
                            _ => {}
                        }
                        continue;
                    }
                    ActiveModal::RevertConfirm(path) => {
                        match key.code {
                            KeyCode::Char('y') | KeyCode::Char('Y') | KeyCode::Enter => {
                                let path_to_revert = path.clone();
                                let is_untracked = app.files.iter().any(|f| {
                                    f.path == path_to_revert
                                        && (f.status.starts_with("??") || f.status.contains("??"))
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
                                    // For tracked files, first unstage, then restore
                                    let _ = unstage_file(&path_to_revert);
                                    let success = revert_file(&path_to_revert).is_ok();
                                    if success {
                                        app.status_message = if app.current_lang == "vi" {
                                            format!("🔄 Đã khôi phục file: {}", path_to_revert)
                                        } else {
                                            format!(
                                                "🔄 Reverted changes in file: {}",
                                                path_to_revert
                                            )
                                        };
                                    } else {
                                        app.status_message = if app.current_lang == "vi" {
                                            format!("❌ Lỗi khi khôi phục file: {}", path_to_revert)
                                        } else {
                                            format!("❌ Failed to revert file: {}", path_to_revert)
                                        };
                                    }
                                }
                                app.active_modal = ActiveModal::None;
                                app.refresh_git_status();
                            }
                            KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                                app.active_modal = ActiveModal::None;
                                app.status_message = if app.current_lang == "vi" {
                                    "ℹ️ Đã hủy khôi phục file.".to_string()
                                } else {
                                    "ℹ️ Revert cancelled.".to_string()
                                };
                            }
                            _ => {}
                        }
                        continue;
                    }
                    ActiveModal::GitLog => {
                        match key.code {
                            KeyCode::Esc
                            | KeyCode::Char('q')
                            | KeyCode::Char('v')
                            | KeyCode::Char('V') => {
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
                            KeyCode::Enter => {
                                if !app.commit_logs.is_empty()
                                    && app.selected_log_index < app.commit_logs.len()
                                {
                                    let hash = app.commit_logs[app.selected_log_index].hash.clone();
                                    app.fetch_commit_diff(&hash);
                                    app.active_modal = ActiveModal::CommitDiff(hash);
                                }
                            }
                            _ => {}
                        }
                        continue;
                    }
                    ActiveModal::BranchSelect => {
                        match key.code {
                            KeyCode::Esc
                            | KeyCode::Char('q')
                            | KeyCode::Char('b')
                            | KeyCode::Char('B') => {
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
                            KeyCode::Char('m') | KeyCode::Char('M') => {
                                if !app.branches.is_empty()
                                    && app.selected_branch_index < app.branches.len()
                                {
                                    let branch_name =
                                        app.branches[app.selected_branch_index].name.clone();
                                    if branch_name != app.current_branch {
                                        app.active_modal = ActiveModal::MergeConfirm(branch_name);
                                    } else {
                                        app.status_message = if app.current_lang == "vi" {
                                            "❌ Không thể merge chi nhánh hiện tại vào chính nó!"
                                                .to_string()
                                        } else {
                                            "❌ Cannot merge current branch into itself!"
                                                .to_string()
                                        };
                                    }
                                }
                            }
                            KeyCode::Enter => {
                                if !app.branches.is_empty()
                                    && app.selected_branch_index < app.branches.len()
                                {
                                    let branch_name =
                                        app.branches[app.selected_branch_index].name.clone();

                                    match checkout_branch(&branch_name) {
                                        Ok(_) => {
                                            app.status_message = if app.current_lang == "vi" {
                                                format!(
                                                    "🌿 Đã chuyển sang chi nhánh: {}",
                                                    branch_name
                                                )
                                            } else {
                                                format!("🌿 Checked out branch: {}", branch_name)
                                            };
                                        }
                                        Err(err) => {
                                            app.status_message = if app.current_lang == "vi" {
                                                format!("❌ Lỗi chuyển chi nhánh: {}", err)
                                            } else {
                                                format!("❌ Checkout failed: {}", err)
                                            };
                                        }
                                    }
                                    app.active_modal = ActiveModal::None;
                                    app.refresh_git_status();
                                }
                            }
                            _ => {}
                        }
                        continue;
                    }
                    ActiveModal::MergeConfirm(branch_name) => {
                        let branch_name = branch_name.clone();
                        match key.code {
                            KeyCode::Esc | KeyCode::Char('n') | KeyCode::Char('N') => {
                                app.active_modal = ActiveModal::BranchSelect;
                            }
                            KeyCode::Enter | KeyCode::Char('y') | KeyCode::Char('Y') => {
                                app.status_message = if app.current_lang == "vi" {
                                    format!("⚡ Đang merge chi nhánh {}...", branch_name)
                                } else {
                                    format!("⚡ Merging branch {}...", branch_name)
                                };
                                match git_merge(&branch_name) {
                                    Ok(out) => {
                                        app.status_message = if app.current_lang == "vi" {
                                            format!("✅ Đã merge thành công: {}", out)
                                        } else {
                                            format!("✅ Merge successful: {}", out)
                                        };
                                    }
                                    Err(err) => {
                                        app.status_message = if app.current_lang == "vi" {
                                            format!("❌ Lỗi merge: {}", err)
                                        } else {
                                            format!("❌ Merge failed: {}", err)
                                        };
                                    }
                                }
                                app.active_modal = ActiveModal::None;
                                app.refresh_git_status();
                            }
                            _ => {}
                        }
                        continue;
                    }
                    ActiveModal::NewBranchInput => {
                        match key.code {
                            KeyCode::Esc => {
                                app.active_modal = ActiveModal::BranchSelect;
                            }
                            KeyCode::Enter => {
                                let branch_name = app.new_branch_name.trim().to_string();
                                if !branch_name.is_empty() {
                                    app.status_message = if app.current_lang == "vi" {
                                        format!("⚡ Đang tạo chi nhánh {}...", branch_name)
                                    } else {
                                        format!("⚡ Creating branch {}...", branch_name)
                                    };
                                    match create_and_checkout_branch(&branch_name) {
                                        Ok(_) => {
                                            app.status_message = if app.current_lang == "vi" {
                                                format!(
                                                    "🌿 Đã tạo và chuyển sang chi nhánh mới: {}",
                                                    branch_name
                                                )
                                            } else {
                                                format!(
                                                    "🌿 Created and checked out new branch: {}",
                                                    branch_name
                                                )
                                            };
                                            app.active_modal = ActiveModal::None;
                                        }
                                        Err(err) => {
                                            app.status_message = if app.current_lang == "vi" {
                                                format!("❌ Lỗi tạo chi nhánh: {}", err)
                                            } else {
                                                format!("❌ Failed to create branch: {}", err)
                                            };
                                            app.active_modal = ActiveModal::BranchSelect;
                                        }
                                    }
                                    app.refresh_git_status();
                                } else {
                                    app.status_message = if app.current_lang == "vi" {
                                        "❌ Tên chi nhánh không được để trống!".to_string()
                                    } else {
                                        "❌ Branch name cannot be empty!".to_string()
                                    };
                                }
                            }
                            KeyCode::Backspace => {
                                app.new_branch_name.pop();
                            }
                            KeyCode::Char(c) => {
                                app.new_branch_name.push(c);
                            }
                            _ => {}
                        }
                        continue;
                    }
                    ActiveModal::DiffResult => {
                        if !app.diff_kilo_generated.is_empty() {
                            match key.code {
                                KeyCode::Char('c') | KeyCode::Char('C') => {
                                    if let Ok(mut cb) = arboard::Clipboard::new() {
                                        let _ = cb.set_text(app.diff_kilo_generated.clone());
                                    }
                                    app.status_message = if app.current_lang == "vi" {
                                        "✅ Đã copy commit message từ KILO vào clipboard."
                                            .to_string()
                                    } else {
                                        "✅ KILO commit message copied to clipboard.".to_string()
                                    };
                                }
                                KeyCode::Enter | KeyCode::Char('g') | KeyCode::Char('G') => {
                                    app.commit_message_preview = app.diff_kilo_generated.clone();
                                    app.active_modal = ActiveModal::None;
                                    app.status_message = if app.current_lang == "vi" {
                                        "✅ Sử dụng message từ KILO. Nhấn [G] để commit & push."
                                            .to_string()
                                    } else {
                                        "✅ Using KILO message. Press [G] to commit & push."
                                            .to_string()
                                    };
                                }
                                KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('d') => {
                                    app.diff_kilo_generated.clear();
                                    app.active_modal = ActiveModal::None;
                                }
                                _ => {}
                            }
                        } else {
                            match key.code {
                                KeyCode::Char('k') | KeyCode::Char('K') => {
                                    if !app.kilo_ai_enabled {
                                        app.kilo_generation_status = if app.current_lang == "vi" {
                                            "⚠️ Tính năng Kilo AI đã bị tắt trong Cài đặt!"
                                                .to_string()
                                        } else {
                                            "⚠️ Kilo AI Generation is disabled in Settings!"
                                                .to_string()
                                        };
                                    } else if app.last_staged_diff.trim().is_empty() {
                                        app.kilo_generation_status = if app.current_lang == "vi" {
                                            "⚠️ Không có diff staged.".to_string()
                                        } else {
                                            "⚠️ No staged diff.".to_string()
                                        };
                                    } else {
                                        app.kilo_generating = true;
                                        app.kilo_generation_status = if app.current_lang == "vi" {
                                            "⏳ Đang hỏi KILO...".to_string()
                                        } else {
                                            "⏳ Asking KILO...".to_string()
                                        };
                                        app.diff_kilo_generated.clear();
                                    }
                                }
                                KeyCode::Char('m') | KeyCode::Char('M') => {
                                    app.status_message = if app.current_lang == "vi" {
                                        "⏳ Đang tải danh sách model Kilo...".to_string()
                                    } else {
                                        "⏳ Fetching Kilo model list...".to_string()
                                    };
                                    terminal.draw(|f| crate::ui::ui(f, app))?;
                                    app.fetch_kilo_models();
                                    app.kilo_model_filter.clear();
                                    app.kilo_model_search_mode = false;
                                    app.selected_kilo_model_index = 0;
                                    if !app.current_kilo_model.is_empty() {
                                        if let Some(idx) = app
                                            .kilo_models
                                            .iter()
                                            .position(|m| m == &app.current_kilo_model)
                                        {
                                            app.selected_kilo_model_index = idx;
                                        }
                                    }
                                    app.active_modal = ActiveModal::KiloModelSelect;
                                }
                                KeyCode::Esc
                                | KeyCode::Enter
                                | KeyCode::Char('q')
                                | KeyCode::Char('d') => {
                                    app.active_modal = ActiveModal::None;
                                }
                                _ => {}
                            }
                        }
                        continue;
                    }
                    ActiveModal::KiloModelSelect => {
                        let filtered: Vec<String> = if app.kilo_model_filter.is_empty() {
                            app.kilo_models.clone()
                        } else {
                            let f = app.kilo_model_filter.to_lowercase();
                            app.kilo_models
                                .iter()
                                .filter(|m| m.to_lowercase().contains(&f))
                                .cloned()
                                .collect()
                        };

                        // Keep selected index within filtered bounds
                        if !filtered.is_empty() && app.selected_kilo_model_index >= filtered.len() {
                            app.selected_kilo_model_index = filtered.len() - 1;
                        }

                        match key.code {
                            KeyCode::Esc | KeyCode::Char('q') => {
                                if app.kilo_model_search_mode || !app.kilo_model_filter.is_empty() {
                                    app.kilo_model_filter.clear();
                                    app.kilo_model_search_mode = false;
                                    app.selected_kilo_model_index = 0;
                                } else {
                                    app.active_modal = ActiveModal::DiffResult;
                                }
                            }
                            KeyCode::Enter => {
                                if !filtered.is_empty() {
                                    let idx = app.selected_kilo_model_index.min(filtered.len() - 1);
                                    app.current_kilo_model = filtered[idx].clone();
                                }
                                app.kilo_model_filter.clear();
                                app.kilo_model_search_mode = false;
                                app.active_modal = ActiveModal::DiffResult;
                            }
                            KeyCode::Up | KeyCode::Char('k') => {
                                if app.selected_kilo_model_index > 0 {
                                    app.selected_kilo_model_index -= 1;
                                }
                            }
                            KeyCode::Down | KeyCode::Char('j') => {
                                if !filtered.is_empty()
                                    && app.selected_kilo_model_index + 1 < filtered.len()
                                {
                                    app.selected_kilo_model_index += 1;
                                }
                            }
                            KeyCode::Char('/') => {
                                app.kilo_model_search_mode = true;
                            }
                            KeyCode::Backspace => {
                                if app.kilo_model_search_mode && !app.kilo_model_filter.is_empty() {
                                    app.kilo_model_filter.pop();
                                    app.selected_kilo_model_index = 0;
                                }
                            }
                            KeyCode::Char(c) => {
                                if c.is_ascii_alphanumeric() || c == '-' || c == '.' {
                                    if !app.kilo_model_search_mode {
                                        app.kilo_model_search_mode = true;
                                    }
                                    app.kilo_model_filter.push(c);
                                    app.selected_kilo_model_index = 0;
                                }
                            }
                            _ => {}
                        }
                        continue;
                    }
                    ActiveModal::ManualCommit => {
                        match key.code {
                            KeyCode::Esc => {
                                app.active_modal = ActiveModal::None;
                            }
                            KeyCode::Enter => {
                                let msg = app.manual_commit_message.trim().to_string();
                                if msg.is_empty() {
                                    app.status_message = if app.current_lang == "vi" {
                                        "❌ Commit message không được để trống!".to_string()
                                    } else {
                                        "❌ Commit message cannot be empty!".to_string()
                                    };
                                } else if app.staged_count == 0 {
                                    app.status_message = if app.current_lang == "vi" {
                                        "⚠️ Chưa có file nào staged! Hãy nhấn [Space] để stage trước.".to_string()
                                    } else {
                                        "⚠️ No files staged! Press [Space] to stage first."
                                            .to_string()
                                    };
                                } else {
                                    match crate::git::commit::commit(&msg) {
                                        Ok(_) => {
                                            app.status_message = if app.current_lang == "vi" {
                                                format!("✅ Đã commit thủ công: {}", msg)
                                            } else {
                                                format!("✅ Manual commit successful: {}", msg)
                                            };
                                            app.active_modal = ActiveModal::None;
                                            app.manual_commit_message.clear();
                                            app.refresh_git_status();
                                        }
                                        Err(e) => {
                                            app.status_message = if app.current_lang == "vi" {
                                                format!("❌ Commit thất bại: {}", e)
                                            } else {
                                                format!("❌ Commit failed: {}", e)
                                            };
                                        }
                                    }
                                }
                            }
                            KeyCode::Backspace => {
                                app.manual_commit_message.pop();
                            }
                            KeyCode::Char(c) => {
                                app.manual_commit_message.push(c);
                            }
                            _ => {}
                        }
                        continue;
                    }
                    ActiveModal::GitMenu => {
                        let max = 13;

                        match key.code {
                            KeyCode::Char('g') | KeyCode::Char('G') => {
                                if !app.kilo_ai_enabled {
                                    app.status_message = if app.current_lang == "vi" {
                                        "⚠️ Tính năng Kilo AI đã bị tắt trong Cài đặt!".to_string()
                                    } else {
                                        "⚠️ Kilo AI Generation is disabled in Settings!".to_string()
                                    };
                                    continue;
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
                                app.active_modal = ActiveModal::GoConfirm;
                                continue;
                            }
                            KeyCode::Char('n') | KeyCode::Char('N') => {
                                app.github_download_url.clear();
                                app.github_cloning_error = None;
                                app.github_cloning = false;
                                app.active_modal = ActiveModal::GithubDownloadUrlInput;
                                continue;
                            }
                            KeyCode::Char('c') | KeyCode::Char('C') => {
                                app.manual_commit_message.clear();
                                app.auto_stage_all_if_enabled();
                                app.active_modal = ActiveModal::ManualCommit;
                                continue;
                            }
                            KeyCode::Char('m') | KeyCode::Char('M') => {
                                app.fetch_amend_msg();
                                app.active_modal = ActiveModal::AmendCommit;
                                continue;
                            }
                            KeyCode::Char('f') | KeyCode::Char('F') => {
                                app.status_message = if app.current_lang == "vi" {
                                    "⏳ Đang tải thông tin mới từ Remote (Fetch)..."
                                } else {
                                    "⏳ Fetching new updates from Remote..."
                                }
                                .to_string();
                                terminal.draw(|f| crate::ui::ui(f, app))?;
                                let _ = crate::git::remote::git_fetch();
                                app.status_message = if app.current_lang == "vi" {
                                    "✅ Fetch hoàn tất"
                                } else {
                                    "✅ Fetch completed"
                                }
                                .to_string();
                                app.active_modal = ActiveModal::None;
                                app.refresh_git_status();
                                continue;
                            }
                            KeyCode::Char('p') | KeyCode::Char('P') => {
                                app.status_message = if app.current_lang == "vi" {
                                    "⏳ Đang cập nhật thay đổi từ Remote (Pull)..."
                                } else {
                                    "⏳ Pulling changes from Remote..."
                                }
                                .to_string();
                                terminal.draw(|f| crate::ui::ui(f, app))?;
                                let _ = crate::git::remote::git_pull();
                                app.status_message = if app.current_lang == "vi" {
                                    "✅ Pull hoàn tất"
                                } else {
                                    "✅ Pull completed"
                                }
                                .to_string();
                                app.active_modal = ActiveModal::None;
                                app.refresh_git_status();
                                continue;
                            }
                            KeyCode::Char('u') | KeyCode::Char('U') => {
                                app.status_message = if app.current_lang == "vi" {
                                    "⏳ Đang đẩy các thay đổi lên Remote (Push)..."
                                } else {
                                    "⏳ Pushing committed changes to Remote..."
                                }
                                .to_string();
                                terminal.draw(|f| crate::ui::ui(f, app))?;
                                match crate::git::remote::git_push() {
                                    Ok(_) => {
                                        app.status_message = if app.current_lang == "vi" {
                                            "✅ Push thành công"
                                        } else {
                                            "✅ Push successful"
                                        }
                                        .to_string();
                                    }
                                    Err(e) => {
                                        app.status_message = format!("❌ Push failed: {}", e);
                                    }
                                }
                                app.active_modal = ActiveModal::None;
                                app.refresh_git_status();
                                continue;
                            }
                            KeyCode::Char('i') | KeyCode::Char('I') => {
                                app.fetch_remote_info();
                                app.active_modal = ActiveModal::RemoteInfo;
                                continue;
                            }
                            KeyCode::Char('b') | KeyCode::Char('B') => {
                                app.fetch_branches();
                                app.active_modal = ActiveModal::BranchSelect;
                                continue;
                            }
                            KeyCode::Char('s') | KeyCode::Char('S') => {
                                app.fetch_stash();
                                app.active_modal = ActiveModal::StashList;
                                continue;
                            }
                            KeyCode::Char('v') | KeyCode::Char('V') => {
                                app.fetch_commit_logs();
                                app.active_modal = ActiveModal::GitLog;
                                continue;
                            }
                            KeyCode::Char('t') | KeyCode::Char('T') => {
                                app.fetch_commit_tree();
                                app.selected_log_index = 0;
                                if !app.commit_logs.is_empty() {
                                    let hash = app.commit_logs[0].hash.clone();
                                    app.fetch_commit_diff(&hash);
                                }
                                app.active_modal = ActiveModal::CommitTree;
                                continue;
                            }
                            KeyCode::Char('e') | KeyCode::Char('E') => {
                                app.compute_feature_groups();
                                app.active_modal = ActiveModal::FeatureCommit;
                                continue;
                            }
                            KeyCode::Char('y') | KeyCode::Char('Y') => {
                                app.selected_setting_index = 0;
                                app.active_modal = ActiveModal::Settings;
                                continue;
                            }
                            _ => {}
                        }

                        match key.code {
                            KeyCode::Esc | KeyCode::Char('q') => {
                                app.active_modal = ActiveModal::None;
                            }
                            KeyCode::Enter => {
                                match app.selected_git_action {
                                    0 => {
                                        // AI Commit & Push
                                        if !app.kilo_ai_enabled {
                                            app.status_message = if app.current_lang == "vi" {
                                                "⚠️ Tính năng Kilo AI đã bị tắt trong Cài đặt!"
                                                    .to_string()
                                            } else {
                                                "⚠️ Kilo AI Generation is disabled in Settings!"
                                                    .to_string()
                                            };
                                        } else {
                                            let clipboard_msg =
                                                if let Ok(mut cb) = arboard::Clipboard::new() {
                                                    cb.get_text().unwrap_or_default()
                                                } else {
                                                    String::new()
                                                };
                                            app.commit_message_preview =
                                                if clipboard_msg.trim().is_empty() {
                                                    if app.current_lang == "vi" {
                                                        "(Chưa có commit message trong clipboard)"
                                                            .to_string()
                                                    } else {
                                                        "(No commit message in clipboard)"
                                                            .to_string()
                                                    }
                                                } else {
                                                    clipboard_msg.trim().to_string()
                                                };
                                            app.go_step = GoStep::Confirm;
                                            app.auto_stage_all_if_enabled();
                                            app.active_modal = ActiveModal::GoConfirm;
                                        }
                                    }
                                    1 => {
                                        // Manual Commit
                                        app.manual_commit_message.clear();
                                        app.auto_stage_all_if_enabled();
                                        app.active_modal = ActiveModal::ManualCommit;
                                    }
                                    2 => {
                                        // Amend
                                        app.fetch_amend_msg();
                                        app.active_modal = ActiveModal::AmendCommit;
                                    }
                                    3 => {
                                        // Fetch
                                        app.status_message = if app.current_lang == "vi" {
                                            "⏳ Đang tải thông tin mới từ Remote (Fetch)..."
                                        } else {
                                            "⏳ Fetching new updates from Remote..."
                                        }
                                        .to_string();
                                        terminal.draw(|f| crate::ui::ui(f, app))?;
                                        let _ = crate::git::remote::git_fetch();
                                        app.status_message = if app.current_lang == "vi" {
                                            "✅ Fetch hoàn tất"
                                        } else {
                                            "✅ Fetch completed"
                                        }
                                        .to_string();
                                        app.active_modal = ActiveModal::None;
                                        app.refresh_git_status();
                                    }
                                    4 => {
                                        // Pull
                                        app.status_message = if app.current_lang == "vi" {
                                            "⏳ Đang cập nhật thay đổi từ Remote (Pull)..."
                                        } else {
                                            "⏳ Pulling changes from Remote..."
                                        }
                                        .to_string();
                                        terminal.draw(|f| crate::ui::ui(f, app))?;
                                        let _ = crate::git::remote::git_pull();
                                        app.status_message = if app.current_lang == "vi" {
                                            "✅ Pull hoàn tất"
                                        } else {
                                            "✅ Pull completed"
                                        }
                                        .to_string();
                                        app.active_modal = ActiveModal::None;
                                        app.refresh_git_status();
                                    }
                                    5 => {
                                        // Push
                                        app.status_message = if app.current_lang == "vi" {
                                            "⏳ Đang đẩy các thay đổi lên Remote (Push)..."
                                        } else {
                                            "⏳ Pushing committed changes to Remote..."
                                        }
                                        .to_string();
                                        terminal.draw(|f| crate::ui::ui(f, app))?;
                                        match crate::git::remote::git_push() {
                                            Ok(_) => {
                                                app.status_message = if app.current_lang == "vi" {
                                                    "✅ Push thành công"
                                                } else {
                                                    "✅ Push successful"
                                                }
                                                .to_string();
                                            }
                                            Err(e) => {
                                                app.status_message =
                                                    format!("❌ Push failed: {}", e);
                                            }
                                        }
                                        app.active_modal = ActiveModal::None;
                                        app.refresh_git_status();
                                    }
                                    6 => {
                                        // Remote Info
                                        app.fetch_remote_info();
                                        app.active_modal = ActiveModal::RemoteInfo;
                                    }
                                    7 => {
                                        // Branch
                                        app.fetch_branches();
                                        app.active_modal = ActiveModal::BranchSelect;
                                    }
                                    8 => {
                                        // Stash
                                        app.fetch_stash();
                                        app.active_modal = ActiveModal::StashList;
                                    }
                                    9 => {
                                        app.fetch_commit_tree();
                                        app.selected_log_index = 0;
                                        if !app.commit_logs.is_empty() {
                                            let hash = app.commit_logs[0].hash.clone();
                                            app.fetch_commit_diff(&hash);
                                        }
                                        app.active_modal = ActiveModal::CommitTree;
                                    }
                                    10 => {
                                        app.fetch_commit_logs();
                                        app.active_modal = ActiveModal::GitLog;
                                    }
                                    11 => {
                                        app.compute_feature_groups();
                                        app.active_modal = ActiveModal::FeatureCommit;
                                    }
                                    12 => {
                                        app.github_download_url.clear();
                                        app.github_cloning_error = None;
                                        app.github_cloning = false;
                                        app.active_modal = ActiveModal::GithubDownloadUrlInput;
                                    }
                                    13 => {
                                        app.selected_setting_index = 0;
                                        app.active_modal = ActiveModal::Settings;
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
                        continue;
                    }
                    ActiveModal::CommitTree => {
                        match key.code {
                            KeyCode::Esc
                            | KeyCode::Char('q')
                            | KeyCode::Char('t')
                            | KeyCode::Char('T') => {
                                app.active_modal = ActiveModal::None;
                            }
                            KeyCode::Up | KeyCode::Char('k') => {
                                if app.selected_log_index > 0 {
                                    app.selected_log_index -= 1;
                                } else if !app.commit_logs.is_empty() {
                                    app.selected_log_index = app.commit_logs.len() - 1;
                                }
                                if !app.commit_logs.is_empty() {
                                    let hash = app.commit_logs[app.selected_log_index].hash.clone();
                                    app.fetch_commit_diff(&hash);
                                }
                            }
                            KeyCode::Down | KeyCode::Char('j') => {
                                if !app.commit_logs.is_empty() {
                                    app.selected_log_index =
                                        (app.selected_log_index + 1) % app.commit_logs.len();
                                }
                                if !app.commit_logs.is_empty() {
                                    let hash = app.commit_logs[app.selected_log_index].hash.clone();
                                    app.fetch_commit_diff(&hash);
                                }
                            }
                            _ => {}
                        }
                        continue;
                    }
                    ActiveModal::FeatureCommit => {
                        match key.code {
                            KeyCode::Esc
                            | KeyCode::Char('q')
                            | KeyCode::Char('e')
                            | KeyCode::Char('E') => {
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
                                if !app.feature_groups.is_empty()
                                    && app.selected_feature_index < app.feature_groups.len()
                                {
                                    let group = &app.feature_groups[app.selected_feature_index];
                                    let files_to_stage = group.files.clone();
                                    let feature_name = group.name.clone();
                                    let file_count = files_to_stage.len();

                                    // Unstage everything first for clean feature slice
                                    let _ = crate::git::status::unstage_all();

                                    for path in &files_to_stage {
                                        let _ = crate::git::status::stage_file(path);
                                    }

                                    app.refresh_git_status();

                                    app.status_message = if app.current_lang == "vi" {
                                        format!("✅ Đã stage feature '{}': {} file(s). Nhấn [g] hoặc [K] để commit.", feature_name, file_count)
                                    } else {
                                        format!("✅ Staged feature '{}': {} file(s). Press [g] hoặc [K] để commit.", feature_name, file_count)
                                    };

                                    app.active_modal = ActiveModal::None;
                                }
                            }
                            _ => {}
                        }
                        continue;
                    }
                    ActiveModal::GithubDownloadUrlInput => {
                        match key.code {
                            KeyCode::Esc => {
                                app.active_modal = ActiveModal::None;
                            }
                            KeyCode::Enter => {
                                let url = app.github_download_url.trim().to_string();
                                if !url.is_empty() {
                                    app.status_message = if app.current_lang == "vi" {
                                        "⏳ Đang tải thông tin repository từ GitHub...".to_string()
                                    } else {
                                        "⏳ Fetching repository metadata from GitHub...".to_string()
                                    };
                                    terminal.draw(|f| crate::ui::ui(f, app))?;
                                    app.github_cloning = true;
                                    app.github_cloning_error = None;
                                }
                            }
                            KeyCode::Up | KeyCode::Char('k') | KeyCode::Char('K') => {
                                if !app.github_history.is_empty() {
                                    if app.selected_github_history_index.is_none() {
                                        app.github_download_url_temp =
                                            app.github_download_url.clone();
                                        app.selected_github_history_index =
                                            Some(app.github_history.len() - 1);
                                        app.github_download_url = app.github_history
                                            [app.github_history.len() - 1]
                                            .clone();
                                    } else if let Some(idx) = app.selected_github_history_index {
                                        if idx > 0 {
                                            app.selected_github_history_index = Some(idx - 1);
                                            app.github_download_url =
                                                app.github_history[idx - 1].clone();
                                        } else {
                                            app.selected_github_history_index = None;
                                            app.github_download_url =
                                                app.github_download_url_temp.clone();
                                        }
                                    }
                                }
                            }
                            KeyCode::Down | KeyCode::Char('j') | KeyCode::Char('J') => {
                                if !app.github_history.is_empty() {
                                    if app.selected_github_history_index.is_none() {
                                        app.github_download_url_temp =
                                            app.github_download_url.clone();
                                        app.selected_github_history_index = Some(0);
                                        app.github_download_url = app.github_history[0].clone();
                                    } else if let Some(idx) = app.selected_github_history_index {
                                        if idx < app.github_history.len() - 1 {
                                            app.selected_github_history_index = Some(idx + 1);
                                            app.github_download_url =
                                                app.github_history[idx + 1].clone();
                                        } else {
                                            app.selected_github_history_index = None;
                                            app.github_download_url =
                                                app.github_download_url_temp.clone();
                                        }
                                    }
                                }
                            }
                            KeyCode::Backspace => {
                                if let Some(idx) = app.selected_github_history_index {
                                    app.remove_from_github_history(idx);
                                    if app.github_history.is_empty() {
                                        app.selected_github_history_index = None;
                                        app.github_download_url =
                                            app.github_download_url_temp.clone();
                                    } else if let Some(new_idx) = app.selected_github_history_index
                                    {
                                        app.github_download_url =
                                            app.github_history[new_idx].clone();
                                    }
                                } else {
                                    app.github_download_url.pop();
                                    app.github_download_url_temp = app.github_download_url.clone();
                                }
                            }
                            KeyCode::Char(c) => {
                                app.selected_github_history_index = None;
                                app.github_download_url.push(c);
                                app.github_download_url_temp = app.github_download_url.clone();
                            }
                            _ => {}
                        }
                        continue;
                    }

                    // TODO: [KEYMAP] Github Download Tree Navigation
                    // SEARCH_TAG: #github_modal_logic #navigation
                    ActiveModal::GithubDownloadTree => {
                        match key.code {
                            KeyCode::Esc => {
                                app.github_temp_dir = None;
                                app.active_modal = ActiveModal::GithubDownloadUrlInput;
                            }
                            KeyCode::Up | KeyCode::Char('k') => {
                                let len = app.get_visible_github_tree_entries().len();
                                if len > 0 {
                                    if app.selected_github_tree_index > 0 {
                                        app.selected_github_tree_index -= 1;
                                    } else {
                                        app.selected_github_tree_index = len - 1;
                                    }
                                }
                            }
                            KeyCode::Down | KeyCode::Char('j') => {
                                let len = app.get_visible_github_tree_entries().len();
                                if len > 0 {
                                    if app.selected_github_tree_index < len - 1 {
                                        app.selected_github_tree_index += 1;
                                    } else {
                                        app.selected_github_tree_index = 0;
                                    }
                                }
                            }
                            KeyCode::Char(' ') => {
                                app.toggle_github_tree_selection(app.selected_github_tree_index);
                            }
                            KeyCode::Char('b') | KeyCode::Char('B') => {
                                app.status_message = if app.current_lang == "vi" {
                                    "⏳ Đang tải danh sách chi nhánh (branch)...".to_string()
                                } else {
                                    "⏳ Fetching branch list...".to_string()
                                };
                                terminal.draw(|f| crate::ui::ui(f, app))?;
                                match app.fetch_github_branches() {
                                    Ok(_) => {
                                        app.selected_github_branch_index = app.github_branches
                                            .iter()
                                            .position(|b| b == &app.current_github_branch)
                                            .unwrap_or(0);
                                        app.active_modal = ActiveModal::GithubBranchSelect;
                                        app.status_message = if app.current_lang == "vi" {
                                            "✅ Tải danh sách chi nhánh hoàn tất".to_string()
                                        } else {
                                            "✅ Branches loaded".to_string()
                                        };
                                    }
                                    Err(e) => {
                                        app.status_message = format!("❌ Lỗi: {}", e);
                                    }
                                }
                            }
                            KeyCode::Char('v') | KeyCode::Char('V') => {
                                let visible = app.get_visible_github_tree_entries();
                                if !visible.is_empty()
                                    && app.selected_github_tree_index < visible.len()
                                {
                                    let entry = visible[app.selected_github_tree_index].clone();
                                    if !entry.is_dir {
                                        if let Some(ref dir) = app.github_temp_dir {
                                            let _ = Command::new("git")
                                                .args(["checkout", "HEAD", &entry.path])
                                                .current_dir(dir.path())
                                                .output();
                                        }
                                        app.active_modal = ActiveModal::GithubQuickView {
                                            path: entry.path.clone(),
                                            name: entry.name.clone(),
                                        };
                                    }
                                }
                            }
                            KeyCode::Right => {
                                let entry = {
                                    let visible = app.get_visible_github_tree_entries();
                                    if app.selected_github_tree_index < visible.len() {
                                        Some(visible[app.selected_github_tree_index].clone())
                                    } else {
                                        None
                                    }
                                };
                                if let Some(entry) = entry {
                                    if entry.is_dir {
                                        app.github_expanded_dirs.insert(entry.path.clone());
                                        let next_len = app.get_visible_github_tree_entries().len();
                                        if app.selected_github_tree_index >= next_len {
                                            app.selected_github_tree_index =
                                                next_len.saturating_sub(1);
                                        }
                                    }
                                }
                            }
                            KeyCode::Left => {
                                let entry = {
                                    let visible = app.get_visible_github_tree_entries();
                                    if app.selected_github_tree_index < visible.len() {
                                        Some(visible[app.selected_github_tree_index].clone())
                                    } else {
                                        None
                                    }
                                };
                                if let Some(entry) = entry {
                                    if entry.is_dir {
                                        if app.github_expanded_dirs.contains(&entry.path) {
                                            app.github_expanded_dirs.remove(&entry.path);
                                            let prefix = format!("{}/", entry.path);
                                            app.github_expanded_dirs
                                                .retain(|k| !k.starts_with(&prefix));
                                        }
                                        let next_len = app.get_visible_github_tree_entries().len();
                                        if app.selected_github_tree_index >= next_len {
                                            app.selected_github_tree_index =
                                                next_len.saturating_sub(1);
                                        }
                                    }
                                }
                            }
                            KeyCode::Enter => {
                                let len = app.get_visible_github_tree_entries().len();
                                if len > 0 && app.selected_github_tree_index < len {
                                    app.github_download_target_path = app.current_dir.clone();
                                    app.active_modal = ActiveModal::GithubDownloadTargetInput;
                                    if let Ok(output) = Command::new("osascript")
                                        .args(["-e", "POSIX path of (choose folder with prompt \"Select Destination Folder:\")"])
                                        .output()
                                    {
                                        if output.status.success() {
                                            let path_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
                                            if !path_str.is_empty() {
                                                app.github_download_target_path = path_str;
                                            }
                                        }
                                    }
                                }
                            }
                            _ => {}
                        }
                        continue;
                    }

                    ActiveModal::GithubDownloadTargetInput => {
                        match key.code {
                            KeyCode::Esc => {
                                app.active_modal = ActiveModal::GithubDownloadTree;
                            }
                            KeyCode::Tab => {
                                if let Ok(output) = Command::new("osascript")
                                    .args(["-e", "POSIX path of (choose folder with prompt \"Select Destination Folder:\")"])
                                    .output()
                                {
                                    if output.status.success() {
                                        let path_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
                                        if !path_str.is_empty() {
                                            app.github_download_target_path = path_str;
                                        }
                                    }
                                }
                            }
                            KeyCode::Enter => {
                                let is_vi = app.current_lang == "vi";
                                let current = std::path::Path::new(&app.current_dir);
                                let target = std::path::Path::new(&app.github_download_target_path);
                                let current_canon = current.canonicalize().unwrap_or_else(|_| current.to_path_buf());
                                let target_canon = target.canonicalize().unwrap_or_else(|_| target.to_path_buf());
                                if target_canon.starts_with(&current_canon) || target.starts_with(&current) {
                                    app.status_message = if is_vi {
                                        "⚠️ Không thể tải vào dự án hiện tại để tránh xung đột file!".to_string()
                                    } else {
                                        "⚠️ Cannot download into the current project to avoid file conflicts!".to_string()
                                    };
                                    continue;
                                }
                                match {
                                    app.status_message = if is_vi {
                                        "⏳ Đang sao chép tập tin từ GitHub...".to_string()
                                    } else {
                                        "⏳ Copying files from GitHub...".to_string()
                                    };
                                    terminal.draw(|f| crate::ui::ui(f, app))?;
                                    app.copy_github_download_item()
                                } {
                                    Ok(_) => {
                                        let visible = app.get_visible_github_tree_entries();
                                        let selected_name = visible.get(app.selected_github_tree_index)
                                            .map(|e| e.name.clone())
                                            .unwrap_or_default();
                                        app.status_message = if is_vi {
                                            format!("✅ Đã tải thành công: {}", selected_name)
                                        } else {
                                            format!("✅ Downloaded successfully: {}", selected_name)
                                        };
                                        app.github_temp_dir = None;
                                        app.active_modal = ActiveModal::None;
                                        app.refresh_git_status();
                                    }
                                    Err(err) => {
                                        app.status_message = if is_vi {
                                            format!("❌ Lỗi lưu tập tin: {}", err)
                                        } else {
                                            format!("❌ Save error: {}", err)
                                        };
                                        app.github_temp_dir = None;
                                    }
                                }
                            }
                            KeyCode::Backspace => {
                                app.github_download_target_path.pop();
                            }
                            KeyCode::Char(c) => {
                                app.github_download_target_path.push(c);
                            }
                            _ => {}
                        }
                        continue;
                    }
                    ActiveModal::GithubQuickView { .. } => {
                        match key.code {
                            KeyCode::Esc | KeyCode::Enter => {
                                app.active_modal = ActiveModal::GithubDownloadTree;
                            }
                            _ => {}
                        }
                        continue;
                    }
                    ActiveModal::GithubBranchSelect => {
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
                                if !app.github_branches.is_empty() && app.selected_github_branch_index < app.github_branches.len() {
                                    let selected_branch = app.github_branches[app.selected_github_branch_index].clone();
                                    app.status_message = if app.current_lang == "vi" {
                                        format!("⏳ Đang chuyển sang nhánh {}...", selected_branch)
                                    } else {
                                        format!("⏳ Switching to branch {}...", selected_branch)
                                    };
                                    terminal.draw(|f| crate::ui::ui(f, app))?;
                                    if let Some(ref dir) = app.github_temp_dir {
                                        let fetch_out = Command::new("git")
                                            .args(["fetch", "--depth", "1", "origin", &selected_branch])
                                            .current_dir(dir.path())
                                            .output();
                                        if let Ok(out) = fetch_out {
                                            if out.status.success() {
                                                let checkout_out = Command::new("git")
                                                    .args(["checkout", "FETCH_HEAD"])
                                                    .current_dir(dir.path())
                                                    .output();
                                                if let Ok(c_out) = checkout_out {
                                                    if c_out.status.success() {
                                                        if let Err(e) = app.visit_repo_dir() {
                                                            app.status_message = format!("❌ Lỗi: {}", e);
                                                        } else {
                                                            app.current_github_branch = selected_branch;
                                                            app.selected_github_tree_index = 0;
                                                            app.active_modal = ActiveModal::GithubDownloadTree;
                                                            app.status_message = if app.current_lang == "vi" {
                                                                "✅ Đã chuyển nhánh thành công".to_string()
                                                            } else {
                                                                "✅ Successfully switched branch".to_string()
                                                            };
                                                        }
                                                    } else {
                                                        app.status_message = if app.current_lang == "vi" {
                                                            "❌ Lỗi checkout chi nhánh".to_string()
                                                        } else {
                                                            "❌ Error checking out branch".to_string()
                                                        };
                                                    }
                                                } else {
                                                    app.status_message = if app.current_lang == "vi" {
                                                        "❌ Không thể chạy lệnh checkout".to_string()
                                                    } else {
                                                        "❌ Cannot execute checkout command".to_string()
                                                    };
                                                }
                                            } else {
                                                let stderr = String::from_utf8_lossy(&out.stderr).trim().to_string();
                                                app.status_message = format!("❌ Fetch failed: {}", stderr);
                                            }
                                        } else {
                                            app.status_message = if app.current_lang == "vi" {
                                                "❌ Không thể chạy lệnh fetch".to_string()
                                            } else {
                                                "❌ Cannot execute fetch command".to_string()
                                            };
                                        }
                                    }
                                }
                            }
                            _ => {}
                        }
                        continue;
                    }
                    ActiveModal::GoConfirm => {
                        match app.go_step.clone() {
                            GoStep::Confirm => match key.code {
                                KeyCode::Tab => {
                                    app.commit_input_mode = !app.commit_input_mode;
                                    if app.commit_input_mode && app.commit_input_text.is_empty() {
                                        app.commit_input_text = app.commit_message_preview.clone();
                                    }
                                }
                                KeyCode::Enter => {
                                    if app.staged_count == 0 {
                                        app.status_message = if app.current_lang == "vi" {
                                            "⚠️ Không thể tiến hành! Hãy nhấn [Space] để chọn file."
                                                .to_string()
                                        } else {
                                            "⚠️ Cannot proceed! Please stage at least 1 file."
                                                .to_string()
                                        };
                                        app.active_modal = ActiveModal::None;
                                    } else {
                                        let msg = if app.commit_input_mode {
                                            app.commit_input_text.trim().to_string()
                                        } else {
                                            app.commit_message_preview.trim().to_string()
                                        };
                                        if !msg.is_empty() {
                                            app.commit_message_preview = msg;
                                            app.go_step = GoStep::Pushing;
                                        }
                                    }
                                }
                                KeyCode::Char('y') | KeyCode::Char('Y')
                                    if !app.commit_input_mode =>
                                {
                                    if app.staged_count > 0 {
                                        let msg = app.commit_message_preview.trim().to_string();
                                        if !msg.is_empty() {
                                            app.go_step = GoStep::Pushing;
                                        }
                                    }
                                }
                                KeyCode::Backspace if app.commit_input_mode => {
                                    app.commit_input_text.pop();
                                }
                                KeyCode::Char(c) if app.commit_input_mode => {
                                    app.commit_input_text.push(c);
                                }
                                KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc
                                    if !app.commit_input_mode =>
                                {
                                    app.active_modal = ActiveModal::None;
                                    app.commit_input_mode = false;
                                }
                                KeyCode::Esc if app.commit_input_mode => {
                                    app.commit_input_mode = false;
                                }
                                _ => {}
                            },
                            GoStep::Done(_) => match key.code {
                                KeyCode::Esc | KeyCode::Enter | KeyCode::Char('q') => {
                                    app.active_modal = ActiveModal::None;
                                    app.go_step = GoStep::Confirm;
                                    app.refresh_git_status();
                                }
                                _ => {}
                            },
                            GoStep::Pushing => {}
                        }
                        continue;
                    }
                    ActiveModal::StashList => {
                        match &app.stash_step.clone() {
                            StashStep::List => match key.code {
                                KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('s') => {
                                    app.active_modal = ActiveModal::None;
                                }
                                KeyCode::Up | KeyCode::Char('k') => {
                                    if app.selected_stash_index > 0 {
                                        app.selected_stash_index -= 1;
                                    } else if !app.stash_entries.is_empty() {
                                        app.selected_stash_index = app.stash_entries.len() - 1;
                                    }
                                }
                                KeyCode::Down | KeyCode::Char('j') => {
                                    if !app.stash_entries.is_empty() {
                                        if app.selected_stash_index < app.stash_entries.len() - 1 {
                                            app.selected_stash_index += 1;
                                        } else {
                                            app.selected_stash_index = 0;
                                        }
                                    }
                                }
                                KeyCode::Char('n') | KeyCode::Char('N') => {
                                    app.status_message = match stash_push() {
                                        Ok(_) => {
                                            if app.current_lang == "vi" {
                                                "✅ Đã stash thay đổi!".to_string()
                                            } else {
                                                "✅ Changes stashed!".to_string()
                                            }
                                        }
                                        Err(_) => "❌ Stash failed.".to_string(),
                                    };
                                    app.fetch_stash();
                                    app.refresh_git_status();
                                }
                                KeyCode::Enter | KeyCode::Char('p') => {
                                    if !app.stash_entries.is_empty() {
                                        app.stash_step = StashStep::Confirm(
                                            app.selected_stash_index,
                                            StashAction::Pop,
                                        );
                                    }
                                }
                                KeyCode::Char('a') => {
                                    if !app.stash_entries.is_empty() {
                                        app.stash_step = StashStep::Confirm(
                                            app.selected_stash_index,
                                            StashAction::Apply,
                                        );
                                    }
                                }
                                KeyCode::Char('x') | KeyCode::Delete => {
                                    if !app.stash_entries.is_empty() {
                                        app.stash_step = StashStep::Confirm(
                                            app.selected_stash_index,
                                            StashAction::Drop,
                                        );
                                    }
                                }
                                _ => {}
                            },
                            StashStep::Confirm(idx, action) => {
                                let idx = *idx;
                                let action = action.clone();
                                match key.code {
                                    KeyCode::Char('y') | KeyCode::Char('Y') | KeyCode::Enter => {
                                        let ref_str = format!("stash@{{{}}}", idx);
                                        let result = match action {
                                            StashAction::Pop => stash_pop(&ref_str),
                                            StashAction::Apply => stash_apply(&ref_str),
                                            StashAction::Drop => stash_drop(&ref_str),
                                        };
                                        let is_vi = app.current_lang == "vi";
                                        app.status_message = match result {
                                            Ok(_) => match action {
                                                StashAction::Pop => {
                                                    if is_vi {
                                                        "✅ Đã pop stash!".to_string()
                                                    } else {
                                                        "✅ Stash popped!".to_string()
                                                    }
                                                }
                                                StashAction::Apply => {
                                                    if is_vi {
                                                        "✅ Đã apply stash!".to_string()
                                                    } else {
                                                        "✅ Stash applied!".to_string()
                                                    }
                                                }
                                                StashAction::Drop => {
                                                    if is_vi {
                                                        "🗑️ Đã xóa stash!".to_string()
                                                    } else {
                                                        "🗑️ Stash dropped!".to_string()
                                                    }
                                                }
                                            },
                                            Err(_) => "❌ Stash operation failed.".to_string(),
                                        };
                                        app.fetch_stash();
                                        app.refresh_git_status();
                                        app.active_modal = ActiveModal::None;
                                    }
                                    KeyCode::Esc | KeyCode::Char('n') => {
                                        app.stash_step = StashStep::List;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        continue;
                    }
                    ActiveModal::RemoteInfo => {
                        match key.code {
                            KeyCode::Esc
                            | KeyCode::Char('q')
                            | KeyCode::Char('i')
                            | KeyCode::Enter => {
                                app.active_modal = ActiveModal::None;
                            }
                            _ => {}
                        }
                        continue;
                    }
                    ActiveModal::ViewPrompt => {
                        match key.code {
                            KeyCode::Esc
                            | KeyCode::Char('q')
                            | KeyCode::Char('x')
                            | KeyCode::Char('X')
                            | KeyCode::Enter => {
                                app.active_modal = ActiveModal::None;
                            }
                            _ => {}
                        }
                        continue;
                    }
                    ActiveModal::AmendCommit => {
                        match app.amend_step.clone() {
                            AmendStep::Edit => match key.code {
                                KeyCode::Esc | KeyCode::Char('q') => {
                                    app.active_modal = ActiveModal::None;
                                }
                                KeyCode::Enter => {
                                    if !app.amend_message.trim().is_empty() {
                                        app.amend_step = AmendStep::Pushing;
                                    }
                                }
                                KeyCode::Backspace => {
                                    app.amend_message.pop();
                                }
                                KeyCode::Char(c) => {
                                    app.amend_message.push(c);
                                }
                                _ => {}
                            },
                            AmendStep::Pushing => { /* handled in main loop */ }
                            AmendStep::Done(_) => match key.code {
                                KeyCode::Esc | KeyCode::Enter | KeyCode::Char('q') => {
                                    app.active_modal = ActiveModal::None;
                                    app.amend_step = AmendStep::Edit;
                                    app.refresh_git_status();
                                }
                                _ => {}
                            },
                        }
                        continue;
                    }
                    ActiveModal::CommitDiff(_) => {
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
                        continue;
                    }
                    ActiveModal::None => {
                        if app.focus_diff {
                            match key.code {
                                KeyCode::Char('q') => return Ok(()),
                                KeyCode::Down | KeyCode::Char('j') => {
                                    app.diff_scroll_offset =
                                        app.diff_scroll_offset.saturating_add(1);
                                }
                                KeyCode::Up | KeyCode::Char('k') => {
                                    if app.diff_scroll_offset > 0 {
                                        app.diff_scroll_offset =
                                            app.diff_scroll_offset.saturating_sub(1);
                                    }
                                }
                                KeyCode::PageDown | KeyCode::Char('d') => {
                                    app.diff_scroll_offset =
                                        app.diff_scroll_offset.saturating_add(10);
                                }
                                KeyCode::PageUp | KeyCode::Char('u') => {
                                    if app.diff_scroll_offset > 0 {
                                        app.diff_scroll_offset =
                                            app.diff_scroll_offset.saturating_sub(10);
                                    }
                                }
                                KeyCode::Char(' ') => {
                                    if !app.files.is_empty() && app.selected_index < app.files.len()
                                    {
                                        let file = &app.files[app.selected_index];
                                        let is_staged = !file.status.starts_with(' ')
                                            && !file.status.starts_with('?');
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
                            continue;
                        }
                    }
                }

                // Standard controls
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
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
                            let is_staged =
                                !file.status.starts_with(' ') && !file.status.starts_with('?');
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
                            app.active_modal = ActiveModal::RevertConfirm(file.path.clone());
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
                        let diff_output = Command::new("git").args(["diff", "--cached"]).output();

                        match diff_output {
                            Ok(out) => {
                                let diff_str = String::from_utf8_lossy(&out.stdout).to_string();
                                if diff_str.trim().is_empty() {
                                    app.status_message = if app.current_lang == "vi" {
                                        "⚠️ Bạn chưa chọn (stage) file nào! Hãy nhấn [Space] để chọn file trước khi bấm 'd'.".to_string()
                                    } else {
                                        "⚠️ No files staged! Please press [Space] to select files before pressing 'd'.".to_string()
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

                                    let preview: String =
                                        diff_str.lines().take(40).collect::<Vec<_>>().join("\n");
                                    app.diff_snapshot = preview;
                                    app.last_staged_diff = diff_str.clone();
                                    app.diff_kilo_generated.clear();

                                    let ai_lang = Helper::get_ai_language_name();
                                    let prompt = format!(
                                        "{} {}.\n\nDiff:\n\n{}",
                                        crate::constant::Constant::PROMPT_EXPERT,
                                        ai_lang,
                                        diff_str
                                    );
                                    if let Ok(mut cb) = arboard::Clipboard::new() {
                                        let _ = cb.set_text(prompt);
                                    }
                                    app.active_modal = ActiveModal::DiffResult;
                                }
                            }
                            Err(e) => {
                                app.status_message = format!("❌ Error capturing diff: {}", e);
                            }
                        }
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
                        app.active_modal = ActiveModal::GitMenu;
                    }
                    KeyCode::Char('n') | KeyCode::Char('N') => {
                        app.github_download_url.clear();
                        app.github_cloning_error = None;
                        app.github_cloning = false;
                        app.active_modal = ActiveModal::GithubDownloadUrlInput;
                    }
                    KeyCode::Char('c') | KeyCode::Char('C') => {
                        app.manual_commit_message.clear();
                        app.auto_stage_all_if_enabled();
                        app.active_modal = ActiveModal::ManualCommit;
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
                        app.active_modal = ActiveModal::None;
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
                        app.active_modal = ActiveModal::LanguageSelect;
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
                        app.active_modal = ActiveModal::GitLog;
                        app.selected_log_index = 0;
                        app.fetch_commit_logs();
                    }
                    KeyCode::Char('b') | KeyCode::Char('B') => {
                        app.active_modal = ActiveModal::BranchSelect;
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
                        app.active_modal = ActiveModal::Help;
                    }
                    KeyCode::Char('s') | KeyCode::Char('S') => {
                        app.active_modal = ActiveModal::StashList;
                        app.fetch_stash();
                    }
                    KeyCode::Char('i') | KeyCode::Char('I') => {
                        app.fetch_remote_info();
                        app.active_modal = ActiveModal::RemoteInfo;
                    }
                    KeyCode::Char('x') | KeyCode::Char('X') => {
                        app.fetch_prompt();
                        app.active_modal = ActiveModal::ViewPrompt;
                    }
                    KeyCode::Char('m') | KeyCode::Char('M') => {
                        app.fetch_amend_msg();
                        app.active_modal = ActiveModal::AmendCommit;
                    }
                    KeyCode::Char('w') => {
                        app.load_workspace_history();
                        app.selected_workspace_index = 0;
                        app.active_modal = ActiveModal::WorkspaceHistory;
                    }
                    KeyCode::Char('t') | KeyCode::Char('T') => {
                        app.selected_theme_index = if app.is_light_theme { 1 } else { 0 };
                        app.active_modal = ActiveModal::ThemeSelect;
                    }
                    KeyCode::Char(',') => {
                        app.selected_setting_index = 0;
                        app.active_modal = ActiveModal::Settings;
                    }
                    _ => {}
                }
            }
        }
    }
}

fn run_cli_command<B: Backend + std::io::Write, F>(
    terminal: &mut Terminal<B>,
    mut cmd: F,
) -> Result<()>
where
    F: FnMut() -> Result<()>,
{
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    print!("{}[2J{}[1;1H", 27 as char, 27 as char);

    if let Err(e) = cmd() {
        println!("❌ Error: {}", e);
    }

    println!("\n👉 Press Enter to return to Dashboard...");
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf)?;

    enable_raw_mode()?;
    execute!(terminal.backend_mut(), EnterAlternateScreen)?;
    terminal.clear()?;

    Ok(())
}
