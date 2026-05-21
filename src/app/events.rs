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
use crate::git::branch::{checkout_branch, git_merge};
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
                    // Next step: git push
                    match git_push() {
                        Ok(_) => {
                            app.go_step = GoStep::Done(if is_vi {
                                "✅ Commit & Push thành công! Code đã lên mây ☁️".to_string()
                            } else {
                                "✅ Commit & Push successful! Code is in the cloud ☁️".to_string()
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
                                            "❌ Không thể merge chi nhánh hiện tại vào chính nó!".to_string()
                                        } else {
                                            "❌ Cannot merge current branch into itself!".to_string()
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
                            KeyCode::Esc
                            | KeyCode::Char('n')
                            | KeyCode::Char('N') => {
                                app.active_modal = ActiveModal::BranchSelect;
                            }
                            KeyCode::Enter
                            | KeyCode::Char('y')
                            | KeyCode::Char('Y') => {
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
                    ActiveModal::DiffResult => {
                        match key.code {
                            KeyCode::Esc
                            | KeyCode::Enter
                            | KeyCode::Char('q')
                            | KeyCode::Char('d') => {
                                app.active_modal = ActiveModal::None;
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
                    ActiveModal::None => {}
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

                                    let ai_lang = Helper::get_ai_language();
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
                        app.go_result = String::new();
                        app.active_modal = ActiveModal::GoConfirm;
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
                            "⚡ Đang tìm nạp (git fetch)...".to_string()
                        } else {
                            "⚡ Fetching (git fetch)...".to_string()
                        };
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
                            "⚡ Đang cập nhật (git pull)...".to_string()
                        } else {
                            "⚡ Pulling (git pull)...".to_string()
                        };
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
                    KeyCode::Char('m') | KeyCode::Char('M') => {
                        app.fetch_amend_msg();
                        app.active_modal = ActiveModal::AmendCommit;
                    }
                    KeyCode::Char('w') => {
                        let is_vi = app.current_lang == "vi";
                        let dialog_title = if is_vi {
                            "Chọn thư mục Project mới"
                        } else {
                            "Select New Project Folder"
                        };

                        if let Some(folder) =
                            rfd::FileDialog::new().set_title(dialog_title).pick_folder()
                        {
                            if std::env::set_current_dir(&folder).is_ok() {
                                app.current_dir = folder.display().to_string();
                                app.refresh_git_status();
                                app.status_message = if is_vi {
                                    "🔄 Đã tải Project mới thành công!".to_string()
                                } else {
                                    "🔄 Loaded new Project successfully!".to_string()
                                };
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
