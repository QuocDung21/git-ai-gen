use std::process::Command;

use crossterm::event::KeyCode;
use ratatui::backend::Backend;
use ratatui::Terminal;
use rust_i18n::t;

use crate::app::App;
use crate::git::remote::{git_fetch, git_pull};
use crate::git::status::{stage_all, stage_file, unstage_all, unstage_file};

use crate::app::events::run_cli_command;

pub fn handle_standard_keys<B: Backend + std::io::Write>(
    app: &mut App,
    terminal: &mut Terminal<B>,
    key: &crossterm::event::KeyEvent,
) -> Result<bool, anyhow::Error> {
    if app.focus_diff {
        match key.code {
            KeyCode::Char('q') => {
                return Ok(true);
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
                        app.status_message = t!("nav_unstaged", path = path).to_string();
                    } else {
                        let _ = stage_file(&path);
                        app.status_message = t!("nav_staged", path = path).to_string();
                    }
                    app.refresh_git_status();
                }
            }
            KeyCode::Tab | KeyCode::Esc | KeyCode::Left => {
                app.focus_diff = false;
                app.status_message = t!("nav_returned_files").to_string();
            }
            _ => {}
        }
        return Ok(false);
    }

    match key.code {
        KeyCode::Char('q') => {
            return Ok(true);
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
                    app.status_message = t!("nav_unstaged", path = path).to_string();
                } else {
                    let _ = stage_file(&path);
                    app.status_message = t!("nav_staged", path = path).to_string();
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
                app.status_message = t!("nav_diff_focused").to_string();
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
        KeyCode::Char('o') => {
            let cmd = &app.editor;
            let friendly_name = match cmd.as_str() {
                "code" => "VS Code",
                "cursor" => "Cursor",
                "zed" => "Zed",
                "subl" => "Sublime Text",
                _ => &t!("nav_system_default").to_string(),
            };
            match Command::new(cmd).arg(".").spawn() {
                Ok(_) => {
                    app.status_message = t!("nav_open_ok", name = friendly_name).to_string();
                }
                Err(_) => {
                    app.status_message = t!("nav_open_err", cmd = cmd.clone()).to_string();
                }
            }
        }
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
            app.status_message = t!("nav_reset_ok").to_string();
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
                app.status_message = t!("nav_stage_all_ok").to_string();
            } else {
                app.status_message = t!("nav_stage_all_err").to_string();
            }
            app.refresh_git_status();
        }
        KeyCode::Char('u') | KeyCode::Char('U') => {
            let success = unstage_all().is_ok();
            if success {
                app.status_message = t!("nav_unstage_all_ok").to_string();
            } else {
                app.status_message = t!("nav_unstage_all_err").to_string();
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
            app.status_message = t!("nav_fetch_start").to_string();
            terminal.draw(|f| crate::ui::ui(f, app))?;
            match git_fetch() {
                Ok(_) => {
                    app.status_message = t!("nav_fetch_ok").to_string();
                }
                Err(err) => {
                    app.status_message = t!("nav_fetch_err", err = err.to_string()).to_string();
                }
            }
            app.refresh_git_status();
        }
        KeyCode::Char('p') | KeyCode::Char('P') => {
            app.status_message = t!("nav_pull_start").to_string();
            terminal.draw(|f| crate::ui::ui(f, app))?;
            match git_pull() {
                Ok(_) => {
                    app.status_message = t!("nav_pull_ok").to_string();
                }
                Err(err) => {
                    app.status_message = t!("nav_pull_err", err = err.to_string()).to_string();
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
        KeyCode::Char('e') | KeyCode::Char('E') => {
            app.status_message = t!("nav_lang_analyze_start").to_string();
            app.language_analysis_pending = true;
            app.active_modal = crate::models::ActiveModal::ProjectLanguages;
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
        KeyCode::Char('y') | KeyCode::Char('Y') => {
            app.status_message = t!("nav_dev_opened").to_string();
            app.active_modal = crate::models::ActiveModal::HandleTest;
        }
        KeyCode::Char(',') => {
            app.selected_setting_index = 0;
            app.active_modal = crate::models::ActiveModal::Settings;
        }
        _ => {}
    }
    Ok(false)
}

fn filter_diff(diff: &str) -> String {
    let mut filtered_diffs = Vec::new();
    let sections = diff.split("diff --git ");
    for section in sections {
        if section.trim().is_empty() {
            continue;
        }
        let full_section = format!("diff --git {}", section);
        let first_line = full_section.lines().next().unwrap_or("");
        
        let is_lockfile = first_line.contains("Cargo.lock")
            || first_line.contains("package-lock.json")
            || first_line.contains("yarn.lock")
            || first_line.contains("pnpm-lock.yaml")
            || first_line.contains("composer.lock");
            
        if is_lockfile {
            let mut headers = Vec::new();
            for line in full_section.lines() {
                if line.starts_with("diff --git") 
                    || line.starts_with("index") 
                    || line.starts_with("---") 
                    || line.starts_with("+++") 
                {
                    headers.push(line);
                } else if line.starts_with("@@") {
                    headers.push(line);
                    break;
                }
            }
            headers.push(" [Modified lockfile diff content omitted for brevity to save AI tokens] ");
            filtered_diffs.push(headers.join("\n"));
        } else {
            filtered_diffs.push(full_section);
        }
    }
    
    if filtered_diffs.is_empty() {
        diff.to_string()
    } else {
        filtered_diffs.join("\n")
    }
}

fn handle_diff_capture(app: &mut App) {
    let mut is_unstaged = false;
    let mut diff_output = Command::new("git").args(["diff", "--cached"]).output();
    
    if let Ok(ref out) = diff_output {
        let staged_diff = String::from_utf8_lossy(&out.stdout).to_string();
        if staged_diff.trim().is_empty() {
            diff_output = Command::new("git").args(["diff"]).output();
            is_unstaged = true;
        }
    }

    match diff_output {
        Ok(out) => {
            let diff_str = String::from_utf8_lossy(&out.stdout).to_string();
            if diff_str.trim().is_empty() {
                app.status_message = t!("diff_no_changes").to_string();
            } else {
                app.diff_captured_unstaged = is_unstaged;
                
                let clean_diff = filter_diff(&diff_str);

                app.diff_added_lines = clean_diff
                    .lines()
                    .filter(|l| l.starts_with('+') && !l.starts_with("++"))
                    .count();
                app.diff_removed_lines = clean_diff
                    .lines()
                    .filter(|l| l.starts_with('-') && !l.starts_with("--"))
                    .count();

                app.diff_snapshot = clean_diff.clone();
                app.diff_snapshot_scroll = 0;
                app.last_staged_diff = clean_diff.clone();
                app.diff_kilo_generated.clear();

                let ai_lang = crate::helper::Helper::get_ai_language_name();
                let prompt = format!(
                    "{} {}.\n\nDiff:\n\n{}",
                    crate::constant::Constant::PROMPT_EXPERT,
                    ai_lang,
                    clean_diff
                );
                
                let mut copy_failed = false;
                if let Ok(mut cb) = arboard::Clipboard::new() {
                    if cb.set_text(prompt.clone()).is_err() {
                        copy_failed = true;
                    }
                } else {
                    copy_failed = true;
                }
                
                app.diff_copy_failed = copy_failed;
                if copy_failed {
                    let _ = std::fs::write(".git-ai-prompt.txt", &prompt);
                    app.status_message = t!("diff_clipboard_fail").to_string();
                } else {
                    app.status_message = if is_unstaged {
                        t!("diff_captured_unstaged").to_string()
                    } else {
                        t!("diff_captured_staged").to_string()
                    };
                }
                
                app.active_modal = crate::models::ActiveModal::DiffResult;
            }
        }
        Err(e) => {
            app.status_message = format!("❌ Error capturing diff: {}", e);
        }
    }
}

pub fn handle_language_analysis(app: &mut App) -> bool {
    if !app.language_analysis_pending {
        return false;
    }
    app.language_stats = crate::helper::Helper::detect_project_languages(&app.current_dir);
    app.status_message = t!("nav_lang_analyze_ok").to_string();
    app.language_analysis_pending = false;
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_diff_strips_lockfiles() {
        let diff = r#"diff --git a/src/app/mod.rs b/src/app/mod.rs
index 123456..789012 100644
--- a/src/app/mod.rs
+++ b/src/app/mod.rs
@@ -1,3 +1,4 @@
 pub struct App {
+    pub foo: bool,
 }
diff --git a/Cargo.lock b/Cargo.lock
index abcdef..fedcba 100644
--- a/Cargo.lock
+++ b/Cargo.lock
@@ -1,20 +1,25 @@
 [[package]]
 name = "anyhow"
-version = "1.0.80"
+version = "1.0.81"
 [[package]]
 name = "git-ai"
-version = "3.0.0"
+version = "3.0.1""#;

        let filtered = filter_diff(diff);
        
        assert!(filtered.contains("pub struct App {"));
        assert!(filtered.contains("+    pub foo: bool,"));
        
        assert!(filtered.contains("diff --git a/Cargo.lock b/Cargo.lock"));
        assert!(filtered.contains(" [Modified lockfile diff content omitted for brevity to save AI tokens] "));
        assert!(!filtered.contains("name = \"anyhow\""));
        assert!(!filtered.contains("version = \"3.0.1\""));
    }
}
