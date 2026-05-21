use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState},
    Frame, Terminal,
};
use std::process::Command;
use std::{env, io, time::Duration};

use crate::helper::Helper;

struct ChangedFile {
    status: String,
    path: String,
}

struct CommitLogEntry {
    hash: String,
    author: String,
    time: String,
    subject: String,
}

struct StashEntry {
    index: usize,
    branch: String,
    message: String,
}

#[derive(Clone, PartialEq)]
enum StashAction { Pop, Apply, Drop }

#[derive(Clone, PartialEq)]
enum StashStep {
    List,
    Confirm(usize, StashAction),
}

#[derive(Clone, PartialEq)]
enum GoStep {
    Confirm,          // Bước 1: hỏi xác nhận
    Pushing,          // Bước 2: đang chạy git
    Done(String),     // Bước 3: kết quả (success/fail message)
}

#[derive(Clone, PartialEq)]
enum AmendStep {
    Edit,
    Pushing,
    Done(String),
}

#[derive(Clone, PartialEq)]
enum ActiveModal {
    None,
    RevertConfirm(String), // Path of the file to revert
    LanguageSelect,
    Help,
    GitLog,
    BranchSelect,
    DiffResult,       // Hiển thị kết quả diff copy to clipboard
    GoConfirm,        // Multi-step commit & push modal
    StashList,
    RemoteInfo,
    AmendCommit,
    CommitDiff(String), // commit hash
}

// --- DASHBOARD STATE MANAGEMENT ---
struct App {
    status_message: String,
    git_status_lines: Vec<String>,
    current_lang: String,
    current_dir: String,
    files: Vec<ChangedFile>,
    selected_index: usize,
    selected_file_diff: String,
    diff_scroll_offset: usize,
    active_modal: ActiveModal,
    selected_lang_index: usize,
    // Real-time Git Statistics
    current_branch: String,
    staged_count: usize,
    unstaged_count: usize,
    untracked_count: usize,
    // Commit history state
    commit_logs: Vec<CommitLogEntry>,
    selected_log_index: usize,
    // Branch switcher state
    branches: Vec<String>,
    selected_branch_index: usize,
    // Diff result modal
    diff_snapshot: String,
    diff_added_lines: usize,
    diff_removed_lines: usize,
    // Go confirm modal
    commit_message_preview: String,
    go_step: GoStep,
    go_result: String,
    // Inline commit input (Tab to toggle)
    commit_input_mode: bool,
    commit_input_text: String,
    // Stash manager
    stash_entries: Vec<StashEntry>,
    selected_stash_index: usize,
    stash_step: StashStep,
    // Remote info
    remote_url: String,
    remote_tracking: String,
    ahead_count: i32,
    behind_count: i32,
    // Amend commit
    amend_step: AmendStep,
    amend_message: String,
    // Commit diff viewer
    commit_diff_content: String,
    commit_diff_scroll: usize,
    // Conflict detection
    has_conflicts: bool,
    conflict_count: usize,
}

impl App {
    fn new() -> Self {
        let current_dir = env::current_dir()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|_| "Unknown".to_string());

        let current_lang =  Helper::get_ai_language();
        let init_msg = if current_lang == "vi" {
            "Sẵn sàng tạo Commit Message! Nhấn Space để stage, Backspace để revert."
        } else {
            "Ready to generate Commit Message! Press Space to stage, Backspace to revert."
        };

        let mut app = App {
            status_message: init_msg.to_string(),
            git_status_lines: Vec::new(),
            current_lang,
            current_dir,
            files: Vec::new(),
            selected_index: 0,
            selected_file_diff: String::new(),
            diff_scroll_offset: 0,
            active_modal: ActiveModal::None,
            selected_lang_index: 0,
            current_branch: "detached".to_string(),
            staged_count: 0,
            unstaged_count: 0,
            untracked_count: 0,
            commit_logs: Vec::new(),
            selected_log_index: 0,
            branches: Vec::new(),
            selected_branch_index: 0,
            diff_snapshot: String::new(),
            diff_added_lines: 0,
            diff_removed_lines: 0,
            commit_message_preview: String::new(),
            go_step: GoStep::Confirm,
            go_result: String::new(),
            commit_input_mode: false,
            commit_input_text: String::new(),
            stash_entries: Vec::new(),
            selected_stash_index: 0,
            stash_step: StashStep::List,
            remote_url: String::new(),
            remote_tracking: String::new(),
            ahead_count: 0,
            behind_count: 0,
            amend_step: AmendStep::Edit,
            amend_message: String::new(),
            commit_diff_content: String::new(),
            commit_diff_scroll: 0,
            has_conflicts: false,
            conflict_count: 0,
        };
        app.refresh_git_status();
        app
    }

    fn refresh_git_status(&mut self) {
        let prev_selected_path = if self.files.is_empty() || self.selected_index >= self.files.len() {
            None
        } else {
            Some(self.files[self.selected_index].path.clone())
        };

        self.git_status_lines.clear();
        self.files.clear();
        if let Ok(output) = Command::new("git").args(["status", "-s"]).output() {
            let status_text = String::from_utf8_lossy(&output.stdout);
            if status_text.trim().is_empty() {
                let msg = if self.current_lang == "vi" {
                    "✅ Thư mục làm việc sạch sẽ (Không có thay đổi)."
                } else {
                    "✅ Working tree clean (No changes)."
                };
                self.git_status_lines.push(msg.to_string());
            } else {
                for line in status_text.lines() {
                    self.git_status_lines.push(format!(" {}", line));
                    let trimmed = line.trim();
                    if trimmed.len() >= 3 {
                        // Porcelain status is exactly 2 characters at the start
                        let status = line[..3].to_string(); // Keep leading/trailing spaces in status
                        let path = line[3..].trim().to_string();
                        self.files.push(ChangedFile { status, path });
                    }
                }
            }
        } else {
            let msg = if self.current_lang == "vi" {
                "❌ Không thể đọc trạng thái Git."
            } else {
                "❌ Failed to read Git status."
            };
            self.git_status_lines.push(msg.to_string());
        }

        // Fetch active branch name
        self.current_branch = if let Ok(output) = Command::new("git").args(["rev-parse", "--abbrev-ref", "HEAD"]).output() {
            let br = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if br.is_empty() {
                "detached".to_string()
            } else {
                br
            }
        } else {
            "detached".to_string()
        };

        // Reset and calculate change statistics + conflicts
        self.staged_count = 0;
        self.unstaged_count = 0;
        self.untracked_count = 0;
        self.conflict_count = 0;

        for file in &self.files {
            let first_char = file.status.chars().next().unwrap_or(' ');
            let second_char = file.status.chars().nth(1).unwrap_or(' ');
            let status_trimmed = file.status.trim();

            // Detect merge conflicts: UU, AA, DD, DU, UD, AU, UA
            let is_conflict = matches!(status_trimmed, "UU" | "AA" | "DD" | "DU" | "UD" | "AU" | "UA");
            if is_conflict {
                self.conflict_count += 1;
            }

            if first_char != ' ' && first_char != '?' {
                self.staged_count += 1;
            }
            if second_char != ' ' && second_char != '?' {
                self.unstaged_count += 1;
            }
            if first_char == '?' && second_char == '?' {
                self.untracked_count += 1;
            }
        }
        self.has_conflicts = self.conflict_count > 0;

        if self.files.is_empty() {
            self.selected_index = 0;
            self.selected_file_diff = String::new();
            self.diff_scroll_offset = 0;
        } else {
            if let Some(path) = prev_selected_path {
                if let Some(new_idx) = self.files.iter().position(|f| f.path == path) {
                    self.selected_index = new_idx;
                } else {
                    if self.selected_index >= self.files.len() {
                        self.selected_index = self.files.len() - 1;
                    }
                }
            } else {
                if self.selected_index >= self.files.len() {
                    self.selected_index = self.files.len() - 1;
                }
            }
            self.update_selected_diff();
        }
    }

    fn update_selected_diff(&mut self) {
        if self.files.is_empty() || self.selected_index >= self.files.len() {
            self.selected_file_diff = String::new();
            return;
        }

        let file = &self.files[self.selected_index];
        let is_untracked = file.status.starts_with("??") || file.status.contains("??");

        let output = if is_untracked {
            if let Ok(content) = std::fs::read_to_string(&file.path) {
                let lines: Vec<&str> = content.lines().take(500).collect();
                let heading = if self.current_lang == "vi" {
                    format!("📄 [Tập tin chưa theo dõi]\n\n")
                } else {
                    format!("📄 [Untracked File]\n\n")
                };
                heading + &lines.join("\n")
            } else {
                if self.current_lang == "vi" {
                    "[Không thể đọc tập tin]"
                } else {
                    "[Cannot read file]"
                }.to_string()
            }
        } else {
            // Try git diff HEAD <file> first so we see staged AND unstaged changes
            let mut diff_output = None;
            if let Ok(out) = Command::new("git").args(["diff", "HEAD", "--", &file.path]).output() {
                let diff = String::from_utf8_lossy(&out.stdout).to_string();
                if !diff.trim().is_empty() {
                    diff_output = Some(diff);
                }
            }

            // Fallback to git diff cached or regular diff if git diff HEAD didn't work/is empty
            if diff_output.is_none() {
                if let Ok(out) = Command::new("git").args(["diff", "--", &file.path]).output() {
                    let diff = String::from_utf8_lossy(&out.stdout).to_string();
                    if !diff.trim().is_empty() {
                        diff_output = Some(diff);
                    }
                }
            }

            if diff_output.is_none() {
                if let Ok(out) = Command::new("git").args(["diff", "--cached", "--", &file.path]).output() {
                    let diff = String::from_utf8_lossy(&out.stdout).to_string();
                    if !diff.trim().is_empty() {
                        diff_output = Some(diff);
                    }
                }
            }

            diff_output.unwrap_or_else(|| {
                if self.current_lang == "vi" {
                    "[Không có thay đổi so với commit cuối cùng]".to_string()
                } else {
                    "[No changes compared to last commit]".to_string()
                }
            })
        };
        self.selected_file_diff = output;
    }

    fn fetch_commit_logs(&mut self) {
        self.commit_logs.clear();
        if let Ok(output) = Command::new("git")
            .args(["log", "--pretty=format:%h|%an|%ar|%s", "-n", "15"])
            .output()
        {
            let logs_text = String::from_utf8_lossy(&output.stdout);
            for line in logs_text.lines() {
                let parts: Vec<&str> = line.split('|').collect();
                if parts.len() >= 4 {
                    self.commit_logs.push(CommitLogEntry {
                        hash: parts[0].to_string(),
                        author: parts[1].to_string(),
                        time: parts[2].to_string(),
                        subject: parts[3..].join("|"),
                    });
                }
            }
        }
    }

    fn fetch_branches(&mut self) {
        self.branches.clear();
        if let Ok(output) = Command::new("git")
            .args(["branch", "--format=%(refname:short)"])
            .output()
        {
            let branches_text = String::from_utf8_lossy(&output.stdout);
            for line in branches_text.lines() {
                let trimmed = line.trim();
                if !trimmed.is_empty() {
                    self.branches.push(trimmed.to_string());
                }
            }
        }
        if let Some(idx) = self.branches.iter().position(|b| b == &self.current_branch) {
            self.selected_branch_index = idx;
        } else {
            self.selected_branch_index = 0;
        }
    }

    fn fetch_stash(&mut self) {
        self.stash_entries.clear();
        if let Ok(output) = Command::new("git")
            .args(["stash", "list", "--format=%gd|%gs"])
            .output()
        {
            let text = String::from_utf8_lossy(&output.stdout);
            for (i, line) in text.lines().enumerate() {
                // Format: stash@{N}|On branch: message
                let parts: Vec<&str> = line.splitn(2, '|').collect();
                if parts.len() == 2 {
                    let info = parts[1];
                    // Try to parse "On <branch>: <message>"
                    let (branch, message) = if let Some(rest) = info.strip_prefix("On ") {
                        if let Some(colon_idx) = rest.find(": ") {
                            (rest[..colon_idx].to_string(), rest[colon_idx + 2..].to_string())
                        } else {
                            ("?".to_string(), info.to_string())
                        }
                    } else if let Some(rest) = info.strip_prefix("WIP on ") {
                        if let Some(colon_idx) = rest.find(": ") {
                            (rest[..colon_idx].to_string(), rest[colon_idx + 2..].to_string())
                        } else {
                            ("?".to_string(), info.to_string())
                        }
                    } else {
                        ("?".to_string(), info.to_string())
                    };
                    self.stash_entries.push(StashEntry { index: i, branch, message });
                }
            }
        }
        if self.selected_stash_index >= self.stash_entries.len() {
            self.selected_stash_index = 0;
        }
        self.stash_step = StashStep::List;
    }

    fn fetch_remote_info(&mut self) {
        // Remote URL
        self.remote_url = if let Ok(out) = Command::new("git")
            .args(["remote", "get-url", "origin"])
            .output()
        {
            String::from_utf8_lossy(&out.stdout).trim().to_string()
        } else {
            "(no remote)".to_string()
        };

        // Tracking branch
        let tracking_key = format!("branch.{}.remote", self.current_branch);
        let remote_name = if let Ok(out) = Command::new("git")
            .args(["config", "--get", &tracking_key])
            .output()
        {
            String::from_utf8_lossy(&out.stdout).trim().to_string()
        } else {
            "origin".to_string()
        };
        self.remote_tracking = format!("{}/{}", remote_name, self.current_branch);

        // Ahead/behind counts
        let rev_range = format!("{}...{}", self.current_branch, self.remote_tracking);
        if let Ok(out) = Command::new("git")
            .args(["rev-list", "--left-right", "--count", &rev_range])
            .output()
        {
            let text = String::from_utf8_lossy(&out.stdout);
            let nums: Vec<&str> = text.trim().split_whitespace().collect();
            if nums.len() == 2 {
                self.ahead_count = nums[0].parse().unwrap_or(0);
                self.behind_count = nums[1].parse().unwrap_or(0);
            }
        } else {
            self.ahead_count = 0;
            self.behind_count = 0;
        }
    }

    fn fetch_amend_msg(&mut self) {
        self.amend_message = if let Ok(out) = Command::new("git")
            .args(["log", "-1", "--pretty=format:%s"])
            .output()
        {
            String::from_utf8_lossy(&out.stdout).trim().to_string()
        } else {
            String::new()
        };
        self.amend_step = AmendStep::Edit;
    }

    fn fetch_commit_diff(&mut self, hash: &str) {
        self.commit_diff_content = if let Ok(out) = Command::new("git")
            .args(["show", "--stat", "--patch", hash])
            .output()
        {
            String::from_utf8_lossy(&out.stdout).to_string()
        } else {
            format!("Error: could not get diff for {}", hash)
        };
        self.commit_diff_scroll = 0;
    }
}

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
        terminal.draw(|f| ui(f, app))?;

        // Handle GoStep::Pushing: run git commands outside of event poll
        // Trong hàm run_app tại vòng lặp chính (loop)
        if app.active_modal == ActiveModal::GoConfirm {
            if let GoStep::Pushing = &app.go_step {
                let is_vi = app.current_lang == "vi";
                let msg = app.commit_message_preview.clone();

                // ĐÃ SỬA: Loại bỏ hoàn toàn bước "git add ." tự động bừa bãi
                // Tiến hành commit trực tiếp các file đã được chọn (Staged) qua phím Space
                let commit_ok = Command::new("git").args(["commit", "-m", &msg]).output()
                    .map(|o| o.status.success()).unwrap_or(false);

                if !commit_ok {
                    app.go_step = GoStep::Done(if is_vi {
                        "❌ Lỗi: git commit thất bại. Hãy chắc chắn bạn đã chọn file cần commit.".to_string()
                    } else {
                        "❌ Error: git commit failed. Make sure you have staged files to commit.".to_string()
                    });
                } else {
                    // Bước tiếp theo: git push
                    let push_output = Command::new("git").arg("push").output();
                    match push_output {
                        Ok(out) if out.status.success() => {
                            app.go_step = GoStep::Done(if is_vi {
                                "✅ Commit & Push thành công! Code đã lên mây ☁️".to_string()
                            } else {
                                "✅ Commit & Push successful! Code is in the cloud ☁️".to_string()
                            });
                        }
                        Ok(out) => {
                            let err = String::from_utf8_lossy(&out.stderr).trim().to_string();
                            app.go_step = GoStep::Done(format!(
                                "{} {}",
                                if is_vi { "❌ Push thất bại:" } else { "❌ Push failed:" },
                                err
                            ));
                        }
                        Err(e) => {
                            app.go_step = GoStep::Done(format!("❌ Error: {}", e));
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
                let result = Command::new("git")
                    .args(["commit", "--amend", "--no-edit", "-m", &msg])
                    .output();
                app.amend_step = match result {
                    Ok(out) if out.status.success() => AmendStep::Done(
                        if is_vi { "✅ Đã sửa commit cuối! (Amend thành công)".to_string() }
                        else { "✅ Last commit amended successfully!".to_string() }
                    ),
                    Ok(out) => {
                        let err = String::from_utf8_lossy(&out.stderr).trim().to_string();
                        AmendStep::Done(format!("❌ Amend failed: {}", err))
                    }
                    Err(e) => AmendStep::Done(format!("❌ Error: {}", e)),
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
                            KeyCode::Esc | KeyCode::Char(' ') | KeyCode::Enter | KeyCode::Char('q') | KeyCode::Char('?') | KeyCode::Char('h') => {
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
                                let locales = crate::Locales::new(&app.current_lang);
                                if let Ok(msg) = crate::handle_lang("vi", &locales) {
                                    app.status_message = msg;
                                    app.current_lang = Helper::get_ai_language();
                                }
                                app.active_modal = ActiveModal::None;
                            }
                            KeyCode::Char('e') | KeyCode::Char('E') => {
                                let locales = crate::Locales::new(&app.current_lang);
                                if let Ok(msg) = crate::handle_lang("en", &locales) {
                                    app.status_message = msg;
                                    app.current_lang = Helper::get_ai_language();
                                }
                                app.active_modal = ActiveModal::None;
                            }
                            KeyCode::Char('a') | KeyCode::Char('A') => {
                                let locales = crate::Locales::new(&app.current_lang);
                                if let Ok(msg) = crate::handle_lang("auto", &locales) {
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
                                let locales = crate::Locales::new(&app.current_lang);
                                if let Ok(msg) = crate::handle_lang(selection, &locales) {
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
                                    f.path == path_to_revert && (f.status.starts_with("??") || f.status.contains("??"))
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
                                    let _ = Command::new("git").args(["restore", "--staged", "--", &path_to_revert]).output();
                                    let success = Command::new("git").args(["restore", "--", &path_to_revert]).output()
                                        .map(|o| o.status.success()).unwrap_or(false);
                                    if success {
                                        app.status_message = if app.current_lang == "vi" {
                                            format!("🔄 Đã khôi phục file: {}", path_to_revert)
                                        } else {
                                            format!("🔄 Reverted changes in file: {}", path_to_revert)
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
                            KeyCode::Enter => {
                                if !app.commit_logs.is_empty() && app.selected_log_index < app.commit_logs.len() {
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
                            KeyCode::Enter => {
                                if !app.branches.is_empty() && app.selected_branch_index < app.branches.len() {
                                    let branch_name = app.branches[app.selected_branch_index].clone();
                                    let status = Command::new("git")
                                        .args(["checkout", &branch_name])
                                        .output();

                                    match status {
                                        Ok(output) if output.status.success() => {
                                            app.status_message = if app.current_lang == "vi" {
                                                format!("🌿 Đã chuyển sang chi nhánh: {}", branch_name)
                                            } else {
                                                format!("🌿 Checked out branch: {}", branch_name)
                                            };
                                        }
                                        Ok(output) => {
                                            let err_msg = String::from_utf8_lossy(&output.stderr).trim().to_string();
                                            app.status_message = if app.current_lang == "vi" {
                                                format!("❌ Lỗi chuyển chi nhánh: {}", err_msg)
                                            } else {
                                                format!("❌ Checkout failed: {}", err_msg)
                                            };
                                        }
                                        Err(e) => {
                                            app.status_message = format!("❌ Error: {}", e);
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
                    ActiveModal::DiffResult => {
                        match key.code {
                            KeyCode::Esc | KeyCode::Enter | KeyCode::Char('q') | KeyCode::Char('d') => {
                                app.active_modal = ActiveModal::None;
                            }
                            _ => {}
                        }
                        continue;
                    }
                    ActiveModal::GoConfirm => {
                        match app.go_step.clone() {
                            GoStep::Confirm => {
                                match key.code {
                                    KeyCode::Tab => {
                                        app.commit_input_mode = !app.commit_input_mode;
                                        if app.commit_input_mode && app.commit_input_text.is_empty() {
                                            app.commit_input_text = app.commit_message_preview.clone();
                                        }
                                    }
                                    KeyCode::Enter => {
                                        // ĐÃ SỬA: Chặn hành động nếu không có file nào được chọn
                                        if app.staged_count == 0 {
                                            app.status_message = if app.current_lang == "vi" {
                                                "⚠️ Không thể tiến hành! Hãy nhấn [Space] ngoài danh sách để chọn ít nhất 1 file.".to_string()
                                            } else {
                                                "⚠️ Cannot proceed! Please press [Space] outside to select at least 1 file.".to_string()
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
                                    KeyCode::Char('y') | KeyCode::Char('Y') if !app.commit_input_mode => {
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
                                    KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc if !app.commit_input_mode => {
                                        app.active_modal = ActiveModal::None;
                                        app.commit_input_mode = false;
                                        app.status_message = if app.current_lang == "vi" {
                                            "ℹ️ Đã hủy Commit & Push.".to_string()
                                        } else {
                                            "ℹ️ Commit & Push cancelled.".to_string()
                                        };
                                    }
                                    KeyCode::Esc if app.commit_input_mode => {
                                        app.commit_input_mode = false;
                                    }
                                    _ => {}
                                }
                            }
                            _ => {}
                        }
                        continue;
                    }
                    ActiveModal::StashList => {
                        match &app.stash_step.clone() {
                            StashStep::List => {
                                match key.code {
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
                                    // Quick stash current changes
                                    KeyCode::Char('n') | KeyCode::Char('N') => {
                                        let out = Command::new("git").args(["stash", "push", "-m", "WIP stash"]).output();
                                        app.status_message = if out.map(|o| o.status.success()).unwrap_or(false) {
                                            if app.current_lang == "vi" { "✅ Đã stash thay đổi!".to_string() } else { "✅ Changes stashed!".to_string() }
                                        } else {
                                            "❌ Stash failed.".to_string()
                                        };
                                        app.fetch_stash();
                                        app.refresh_git_status();
                                    }
                                    // Pop = apply + drop
                                    KeyCode::Enter | KeyCode::Char('p') => {
                                        if !app.stash_entries.is_empty() {
                                            app.stash_step = StashStep::Confirm(app.selected_stash_index, StashAction::Pop);
                                        }
                                    }
                                    // Apply (keep stash)
                                    KeyCode::Char('a') => {
                                        if !app.stash_entries.is_empty() {
                                            app.stash_step = StashStep::Confirm(app.selected_stash_index, StashAction::Apply);
                                        }
                                    }
                                    // Drop (delete)
                                    KeyCode::Char('x') | KeyCode::Delete => {
                                        if !app.stash_entries.is_empty() {
                                            app.stash_step = StashStep::Confirm(app.selected_stash_index, StashAction::Drop);
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            StashStep::Confirm(idx, action) => {
                                let idx = *idx;
                                let action = action.clone();
                                match key.code {
                                    KeyCode::Char('y') | KeyCode::Char('Y') | KeyCode::Enter => {
                                        let ref_str = format!("stash@{{{}}}", idx);
                                        let result = match action {
                                            StashAction::Pop => Command::new("git").args(["stash", "pop", &ref_str]).output(),
                                            StashAction::Apply => Command::new("git").args(["stash", "apply", &ref_str]).output(),
                                            StashAction::Drop => Command::new("git").args(["stash", "drop", &ref_str]).output(),
                                        };
                                        let is_vi = app.current_lang == "vi";
                                        app.status_message = match result {
                                            Ok(o) if o.status.success() => match action {
                                                StashAction::Pop => if is_vi { "✅ Đã pop stash!".to_string() } else { "✅ Stash popped!".to_string() },
                                                StashAction::Apply => if is_vi { "✅ Đã apply stash!".to_string() } else { "✅ Stash applied!".to_string() },
                                                StashAction::Drop => if is_vi { "🗑️ Đã xóa stash!".to_string() } else { "🗑️ Stash dropped!".to_string() },
                                            },
                                            _ => "❌ Stash operation failed.".to_string(),
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
                            KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('i') | KeyCode::Enter => {
                                app.active_modal = ActiveModal::None;
                            }
                            _ => {}
                        }
                        continue;
                    }
                    ActiveModal::AmendCommit => {
                        match app.amend_step.clone() {
                            AmendStep::Edit => {
                                match key.code {
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
                                }
                            }
                            AmendStep::Pushing => { /* handled in main loop */ }
                            AmendStep::Done(_) => {
                                match key.code {
                                    KeyCode::Esc | KeyCode::Enter | KeyCode::Char('q') => {
                                        app.active_modal = ActiveModal::None;
                                        app.amend_step = AmendStep::Edit;
                                        app.refresh_git_status();
                                    }
                                    _ => {}
                                }
                            }
                        }
                        continue;
                    }
                    ActiveModal::CommitDiff(_) => {
                        match key.code {
                            KeyCode::Esc | KeyCode::Char('q') => {
                                // Go back to GitLog
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
                    KeyCode::Char('q') => return Ok(()), // Quit
                    KeyCode::Up | KeyCode::Char('k') => {
                        if !app.files.is_empty() {
                            if app.selected_index > 0 {
                                app.selected_index -= 1;
                            } else {
                                app.selected_index = app.files.len() - 1; // Wrap around
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
                                app.selected_index = 0; // Wrap around
                            }
                            app.diff_scroll_offset = 0;
                            app.update_selected_diff();
                        }
                    }
                    KeyCode::Char(' ') => {
                        if !app.files.is_empty() && app.selected_index < app.files.len() {
                            let file = &app.files[app.selected_index];
                            // If first char of status is not space and not '?' (i.e. staged in some way)
                            let is_staged = !file.status.starts_with(' ') && !file.status.starts_with('?');
                            let path = file.path.clone();

                            if is_staged {
                                let _ = Command::new("git").args(["restore", "--staged", "--", &path]).output();
                                app.status_message = if app.current_lang == "vi" {
                                    format!("➖ Đã unstage: {}", path)
                                } else {
                                    format!("➖ Unstaged: {}", path)
                                };
                            } else {
                                let _ = Command::new("git").args(["add", "--", &path]).output();
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
                        // ĐÃ SỬA: Chỉ lấy diff của các file ĐÃ CHỌN (staged) bằng lệnh --cached
                        let diff_output = Command::new("git").args(["diff", "--cached"]).output();

                        match diff_output {
                            Ok(out) => {
                                let diff_str = String::from_utf8_lossy(&out.stdout).to_string();
                                if diff_str.trim().is_empty() {
                                    // ĐÃ SỬA: Thông báo nhắc nhở người dùng chọn file trước khi xin AI commit message
                                    app.status_message = if app.current_lang == "vi" {
                                        "⚠️ Bạn chưa chọn (stage) file nào! Hãy nhấn [Space] để chọn file trước khi bấm 'd'.".to_string()
                                    } else {
                                        "⚠️ No files staged! Please press [Space] to select files before pressing 'd'.".to_string()
                                    };
                                } else {
                                    // Đếm dòng thêm/bớt dựa trên các file đã chọn
                                    app.diff_added_lines = diff_str.lines()
                                        .filter(|l| l.starts_with('+') && !l.starts_with("++"))
                                        .count();
                                    app.diff_removed_lines = diff_str.lines()
                                        .filter(|l| l.starts_with('-') && !l.starts_with("--"))
                                        .count();

                                    // Truncate preview (hiển thị 40 dòng đầu trong modal kết quả)
                                    let preview: String = diff_str.lines().take(40)
                                        .collect::<Vec<_>>().join("\n");
                                    app.diff_snapshot = preview;

                                    // Copy prompt + chỉ diff của những file đã chọn vào Clipboard
                                    let locales = crate::Locales::new(&app.current_lang);
                                    let ai_lang = Helper::get_ai_language();
                                    let prompt = format!(
                                        "{} {}.\n\nDiff:\n\n{}",
                                        locales.prompt_expert, ai_lang, diff_str
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
                        // Read commit message from clipboard, open GoConfirm modal
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
                        let locales = crate::Locales::new(&app.current_lang);
                        run_cli_command(terminal, || crate::handle_restore(&locales))?;
                        app.refresh_git_status();
                        app.status_message = if app.current_lang == "vi" {
                            "🔄 Đã reset cấu hình hệ thống.".to_string()
                        } else {
                            "🔄 System configuration reset.".to_string()
                        };
                    }
                    KeyCode::Char('l') => {
                        app.active_modal = ActiveModal::None; // Safe fallback
                        let raw_lang = if let Ok(output) = Command::new("git")
                            .args(["config", "--global", "--get", "git-ai.lang"])
                            .output()
                        {
                            let stdout = String::from_utf8_lossy(&output.stdout).trim().to_lowercase();
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
                        let success = Command::new("git").args(["add", "."]).output()
                            .map(|o| o.status.success())
                            .unwrap_or(false);
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
                        let success = Command::new("git").args(["reset"]).output()
                            .map(|o| o.status.success())
                            .unwrap_or(false);
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
                        let result = Command::new("git").arg("fetch").output();
                        match result {
                            Ok(out) if out.status.success() => {
                                app.status_message = if is_vi {
                                    "✅ Đã tìm nạp (git fetch) thành công!".to_string()
                                } else {
                                    "✅ Git fetch completed successfully!".to_string()
                                };
                            }
                            Ok(out) => {
                                let err = String::from_utf8_lossy(&out.stderr).trim().to_string();
                                app.status_message = if is_vi {
                                    format!("❌ Lỗi git fetch: {}", err)
                                } else {
                                    format!("❌ git fetch failed: {}", err)
                                };
                            }
                            Err(e) => {
                                app.status_message = if is_vi {
                                    format!("❌ Lỗi: {}", e)
                                } else {
                                    format!("❌ Error: {}", e)
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
                        let result = Command::new("git").arg("pull").output();
                        match result {
                            Ok(out) if out.status.success() => {
                                app.status_message = if is_vi {
                                    "✅ Đã cập nhật (git pull) thành công!".to_string()
                                } else {
                                    "✅ Git pull completed successfully!".to_string()
                                };
                            }
                            Ok(out) => {
                                let err = String::from_utf8_lossy(&out.stderr).trim().to_string();
                                app.status_message = if is_vi {
                                    format!("❌ Lỗi git pull: {}", err)
                                } else {
                                    format!("❌ git pull failed: {}", err)
                                };
                            }
                            Err(e) => {
                                app.status_message = if is_vi {
                                    format!("❌ Lỗi: {}", e)
                                } else {
                                    format!("❌ Error: {}", e)
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

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}

fn ui(f: &mut Frame, app: &App) {
    let is_vi = app.current_lang == "vi";

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(8), // Upgraded Header / Banner to fit 6 lines of ASCII logo!
                Constraint::Length(3), // Workspace Badge Bar
                Constraint::Min(0),    // Main workspace area
                Constraint::Length(3), // Status message bar
            ]
            .as_ref(),
        )
        .split(f.size());

    // 1. SPLIT HEADER / BANNER
    let header_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(70), // Title and Neon ASCII Brand
                Constraint::Percentage(30), // System Details
            ]
            .as_ref(),
        )
        .split(chunks[0]);

    // Left Header: Neon HSL Gradient Block Letter ASCII Logo ("GIT-AI")
    let brand_lines = vec![
        Line::from(vec![
            Span::styled("  ██████╗ ██╗████████╗      ███████╗██╗", Style::default().fg(Color::Rgb(189, 147, 249)).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled(" ██╔════╝ ██║╚══██╔══╝██═══██╔════╝██║", Style::default().fg(Color::Rgb(255, 121, 198)).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled(" ██║  ███╗██║   ██║   ╚█████╔█████╗ ██║", Style::default().fg(Color::Rgb(139, 233, 253)).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled(" ██║   ██║██║   ██║   ██╔═══██╔═══╝ ██║", Style::default().fg(Color::Rgb(80, 250, 123)).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled(" ╚██████╔╝██║   ██║   ╚██████╔███████║", Style::default().fg(Color::Rgb(241, 250, 140)).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("  ╚═════╝ ╚═╝   ╚═╝    ╚═════╝╚══════╝", Style::default().fg(Color::Rgb(255, 184, 108)).add_modifier(Modifier::BOLD)),
        ]),
    ];

    let brand_widget = Paragraph::new(brand_lines).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Rgb(189, 147, 249)))
            .border_type(ratatui::widgets::BorderType::Rounded),
    );
    f.render_widget(brand_widget, header_chunks[0]);

    // Right Header: System Settings details
    let lang_display = if app.current_lang == "vi" {
        "Tiếng Việt"
    } else {
        "English"
    };

    let right_header_text = vec![
        Line::from(vec![
            Span::styled(" 🤖  ULTIMATE GIT-AI SYSTEM ", Style::default().fg(Color::Rgb(189, 147, 249)).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled(" ⚡  AI Status: ", Style::default().fg(Color::Rgb(98, 114, 164))),
            Span::styled("ONLINE", Style::default().fg(Color::Rgb(80, 250, 123)).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled(" 🌎  Language:  ", Style::default().fg(Color::Rgb(98, 114, 164))),
            Span::styled(lang_display, Style::default().fg(Color::Rgb(241, 250, 140)).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled(" 📦  Version:   ", Style::default().fg(Color::Rgb(98, 114, 164))),
            Span::styled("v3.0.0", Style::default().fg(Color::Rgb(139, 233, 253))),
        ]),
        Line::from(vec![
            Span::styled(" 💡  Help:      ", Style::default().fg(Color::Rgb(98, 114, 164))),
            Span::styled("Press '?' or 'h'", Style::default().fg(Color::Rgb(255, 184, 108)).add_modifier(Modifier::ITALIC)),
        ]),
    ];

    let right_header_widget = Paragraph::new(right_header_text).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Rgb(139, 233, 253)))
            .border_type(ratatui::widgets::BorderType::Rounded),
    );
    f.render_widget(right_header_widget, header_chunks[1]);

    // 2. WORKSPACE BADGE BAR (SPLIT IN TWO)
    let badge_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(55), // Workspace dir path
                Constraint::Percentage(45), // Branch name & counts
            ]
            .as_ref(),
        )
        .split(chunks[1]);

    // Left Panel: Workspace Directory Path
    let dir_text = Line::from(vec![
        Span::styled("  📂  WORKSPACE: ", Style::default().fg(Color::Rgb(98, 114, 164)).add_modifier(Modifier::BOLD)),
        Span::styled("  ", Style::default()),
        Span::styled(
            &app.current_dir,
            Style::default()
                .fg(Color::Rgb(248, 248, 242))
                .add_modifier(Modifier::BOLD),
        ),
    ]);
    let dir_block = Paragraph::new(dir_text).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Rgb(98, 114, 164)))
            .border_type(ratatui::widgets::BorderType::Rounded),
    );
    f.render_widget(dir_block, badge_chunks[0]);

    // Right Panel: Git Branch & Changes breakdown stats
    let stats_text = if is_vi {
        Line::from(vec![
            Span::styled(" 🌿 ", Style::default().fg(Color::Rgb(80, 250, 123))),
            Span::styled(&app.current_branch, Style::default().fg(Color::Rgb(80, 250, 123)).add_modifier(Modifier::BOLD)),
            Span::styled("  |  🟢 Đã Stage: ", Style::default().fg(Color::Rgb(248, 248, 242))),
            Span::styled(format!("{}", app.staged_count), Style::default().fg(Color::Rgb(80, 250, 123)).add_modifier(Modifier::BOLD)),
            Span::styled("  🟡 Chưa Stage: ", Style::default().fg(Color::Rgb(248, 248, 242))),
            Span::styled(format!("{}", app.unstaged_count), Style::default().fg(Color::Rgb(241, 250, 140)).add_modifier(Modifier::BOLD)),
            Span::styled("  🟣 Chưa theo dõi: ", Style::default().fg(Color::Rgb(248, 248, 242))),
            Span::styled(format!("{}", app.untracked_count), Style::default().fg(Color::Rgb(189, 147, 249)).add_modifier(Modifier::BOLD)),
        ])
    } else {
        Line::from(vec![
            Span::styled(" 🌿 ", Style::default().fg(Color::Rgb(80, 250, 123))),
            Span::styled(&app.current_branch, Style::default().fg(Color::Rgb(80, 250, 123)).add_modifier(Modifier::BOLD)),
            Span::styled("  |  🟢 Staged: ", Style::default().fg(Color::Rgb(248, 248, 242))),
            Span::styled(format!("{}", app.staged_count), Style::default().fg(Color::Rgb(80, 250, 123)).add_modifier(Modifier::BOLD)),
            Span::styled("  🟡 Unstaged: ", Style::default().fg(Color::Rgb(248, 248, 242))),
            Span::styled(format!("{}", app.unstaged_count), Style::default().fg(Color::Rgb(241, 250, 140)).add_modifier(Modifier::BOLD)),
            Span::styled("  🟣 Untracked: ", Style::default().fg(Color::Rgb(248, 248, 242))),
            Span::styled(format!("{}", app.untracked_count), Style::default().fg(Color::Rgb(189, 147, 249)).add_modifier(Modifier::BOLD)),
        ])
    };

    let stats_widget = Paragraph::new(stats_text).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Rgb(139, 233, 253)))
            .border_type(ratatui::widgets::BorderType::Rounded),
    );
    f.render_widget(stats_widget, badge_chunks[1]);

    // Split the main content area into 3 columns horizontally
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(28), // Left: Changes list
                Constraint::Percentage(48), // Middle: Live Diff view
                Constraint::Percentage(24), // Right: Commands Legend
            ]
            .as_ref(),
        )
        .split(chunks[2]);

    // 3. LEFT COLUMN: 📂 CHANGES
    let mut change_lines = vec![Line::from("")];
    if app.files.is_empty() {
        change_lines.push(Line::from(vec![
            Span::styled("   ✨ ", Style::default().fg(Color::Rgb(80, 250, 123))),
            Span::styled(
                if is_vi { "Không có thay đổi!" } else { "No changes detected!" },
                Style::default().fg(Color::Rgb(80, 250, 123)).add_modifier(Modifier::BOLD)
            ),
        ]));
    } else {
        for (i, file) in app.files.iter().enumerate() {
            let is_selected = i == app.selected_index;

            // Check status to determine color badge
            let first_char = file.status.chars().next().unwrap_or(' ');
            let second_char = file.status.chars().nth(1).unwrap_or(' ');

            let is_staged = first_char != ' ' && first_char != '?';
            let is_untracked = first_char == '?' && second_char == '?';
            let is_deleted = first_char == 'D' || second_char == 'D';

            let (badge_text, badge_style) = if is_staged {
                (" [S] ", Style::default().fg(Color::Rgb(80, 250, 123)).add_modifier(Modifier::BOLD)) // Green Staged
            } else if is_untracked {
                (" [?] ", Style::default().fg(Color::Rgb(189, 147, 249)).add_modifier(Modifier::BOLD)) // Purple Untracked
            } else if is_deleted {
                (" [D] ", Style::default().fg(Color::Rgb(255, 85, 85)).add_modifier(Modifier::BOLD)) // Red Deleted
            } else {
                (" [U] ", Style::default().fg(Color::Rgb(241, 250, 140)).add_modifier(Modifier::BOLD)) // Yellow Unstaged
            };

            let cursor_span = if is_selected {
                Span::styled(" ▶ ", Style::default().fg(Color::Rgb(189, 147, 249)).add_modifier(Modifier::BOLD))
            } else {
                Span::styled("   ", Style::default().fg(Color::Rgb(98, 114, 164)))
            };

            let file_style = if is_selected {
                Style::default().fg(Color::Rgb(248, 248, 242)).bg(Color::Rgb(68, 71, 90)).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Rgb(248, 248, 242))
            };

            change_lines.push(Line::from(vec![
                cursor_span,
                Span::styled(badge_text, badge_style),
                Span::styled(file.path.clone(), file_style),
            ]));
        }
    }

    let left_title = if is_vi { " 📂 THAY ĐỔI (CHANGES) " } else { " 📂 WORKSPACE CHANGES " };
    let changes_widget = Paragraph::new(change_lines).block(
        Block::default()
            .title(Span::styled(left_title, Style::default().fg(Color::Rgb(189, 147, 249)).add_modifier(Modifier::BOLD)))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Rgb(189, 147, 249)))
            .border_type(ratatui::widgets::BorderType::Rounded),
    );
    f.render_widget(changes_widget, main_chunks[0]);

    // 4. MIDDLE COLUMN: 📄 LIVE DIFF VIEW
    let mut diff_lines = Vec::new();
    if app.selected_file_diff.is_empty() {
        diff_lines.push(Line::from(""));
        diff_lines.push(Line::from(vec![
            Span::styled(
                if is_vi { "   (Chọn một tập tin để xem thay đổi)" } else { "   (Select a file to preview changes)" },
                Style::default().fg(Color::Rgb(98, 114, 164)).add_modifier(Modifier::ITALIC),
            )
        ]));
    } else {
        for line in app.selected_file_diff.lines() {
            let styled_line = if line.starts_with('+') && !line.starts_with("+++") {
                Line::from(vec![
                    Span::styled(line.to_string(), Style::default().fg(Color::Rgb(80, 250, 123)))
                ])
            } else if line.starts_with('-') && !line.starts_with("---") {
                Line::from(vec![
                    Span::styled(line.to_string(), Style::default().fg(Color::Rgb(255, 85, 85)))
                ])
            } else if line.starts_with("@@") {
                Line::from(vec![
                    Span::styled(line.to_string(), Style::default().fg(Color::Rgb(189, 147, 249)).add_modifier(Modifier::BOLD))
                ])
            } else if line.starts_with("diff --git") || line.starts_with("index") {
                Line::from(vec![
                    Span::styled(line.to_string(), Style::default().fg(Color::Rgb(98, 114, 164)).add_modifier(Modifier::BOLD))
                ])
            } else {
                Line::from(vec![
                    Span::styled(line.to_string(), Style::default().fg(Color::Rgb(248, 248, 242)))
                ])
            };
            diff_lines.push(styled_line);
        }
    }

    // Scroll calculations
    let diff_box_height = if main_chunks[1].height > 2 { (main_chunks[1].height - 2) as usize } else { 0 };
    let max_scroll = if diff_lines.len() > diff_box_height {
        diff_lines.len() - diff_box_height
    } else {
        0
    };
    let scroll_offset = app.diff_scroll_offset.min(max_scroll);

    // Add scroll status info to diff panel title
    let scroll_info = if max_scroll > 0 {
        format!(" [{}/{}]", scroll_offset + 1, diff_lines.len())
    } else {
        "".to_string()
    };
    let diff_title = format!(" 📄 LIVE DIFF VIEW{} ", scroll_info);

    let diff_widget = Paragraph::new(diff_lines)
        .scroll((scroll_offset as u16, 0))
        .block(
            Block::default()
                .title(Span::styled(diff_title, Style::default().fg(Color::Rgb(241, 250, 140)).add_modifier(Modifier::BOLD)))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Rgb(241, 250, 140)))
                .border_type(ratatui::widgets::BorderType::Rounded),
        );
    f.render_widget(diff_widget, main_chunks[1]);

    // Stateful scrollbar overlay inside the diff view
    if max_scroll > 0 {
        let scrollbar = Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("▲"))
            .end_symbol(Some("▼"))
            .track_symbol(Some("░"))
            .thumb_symbol("█");

        let mut scrollbar_state = ScrollbarState::new(max_scroll).position(scroll_offset);
        f.render_stateful_widget(
            scrollbar,
            main_chunks[1].inner(&ratatui::layout::Margin {
                vertical: 1,
                horizontal: 1, // beautiful native overlay inset inside borders
            }),
            &mut scrollbar_state,
        );
    }

    // 5. RIGHT COLUMN: ⚡ KEYBOARD LEGEND
    let mut legend_lines = vec![Line::from("")];
    let groups = vec![
        ("Navigation", vec![
            ("↑/↓ / j/k", if is_vi { "Chọn tập tin" } else { "Select file" }, Color::Rgb(189, 147, 249)),
            ("PgUp/Dn", if is_vi { "Cuộn diff" } else { "Scroll diff" }, Color::Rgb(189, 147, 249)),
        ]),
        ("Git Operations", vec![
            ("Space", if is_vi { "Stage/Unstage" } else { "Stage/Unstage" }, Color::Rgb(80, 250, 123)),
            ("Backspace", if is_vi { "Revert / Xóa" } else { "Revert / Delete" }, Color::Rgb(255, 85, 85)),
            ("A", if is_vi { "Stage tất cả" } else { "Stage all" }, Color::Rgb(80, 250, 123)),
            ("U", if is_vi { "Unstage tất cả" } else { "Unstage all" }, Color::Rgb(255, 85, 85)),
            ("B", if is_vi { "Đổi chi nhánh" } else { "Select branch" }, Color::Rgb(139, 233, 253)),
            ("V", if is_vi { "Xem Lịch sử" } else { "Commit history" }, Color::Rgb(241, 250, 140)),
            ("F", if is_vi { "Tìm nạp (Fetch)" } else { "Git Fetch" }, Color::Rgb(139, 233, 253)),
            ("P", if is_vi { "Cập nhật (Pull)" } else { "Git Pull" }, Color::Rgb(139, 233, 253)),
            ("D", if is_vi { "Copy diff -> AI" } else { "Copy diff -> AI" }, Color::Rgb(241, 250, 140)),
            ("G", if is_vi { "Đóng gói (Go)" } else { "Commit & Push (Go)" }, Color::Rgb(80, 250, 123)),
        ]),
        ("System", vec![
            ("O", if is_vi { "Mở VS Code" } else { "Open VS Code" }, Color::Rgb(255, 121, 198)),
            ("W", if is_vi { "Chọn Project" } else { "Select Project" }, Color::Rgb(139, 233, 253)),
            ("L", if is_vi { "Đổi ngôn ngữ" } else { "Toggle lang" }, Color::Rgb(189, 147, 249)),
            ("R", if is_vi { "Reset cài đặt" } else { "Reset settings" }, Color::Rgb(255, 85, 85)),
            ("? / H", if is_vi { "Mở hướng dẫn" } else { "Open manual" }, Color::Rgb(139, 233, 253)),
            ("Q", if is_vi { "Thoát TUI panel" } else { "Exit TUI panel" }, Color::Rgb(98, 114, 164)),
        ])
    ];

    for (group_title, items) in groups {
        legend_lines.push(Line::from(vec![
            Span::styled(format!("  ■ {}", group_title), Style::default().fg(Color::Rgb(139, 233, 253)).add_modifier(Modifier::BOLD))
        ]));
        for (key, desc, color) in items {
            legend_lines.push(Line::from(vec![
                Span::styled("   ⚡ [", Style::default().fg(Color::Rgb(98, 114, 164))),
                Span::styled(key, Style::default().fg(color).add_modifier(Modifier::BOLD)),
                Span::styled("]  ", Style::default().fg(Color::Rgb(98, 114, 164))),
                Span::styled(desc.to_string(), Style::default().fg(Color::Rgb(248, 248, 242))),
            ]));
        }
        legend_lines.push(Line::from(""));
    }

    let legend_widget = Paragraph::new(legend_lines).block(
        Block::default()
            .title(Span::styled(
                if is_vi { " ⚡ BẢNG PHÍM TẮT " } else { " ⚡ CONTROL LEGEND " },
                Style::default().fg(Color::Rgb(139, 233, 253)).add_modifier(Modifier::BOLD),
            ))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Rgb(139, 233, 253)))
            .border_type(ratatui::widgets::BorderType::Rounded),
    );
    f.render_widget(legend_widget, main_chunks[2]);

    // 6. STATUS MESSAGE BAR
    let is_warning = app.status_message.starts_with("⚠️") || app.status_message.contains("CONFIRM") || app.status_message.contains("XÁC NHẬN");
    let is_error = app.status_message.starts_with("❌") || app.status_message.contains("Error") || app.status_message.contains("Lỗi");
    let is_success = app.status_message.starts_with("✅") || app.status_message.starts_with("🚀") || app.status_message.starts_with("⚡") || app.status_message.starts_with("✨");

    let status_color = if is_warning {
        Color::Rgb(241, 250, 140) // Yellow Warning
    } else if is_error {
        Color::Rgb(255, 85, 85)   // Red Error
    } else if is_success {
        Color::Rgb(80, 250, 123)  // Green Success
    } else {
        Color::Rgb(139, 233, 253) // Cyan Info
    };

    let status_text = Line::from(vec![
        Span::styled(
            if is_vi { "  🔔  THÔNG BÁO HỆ THỐNG  " } else { "  🔔  SYSTEM NOTIFICATION  " },
            Style::default().fg(Color::Rgb(248, 248, 242)).bg(status_color).add_modifier(Modifier::BOLD),
        ),
        Span::styled("  ", Style::default()),
        Span::styled(
            &app.status_message,
            Style::default().fg(status_color).add_modifier(Modifier::BOLD),
        ),
    ]);

    let status_widget = Paragraph::new(status_text).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(status_color))
            .border_type(ratatui::widgets::BorderType::Rounded),
    );
    f.render_widget(status_widget, chunks[3]);

    // 7. RENDER FLOATING MODAL OVERLAYS (LAST IN CANVAS LAYERS)
    match &app.active_modal {
        ActiveModal::Help => {
            let area = centered_rect(65, 75, f.size());
            f.render_widget(Clear, area);

            let mut content = vec![
                Line::from(""),
                Line::from(vec![
                    Span::styled(
                        if is_vi { "🤖 BẢNG HƯỚNG DẪN PHÍM TẮT HỆ THỐNG 🤖" } else { "🤖 SYSTEM MANUAL & KEYBOARD LEGEND 🤖" },
                        Style::default().fg(Color::Rgb(139, 233, 253)).add_modifier(Modifier::BOLD)
                    )
                ]),
                Line::from(""),
            ];

            let shortcut_groups = vec![
                ("Navigation ", vec![
                    ("↑/↓ / j/k", if is_vi { "Chọn tập tin trong danh sách" } else { "Navigate/Select file in change list" }),
                    ("PgUp/PgDn", if is_vi { "Cuộn xem Diff chi tiết" } else { "Scroll detailed code diff viewer" }),
                ]),
                ("Git Operations ", vec![
                    ("Space", if is_vi { "Stage / Unstage (git add / restore)" } else { "Stage / Unstage file (git add / restore)" }),
                    ("Backspace", if is_vi { "Khôi phục / Xóa bỏ thay đổi (git restore)" } else { "Revert / Delete changes (git restore)" }),
                    ("a", if is_vi { "Stage toàn bộ thay đổi (git add .)" } else { "Stage all changes (git add .)" }),
                    ("u", if is_vi { "Unstage toàn bộ thay đổi (git reset)" } else { "Unstage all changes (git reset)" }),
                    ("b", if is_vi { "Xem & Chuyển đổi chi nhánh Git" } else { "View & Select active git branch" }),
                    ("v", if is_vi { "Xem lịch sử commit timeline" } else { "View timeline of last 15 commits" }),
                    ("f", if is_vi { "Tìm nạp toàn bộ metadata từ máy chủ (git fetch)" } else { "Fetch all remote branch metadata (git fetch)" }),
                    ("p", if is_vi { "Cập nhật thay đổi từ máy chủ về máy (git pull)" } else { "Pull latest changes from remote (git pull)" }),
                    ("d", if is_vi { "Chụp ảnh Diff chuyển qua AI Clipboard" } else { "Capture & Copy code diff to AI Clipboard" }),
                    ("g", if is_vi { "Đóng gói toàn bộ, tự động commit & push (Go)" } else { "Commit & Push changes auto (Go)" }),
                ]),
                ("System Operations ", vec![
                    ("o", if is_vi { "Mở thư mục hiện tại bằng VS Code" } else { "Open workspace folder in VS Code" }),
                    ("w", if is_vi { "Đổi sang thư mục Project khác" } else { "Switch to another workspace project" }),
                    ("l", if is_vi { "Thay đổi ngôn ngữ TUI (Language Panel)" } else { "Open language configuration panel" }),
                    ("r", if is_vi { "Khôi phục cài đặt gốc của git-ai" } else { "Reset git-ai to default system settings" }),
                    ("q / Esc", if is_vi { "Đóng cửa sổ / Thoát chương trình" } else { "Close modal / Exit TUI dashboard" }),
                ]),
            ];

            for (group_title, items) in shortcut_groups {
                content.push(Line::from(vec![
                    Span::styled(format!("  ■ {}", group_title), Style::default().fg(Color::Rgb(189, 147, 249)).add_modifier(Modifier::BOLD))
                ]));
                for (key, desc) in items {
                    content.push(Line::from(vec![
                        Span::styled("   ⚡ [", Style::default().fg(Color::Rgb(98, 114, 164))),
                        Span::styled(key, Style::default().fg(Color::Rgb(80, 250, 123)).add_modifier(Modifier::BOLD)),
                        Span::styled("] : ", Style::default().fg(Color::Rgb(98, 114, 164))),
                        Span::styled(desc, Style::default().fg(Color::Rgb(248, 248, 242))),
                    ]));
                }
                content.push(Line::from(""));
            }

            content.push(Line::from(vec![
                Span::styled(
                    if is_vi { "Nhấn [Esc], [Space], hoặc [Enter] để ĐÓNG." } else { "Press [Esc], [Space], or [Enter] to CLOSE." },
                    Style::default().fg(Color::Rgb(255, 184, 108)).add_modifier(Modifier::BOLD)
                )
            ]));

            let block = Block::default()
                .title(Span::styled(" SYSTEM MANUAL ", Style::default().fg(Color::Rgb(139, 233, 253)).add_modifier(Modifier::BOLD)))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Rgb(139, 233, 253)))
                .border_type(BorderType::Double);

            let paragraph = Paragraph::new(content)
                .alignment(ratatui::layout::Alignment::Center)
                .block(block);

            f.render_widget(paragraph, area);
        }
        ActiveModal::LanguageSelect => {
            let area = centered_rect(40, 25, f.size());
            f.render_widget(Clear, area);

            let mut content = vec![
                Line::from(""),
                Line::from(vec![
                    Span::styled(
                        if is_vi { "Chọn ngôn ngữ của bạn / Select your language:" } else { "Select language / Chọn ngôn ngữ:" },
                        Style::default().fg(Color::Rgb(248, 248, 242)).add_modifier(Modifier::ITALIC)
                    )
                ]),
                Line::from(""),
            ];

            let items = vec![
                ("vi", "Tiếng Việt 🇻🇳", "[v]"),
                ("en", "English 🇺🇸", "[e]"),
                ("auto", "Tự động / Auto (System) ⚙️", "[a]"),
            ];

            // Resolve the current raw git config setting dynamically
            let raw_lang = if let Ok(output) = Command::new("git")
                .args(["config", "--global", "--get", "git-ai.lang"])
                .output()
            {
                let stdout = String::from_utf8_lossy(&output.stdout).trim().to_lowercase();
                if stdout == "vi" || stdout == "en" {
                    stdout
                } else {
                    "auto".to_string()
                }
            } else {
                "auto".to_string()
            };

            for (i, (lang_code, label, shortcut)) in items.into_iter().enumerate() {
                let is_hovered = i == app.selected_lang_index;
                let is_currently_active = raw_lang == lang_code;

                let cursor = if is_hovered {
                    Span::styled(" ▶ ", Style::default().fg(Color::Rgb(189, 147, 249)).add_modifier(Modifier::BOLD))
                } else {
                    Span::styled("   ", Style::default())
                };

                let active_badge = if is_currently_active {
                    Span::styled(" (Active) ", Style::default().fg(Color::Rgb(80, 250, 123)).add_modifier(Modifier::ITALIC))
                } else {
                    Span::styled("", Style::default())
                };

                let item_style = if is_hovered {
                    Style::default().fg(Color::Rgb(248, 248, 242)).bg(Color::Rgb(68, 71, 90)).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::Rgb(248, 248, 242))
                };

                content.push(Line::from(vec![
                    cursor,
                    Span::styled(format!("{} ", shortcut), Style::default().fg(Color::Rgb(139, 233, 253)).add_modifier(Modifier::BOLD)),
                    Span::styled(label, item_style),
                    active_badge,
                ]));
            }

            content.push(Line::from(""));
            content.push(Line::from(vec![
                Span::styled(
                    if is_vi { "Dùng ↑/↓ hoặc j/k để di chuyển, Enter để chọn." } else { "Use ↑/↓ or j/k to navigate, Enter to select." },
                    Style::default().fg(Color::Rgb(98, 114, 164))
                )
            ]));

            let block = Block::default()
                .title(Span::styled(
                    if is_vi { " 🌎 THIẾT LẬP NGÔN NGỮ " } else { " 🌎 LANGUAGE CONFIGURATION " },
                    Style::default().fg(Color::Rgb(189, 147, 249)).add_modifier(Modifier::BOLD)
                ))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Rgb(189, 147, 249)))
                .border_type(BorderType::Double);

            let paragraph = Paragraph::new(content)
                .alignment(ratatui::layout::Alignment::Center)
                .block(block);

            f.render_widget(paragraph, area);
        }
        ActiveModal::RevertConfirm(path) => {
            let area = centered_rect(50, 30, f.size());
            f.render_widget(Clear, area); // Clear underlying pixels

            let content = if is_vi {
                vec![
                    Line::from(""),
                    Line::from(vec![
                        Span::styled("⚠️  CẢNH BÁO KHÔI PHỤC HỆ THỐNG  ⚠️", Style::default().fg(Color::Rgb(255, 85, 85)).add_modifier(Modifier::BOLD)),
                    ]),
                    Line::from(""),
                    Line::from(vec![
                        Span::styled("Bạn có chắc chắn muốn khôi phục/xóa các thay đổi trong:", Style::default().fg(Color::Rgb(248, 248, 242))),
                    ]),
                    Line::from(vec![
                        Span::styled(format!("👉 {} ", path), Style::default().fg(Color::Rgb(139, 233, 253)).add_modifier(Modifier::BOLD)),
                    ]),
                    Line::from(""),
                    Line::from(vec![
                        Span::styled("⚠️ HÀNH ĐỘNG NÀY KHÔNG THỂ HOÀN TÁC!", Style::default().fg(Color::Rgb(255, 85, 85)).add_modifier(Modifier::BOLD)),
                    ]),
                    Line::from(""),
                    Line::from(vec![
                        Span::styled(" [y] ĐỒNG Ý ", Style::default().fg(Color::Rgb(248, 248, 242)).bg(Color::Rgb(80, 250, 123)).add_modifier(Modifier::BOLD)),
                        Span::styled("      ", Style::default()),
                        Span::styled(" [n] HỦY BỎ ", Style::default().fg(Color::Rgb(248, 248, 242)).bg(Color::Rgb(255, 85, 85)).add_modifier(Modifier::BOLD)),
                    ]),
                ]
            } else {
                vec![
                    Line::from(""),
                    Line::from(vec![
                        Span::styled("⚠️  SYSTEM REVERT WARNING  ⚠️", Style::default().fg(Color::Rgb(255, 85, 85)).add_modifier(Modifier::BOLD)),
                    ]),
                    Line::from(""),
                    Line::from(vec![
                        Span::styled("Are you sure you want to revert/delete changes in:", Style::default().fg(Color::Rgb(248, 248, 242))),
                    ]),
                    Line::from(vec![
                        Span::styled(format!("👉 {} ", path), Style::default().fg(Color::Rgb(139, 233, 253)).add_modifier(Modifier::BOLD)),
                    ]),
                    Line::from(""),
                    Line::from(vec![
                        Span::styled("⚠️ THIS ACTION CANNOT BE UNDONE!", Style::default().fg(Color::Rgb(255, 85, 85)).add_modifier(Modifier::BOLD)),
                    ]),
                    Line::from(""),
                    Line::from(vec![
                        Span::styled(" [y] CONFIRM ", Style::default().fg(Color::Rgb(248, 248, 242)).bg(Color::Rgb(80, 250, 123)).add_modifier(Modifier::BOLD)),
                        Span::styled("      ", Style::default()),
                        Span::styled(" [n] CANCEL ", Style::default().fg(Color::Rgb(248, 248, 242)).bg(Color::Rgb(255, 85, 85)).add_modifier(Modifier::BOLD)),
                    ]),
                ]
            };

            let block = Block::default()
                .title(Span::styled(" WARNING CONFIRMATION ", Style::default().fg(Color::Rgb(255, 85, 85)).add_modifier(Modifier::BOLD)))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Rgb(255, 85, 85)))
                .border_type(BorderType::Double);

            let paragraph = Paragraph::new(content)
                .alignment(ratatui::layout::Alignment::Center)
                .block(block);

            f.render_widget(paragraph, area);
        }
        ActiveModal::GitLog => {
            let area = centered_rect(75, 70, f.size());
            f.render_widget(Clear, area);

            let mut content = vec![
                Line::from(""),
                Line::from(vec![
                    Span::styled(
                        if is_vi { "🌿 LỊCH SỬ COMMIT WORKSPACE 🌿" } else { "🌿 WORKSPACE COMMIT HISTORY 🌿" },
                        Style::default().fg(Color::Rgb(80, 250, 123)).add_modifier(Modifier::BOLD)
                    )
                ]),
                Line::from(""),
            ];

            if app.commit_logs.is_empty() {
                content.push(Line::from(vec![
                    Span::styled(
                        if is_vi { "Không tìm thấy commit nào." } else { "No commits found." },
                        Style::default().fg(Color::Rgb(98, 114, 164)).add_modifier(Modifier::ITALIC)
                    )
                ]));
            } else {
                for (i, entry) in app.commit_logs.iter().enumerate() {
                    let is_selected = i == app.selected_log_index;
                    let bullet = if is_selected {
                        Span::styled("  ▶ ● ", Style::default().fg(Color::Rgb(189, 147, 249)).add_modifier(Modifier::BOLD))
                    } else {
                        Span::styled("    ● ", Style::default().fg(Color::Rgb(98, 114, 164)))
                    };

                    let hash_span = Span::styled(
                        format!("[{}]", entry.hash),
                        Style::default().fg(Color::Rgb(241, 250, 140)).add_modifier(Modifier::BOLD)
                    );

                    let author_span = Span::styled(
                        format!(" ({})", entry.author),
                        Style::default().fg(Color::Rgb(255, 121, 198))
                    );

                    let time_span = Span::styled(
                        format!(" - {}", entry.time),
                        Style::default().fg(Color::Rgb(139, 233, 253)).add_modifier(Modifier::ITALIC)
                    );

                    let subject_style = if is_selected {
                        Style::default().fg(Color::Rgb(248, 248, 242)).bg(Color::Rgb(68, 71, 90)).add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::Rgb(248, 248, 242))
                    };
                    let subject_span = Span::styled(format!(" : {}", entry.subject), subject_style);

                    content.push(Line::from(vec![
                        bullet,
                        hash_span,
                        author_span,
                        time_span,
                        subject_span,
                    ]));

                    if i < app.commit_logs.len() - 1 {
                        content.push(Line::from(vec![
                            Span::styled("    │", Style::default().fg(Color::Rgb(98, 114, 164)))
                        ]));
                    }
                }
            }

            content.push(Line::from(""));
            content.push(Line::from(vec![
                Span::styled(
                    if is_vi { "   Dùng ↑/↓ hoặc j/k để di chuyển, nhấn [Esc] hoặc [q] để ĐÓNG." } else { "   Use ↑/↓ or j/k to navigate, press [Esc] or [q] to CLOSE." },
                    Style::default().fg(Color::Rgb(255, 184, 108)).add_modifier(Modifier::BOLD)
                )
            ]));

            let block = Block::default()
                .title(Span::styled(
                    if is_vi { " 🌿 LỊCH SỬ COMMIT " } else { " 🌿 COMMIT LOGS " },
                    Style::default().fg(Color::Rgb(80, 250, 123)).add_modifier(Modifier::BOLD)
                ))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Rgb(80, 250, 123)))
                .border_type(BorderType::Double);

            let paragraph = Paragraph::new(content)
                .alignment(ratatui::layout::Alignment::Left)
                .block(block);

            f.render_widget(paragraph, area);
        }
        ActiveModal::BranchSelect => {
            let area = centered_rect(50, 45, f.size());
            f.render_widget(Clear, area);

            let mut content = vec![
                Line::from(""),
                Line::from(vec![
                    Span::styled(
                        if is_vi { "🌿 DANH SÁCH CHI NHÁNH GIT (BRANCHES) 🌿" } else { "🌿 GIT BRANCH SELECTOR 🌿" },
                        Style::default().fg(Color::Rgb(139, 233, 253)).add_modifier(Modifier::BOLD)
                    )
                ]),
                Line::from(""),
            ];

            if app.branches.is_empty() {
                content.push(Line::from(vec![
                    Span::styled(
                        if is_vi { "Không tìm thấy chi nhánh nào." } else { "No branches found." },
                        Style::default().fg(Color::Rgb(98, 114, 164)).add_modifier(Modifier::ITALIC)
                    )
                ]));
            } else {
                for (i, branch) in app.branches.iter().enumerate() {
                    let is_selected = i == app.selected_branch_index;
                    let is_active = branch == &app.current_branch;
                    let cursor = if is_selected {
                        Span::styled(" ▶ ", Style::default().fg(Color::Rgb(189, 147, 249)).add_modifier(Modifier::BOLD))
                    } else {
                        Span::styled("   ", Style::default())
                    };

                    let branch_style = if is_selected {
                        Style::default().fg(Color::Rgb(248, 248, 242)).bg(Color::Rgb(68, 71, 90)).add_modifier(Modifier::BOLD)
                    } else if is_active {
                        Style::default().fg(Color::Rgb(80, 250, 123)).add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::Rgb(248, 248, 242))
                    };

                    let active_badge = if is_active {
                        Span::styled(
                            if is_vi { " (Đang hoạt động) " } else { " (Active) " },
                            Style::default().fg(Color::Rgb(80, 250, 123)).add_modifier(Modifier::ITALIC)
                        )
                    } else {
                        Span::styled("", Style::default())
                    };

                    let prefix = if is_active { "★ " } else { "☆ " };
                    let prefix_span = Span::styled(
                        prefix,
                        if is_active { Style::default().fg(Color::Rgb(80, 250, 123)) } else { Style::default().fg(Color::Rgb(98, 114, 164)) }
                    );

                    content.push(Line::from(vec![
                        cursor,
                        prefix_span,
                        Span::styled(branch.clone(), branch_style),
                        active_badge,
                    ]));
                }
            }

            content.push(Line::from(""));
            content.push(Line::from(vec![
                Span::styled(
                    if is_vi { "Dùng ↑/↓ hoặc j/k để di chuyển, [Enter] để chuyển nhánh." } else { "Use ↑/↓ or j/k to navigate, [Enter] to checkout branch." },
                    Style::default().fg(Color::Rgb(255, 184, 108)).add_modifier(Modifier::BOLD)
                )
            ]));
            content.push(Line::from(vec![
                Span::styled(
                    if is_vi { "Nhấn [Esc] hoặc [q] để HỦY." } else { "Press [Esc] or [q] to CANCEL." },
                    Style::default().fg(Color::Rgb(98, 114, 164))
                )
            ]));

            let block = Block::default()
                .title(Span::styled(
                    if is_vi { " 🌿 CHỌN CHI NHÁNH " } else { " 🌿 SELECT BRANCH " },
                    Style::default().fg(Color::Rgb(139, 233, 253)).add_modifier(Modifier::BOLD)
                ))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Rgb(139, 233, 253)))
                .border_type(BorderType::Double);

            let paragraph = Paragraph::new(content)
                .alignment(ratatui::layout::Alignment::Center)
                .block(block);

            f.render_widget(paragraph, area);
        }
        ActiveModal::DiffResult => {
            let area = centered_rect(72, 72, f.size());
            f.render_widget(Clear, area);

            let mut content = vec![
                Line::from(""),
                Line::from(vec![
                    Span::styled(
                        if is_vi { "🤖 SNAPSHOT DIFF ĐÃ COPY VÀO CLIPBOARD 🤖" }
                        else { "🤖 DIFF SNAPSHOT COPIED TO CLIPBOARD 🤖" },
                        Style::default().fg(Color::Rgb(139, 233, 253)).add_modifier(Modifier::BOLD)
                    )
                ]),
                Line::from(""),
                Line::from(vec![
                    Span::styled("  ➕ ", Style::default().fg(Color::Rgb(80, 250, 123)).add_modifier(Modifier::BOLD)),
                    Span::styled(
                        format!("{} lines added", app.diff_added_lines),
                        Style::default().fg(Color::Rgb(80, 250, 123)).add_modifier(Modifier::BOLD)
                    ),
                    Span::styled("     ➖ ", Style::default().fg(Color::Rgb(255, 85, 85)).add_modifier(Modifier::BOLD)),
                    Span::styled(
                        format!("{} lines removed", app.diff_removed_lines),
                        Style::default().fg(Color::Rgb(255, 85, 85)).add_modifier(Modifier::BOLD)
                    ),
                ]),
                Line::from(""),
                Line::from(vec![
                    Span::styled(
                        if is_vi { "  ── PREVIEW (40 dòng đầu) ──" } else { "  ── DIFF PREVIEW (first 40 lines) ──" },
                        Style::default().fg(Color::Rgb(98, 114, 164)).add_modifier(Modifier::ITALIC)
                    )
                ]),
                Line::from(""),
            ];

            for line in app.diff_snapshot.lines().take(30) {
                let (styled_line, color) = if line.starts_with('+') && !line.starts_with("+++") {
                    (line, Color::Rgb(80, 250, 123))
                } else if line.starts_with('-') && !line.starts_with("---") {
                    (line, Color::Rgb(255, 85, 85))
                } else if line.starts_with("@@") {
                    (line, Color::Rgb(139, 233, 253))
                } else if line.starts_with("diff ") || line.starts_with("index ") {
                    (line, Color::Rgb(189, 147, 249))
                } else {
                    (line, Color::Rgb(98, 114, 164))
                };
                content.push(Line::from(vec![
                    Span::styled(format!("  {}", styled_line), Style::default().fg(color))
                ]));
            }

            if app.diff_snapshot.lines().count() > 30 {
                content.push(Line::from(vec![
                    Span::styled(
                        if is_vi { "  ... (còn nhiều hơn, xem trong AI)" } else { "  ... (more in AI clipboard)" },
                        Style::default().fg(Color::Rgb(255, 184, 108)).add_modifier(Modifier::ITALIC)
                    )
                ]));
            }

            content.push(Line::from(""));
            content.push(Line::from(vec![
                Span::styled("  ✅ ", Style::default().fg(Color::Rgb(80, 250, 123)).add_modifier(Modifier::BOLD)),
                Span::styled(
                    if is_vi { "Prompt + Diff đã được copy! Dán vào AI ngay. 🚀" }
                    else { "Prompt + Diff copied! Paste into your AI now. 🚀" },
                    Style::default().fg(Color::Rgb(241, 250, 140)).add_modifier(Modifier::BOLD)
                ),
            ]));
            content.push(Line::from(""));
            content.push(Line::from(vec![
                Span::styled(
                    if is_vi { "  Nhấn [Enter] hoặc [Esc] để đóng." }
                    else { "  Press [Enter] or [Esc] to close." },
                    Style::default().fg(Color::Rgb(98, 114, 164))
                )
            ]));

            let block = Block::default()
                .title(Span::styled(
                    if is_vi { " 🤖 AI DIFF SNAPSHOT " } else { " 🤖 AI DIFF SNAPSHOT " },
                    Style::default().fg(Color::Rgb(139, 233, 253)).add_modifier(Modifier::BOLD)
                ))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Rgb(139, 233, 253)))
                .border_type(BorderType::Double);

            let paragraph = Paragraph::new(content)
                .alignment(ratatui::layout::Alignment::Left)
                .block(block);

            f.render_widget(paragraph, area);
        }
        ActiveModal::GoConfirm => {
            let area = centered_rect(70, 70, f.size());
            f.render_widget(Clear, area);

            let content = match &app.go_step {
                GoStep::Confirm => {
                    let msg_lines: Vec<&str> = app.commit_message_preview.lines().take(3).collect();
                    let msg_preview = msg_lines.join(" | ");
                    let msg_truncated = if msg_preview.len() > 80 {
                        format!("{}...", &msg_preview[..77])
                    } else {
                        msg_preview
                    };

                    let mut lines = vec![
                        Line::from(""),
                        Line::from(vec![
                            Span::styled(
                                if is_vi { "🚀 XÁC NHẬN ĐÓNG GÓI COMMIT & PUSH 🚀" } else { "🚀 CONFIRM COMMIT & PUSH 🚀" },
                                Style::default().fg(Color::Rgb(80, 250, 123)).add_modifier(Modifier::BOLD)
                            )
                        ]),
                        Line::from(""),
                    ];

                    // ĐÃ THÊM: Duyệt danh sách hiển thị các file được chọn trực quan
                    if app.staged_count > 0 {
                        lines.push(Line::from(vec![
                            Span::styled(
                                if is_vi { "📂 Các file bạn đã chọn để commit:" } else { "📂 Selected files to commit:" },
                                Style::default().fg(Color::Rgb(139, 233, 253)).add_modifier(Modifier::BOLD)
                            )
                        ]));
                        for file in &app.files {
                            let first_char = file.status.chars().next().unwrap_or(' ');
                            // Nếu ký tự đầu tiên không phải khoảng trắng hoặc dấu chấm hỏi -> File đang được Stage
                            if first_char != ' ' && first_char != '?' {
                                lines.push(Line::from(vec![
                                    Span::styled("   🟢 ", Style::default().fg(Color::Rgb(80, 250, 123))),
                                    Span::styled(file.path.clone(), Style::default().fg(Color::Rgb(248, 248, 242))),
                                ]));
                            }
                        }
                    } else {
                        lines.push(Line::from(vec![
                            Span::styled(
                                if is_vi { "⚠️ CẢNH BÁO: Chưa chọn file nào! Vui lòng thoát ra nhấn phím [Space] để chọn." }
                                else { "⚠️ WARNING: No files selected! Please exit and press [Space] to select." },
                                Style::default().fg(Color::Rgb(255, 85, 85)).add_modifier(Modifier::BOLD)
                            )
                        ]));
                    }

                    lines.extend(vec![
                        Line::from(""),
                        Line::from(vec![
                            Span::styled(
                                if is_vi { "📋 Commit message từ Clipboard:" } else { "📋 Commit message from Clipboard:" },
                                Style::default().fg(Color::Rgb(98, 114, 164)).add_modifier(Modifier::ITALIC)
                            )
                        ]),
                        Line::from(vec![
                            Span::styled(
                                format!("  💬 {}", msg_truncated),
                                Style::default().fg(Color::Rgb(248, 248, 242)).bg(Color::Rgb(40, 42, 54)).add_modifier(Modifier::BOLD)
                            )
                        ]),
                        Line::from(""),
                        Line::from(vec![
                            Span::styled(
                                if is_vi { "  ⚡ Tiến trình: git commit -> git push" } else { "  ⚡ Execution: git commit -> git push" },
                                Style::default().fg(Color::Rgb(255, 184, 108))
                            )
                        ]),
                        Line::from(""),
                    ]);

                    if app.staged_count > 0 {
                        lines.push(Line::from(vec![
                            Span::styled(" [y] / Enter ", Style::default().fg(Color::Rgb(40, 42, 54)).bg(Color::Rgb(80, 250, 123)).add_modifier(Modifier::BOLD)),
                            Span::styled(" TIẾN HÀNH          ", Style::default().fg(Color::Rgb(80, 250, 123)).add_modifier(Modifier::BOLD)),
                            Span::styled(" [n] / Esc ", Style::default().fg(Color::Rgb(248, 248, 242)).bg(Color::Rgb(255, 85, 85)).add_modifier(Modifier::BOLD)),
                            Span::styled(" HỦY ", Style::default().fg(Color::Rgb(255, 85, 85)).add_modifier(Modifier::BOLD)),
                        ]));
                    } else {
                        lines.push(Line::from(vec![
                            Span::styled(" [Esc] ", Style::default().fg(Color::Rgb(248, 248, 242)).bg(Color::Rgb(255, 85, 85)).add_modifier(Modifier::BOLD)),
                            Span::styled(" QUAY LẠI CHỌN FILE ", Style::default().fg(Color::Rgb(255, 85, 85)).add_modifier(Modifier::BOLD)),
                        ]));
                    }
                    lines.push(Line::from(""));
                    lines
                }
                GoStep::Pushing => {
                    vec![
                        Line::from(""),
                        Line::from(vec![
                            Span::styled(
                                if is_vi { "⚡ ĐANG XỬ LÝ... ⚡" } else { "⚡ PROCESSING... ⚡" },
                                Style::default().fg(Color::Rgb(241, 250, 140)).add_modifier(Modifier::BOLD | Modifier::SLOW_BLINK)
                            )
                        ]),
                        Line::from(""),
                        Line::from(vec![
                            Span::styled(
                                if is_vi { "  🔄 Đang chạy: git commit → git push" }
                                else { "  🔄 Running: git commit → git push" },
                                Style::default().fg(Color::Rgb(139, 233, 253))
                            )
                        ]),
                        Line::from(""),
                        Line::from(vec![
                            Span::styled(
                                if is_vi { "  Vui lòng chờ..." } else { "  Please wait..." },
                                Style::default().fg(Color::Rgb(98, 114, 164)).add_modifier(Modifier::ITALIC)
                            )
                        ]),
                        Line::from(""),
                    ]
                }
                GoStep::Done(result) => {
                    let result_color = if result.starts_with("✅") { Color::Rgb(80, 250, 123) } else { Color::Rgb(255, 85, 85) };
                    let mut lines = vec![
                        Line::from(""),
                        Line::from(vec![
                            Span::styled(if is_vi { "📋 KẾT QUẢ" } else { "📋 RESULT" }, Style::default().fg(Color::Rgb(189, 147, 249)).add_modifier(Modifier::BOLD))
                        ]),
                        Line::from(""),
                    ];
                    for l in result.lines() {
                        lines.push(Line::from(vec![
                            Span::styled(format!("  {}", l), Style::default().fg(result_color).add_modifier(Modifier::BOLD))
                        ]));
                    }
                    lines.push(Line::from(""));
                    lines.push(Line::from(vec![
                        Span::styled(
                            if is_vi { "  Nhấn [Enter] hoặc [Esc] để đóng và làm mới." } else { "  Press [Enter] or [Esc] to close and refresh." },
                            Style::default().fg(Color::Rgb(98, 114, 164))
                        )
                    ]));
                    lines
                }
            };

            let (title, border_color) = match &app.go_step {
                GoStep::Confirm => (if is_vi { " 🚀 COMMIT & PUSH " } else { " 🚀 COMMIT & PUSH " }, Color::Rgb(80, 250, 123)),
                GoStep::Pushing => (if is_vi { " ⚡ ĐANG TIẾN HÀNH " } else { " ⚡ PROCESSING " }, Color::Rgb(241, 250, 140)),
                GoStep::Done(r) => (
                    if r.starts_with("✅") { if is_vi { " ✅ THÀNH CÔNG " } else { " ✅ SUCCESS " } }
                    else { if is_vi { " ❌ THẤT BẠI " } else { " ❌ FAILED " } },
                    if r.starts_with("✅") { Color::Rgb(80, 250, 123) } else { Color::Rgb(255, 85, 85) }
                ),
            };

            let block = Block::default()
                .title(Span::styled(title, Style::default().fg(border_color).add_modifier(Modifier::BOLD)))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(border_color))
                .border_type(BorderType::Double);

            let paragraph = Paragraph::new(content)
                .alignment(ratatui::layout::Alignment::Left)
                .block(block);

            f.render_widget(paragraph, area);
        }
        ActiveModal::StashList => {
            let area = centered_rect(70, 65, f.size());
            f.render_widget(Clear, area);

            let mut content = vec![
                Line::from(""),
                Line::from(vec![
                    Span::styled(
                        if is_vi { "📦 QUẢN LÝ STASH — GIT STASH MANAGER" }
                        else { "📦 GIT STASH MANAGER" },
                        Style::default().fg(Color::Rgb(255, 184, 108)).add_modifier(Modifier::BOLD)
                    )
                ]),
                Line::from(""),
            ];

            match &app.stash_step.clone() {
                StashStep::List => {
                    if app.stash_entries.is_empty() {
                        content.push(Line::from(vec![
                            Span::styled(
                                if is_vi { "  (Không có stash nào)" } else { "  (No stashes found)" },
                                Style::default().fg(Color::Rgb(98, 114, 164)).add_modifier(Modifier::ITALIC)
                            )
                        ]));
                        content.push(Line::from(""));
                        content.push(Line::from(vec![
                            Span::styled(
                                if is_vi { "  Nhấn [n] để stash thay đổi hiện tại" }
                                else { "  Press [n] to stash current changes" },
                                Style::default().fg(Color::Rgb(139, 233, 253))
                            )
                        ]));
                    } else {
                        for (i, entry) in app.stash_entries.iter().enumerate() {
                            let is_sel = i == app.selected_stash_index;
                            let cursor = if is_sel { " ▶ " } else { "   " };
                            let row_style = if is_sel {
                                Style::default().fg(Color::Rgb(248, 248, 242)).bg(Color::Rgb(68, 71, 90)).add_modifier(Modifier::BOLD)
                            } else {
                                Style::default().fg(Color::Rgb(248, 248, 242))
                            };
                            content.push(Line::from(vec![
                                Span::styled(cursor, Style::default().fg(Color::Rgb(255, 184, 108)).add_modifier(Modifier::BOLD)),
                                Span::styled(format!("[{}] ", entry.index), Style::default().fg(Color::Rgb(189, 147, 249)).add_modifier(Modifier::BOLD)),
                                Span::styled(format!("({}) ", entry.branch), Style::default().fg(Color::Rgb(139, 233, 253))),
                                Span::styled(entry.message.clone(), row_style),
                            ]));
                        }
                        content.push(Line::from(""));
                        content.push(Line::from(vec![
                            Span::styled(
                                if is_vi { "  [n] Stash mới  [Enter/p] Pop  [a] Apply  [x] Xóa  [Esc] Đóng" }
                                else { "  [n] New Stash  [Enter/p] Pop  [a] Apply  [x] Drop  [Esc] Close" },
                                Style::default().fg(Color::Rgb(255, 184, 108)).add_modifier(Modifier::BOLD)
                            )
                        ]));
                    }
                }
                StashStep::Confirm(idx, action) => {
                    let action_str = match action {
                        StashAction::Pop => if is_vi { "POP (apply + xóa)" } else { "POP (apply + drop)" },
                        StashAction::Apply => if is_vi { "APPLY (giữ lại stash)" } else { "APPLY (keep stash)" },
                        StashAction::Drop => if is_vi { "XÓA stash" } else { "DROP stash" },
                    };
                    let action_color = match action {
                        StashAction::Drop => Color::Rgb(255, 85, 85),
                        _ => Color::Rgb(80, 250, 123),
                    };
                    content.push(Line::from(vec![
                        Span::styled(
                            format!("  ⚠️  Xác nhận {} stash@{{{}}}?", action_str, idx),
                            Style::default().fg(action_color).add_modifier(Modifier::BOLD)
                        )
                    ]));
                    content.push(Line::from(""));
                    content.push(Line::from(vec![
                        Span::styled(" [y] XÁC NHẬN ", Style::default().fg(Color::Rgb(40, 42, 54)).bg(action_color).add_modifier(Modifier::BOLD)),
                        Span::styled("    ", Style::default()),
                        Span::styled(" [n/Esc] HỦY ", Style::default().fg(Color::Rgb(248, 248, 242)).bg(Color::Rgb(68, 71, 90)).add_modifier(Modifier::BOLD)),
                    ]));
                }
            }

            let block = Block::default()
                .title(Span::styled(
                    if is_vi { " 📦 STASH MANAGER " } else { " 📦 STASH MANAGER " },
                    Style::default().fg(Color::Rgb(255, 184, 108)).add_modifier(Modifier::BOLD)
                ))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Rgb(255, 184, 108)))
                .border_type(BorderType::Double);

            let paragraph = Paragraph::new(content)
                .alignment(ratatui::layout::Alignment::Left)
                .block(block);
            f.render_widget(paragraph, area);
        }
        ActiveModal::RemoteInfo => {
            let area = centered_rect(65, 55, f.size());
            f.render_widget(Clear, area);

            let ahead_color = if app.ahead_count > 0 { Color::Rgb(80, 250, 123) } else { Color::Rgb(98, 114, 164) };
            let behind_color = if app.behind_count > 0 { Color::Rgb(255, 85, 85) } else { Color::Rgb(98, 114, 164) };

            let content = vec![
                Line::from(""),
                Line::from(vec![
                    Span::styled(
                        if is_vi { "🌐 THÔNG TIN REMOTE & TRACKING" }
                        else { "🌐 REMOTE & UPSTREAM INFO" },
                        Style::default().fg(Color::Rgb(139, 233, 253)).add_modifier(Modifier::BOLD)
                    )
                ]),
                Line::from(""),
                Line::from(vec![
                    Span::styled("  🌿 Branch:   ", Style::default().fg(Color::Rgb(98, 114, 164))),
                    Span::styled(app.current_branch.clone(), Style::default().fg(Color::Rgb(80, 250, 123)).add_modifier(Modifier::BOLD)),
                ]),
                Line::from(vec![
                    Span::styled("  🔗 Tracking: ", Style::default().fg(Color::Rgb(98, 114, 164))),
                    Span::styled(app.remote_tracking.clone(), Style::default().fg(Color::Rgb(255, 184, 108)).add_modifier(Modifier::BOLD)),
                ]),
                Line::from(vec![
                    Span::styled("  📡 Remote:   ", Style::default().fg(Color::Rgb(98, 114, 164))),
                    Span::styled(app.remote_url.clone(), Style::default().fg(Color::Rgb(189, 147, 249))),
                ]),
                Line::from(""),
                Line::from(vec![
                    Span::styled("  ↑ Ahead:  ", Style::default().fg(Color::Rgb(98, 114, 164))),
                    Span::styled(
                        format!("{} commit(s) ahead of remote", app.ahead_count),
                        Style::default().fg(ahead_color).add_modifier(Modifier::BOLD)
                    ),
                ]),
                Line::from(vec![
                    Span::styled("  ↓ Behind: ", Style::default().fg(Color::Rgb(98, 114, 164))),
                    Span::styled(
                        format!("{} commit(s) behind remote", app.behind_count),
                        Style::default().fg(behind_color).add_modifier(Modifier::BOLD)
                    ),
                ]),
                Line::from(""),
                Line::from(vec![
                    Span::styled(
                        if app.ahead_count > 0 && app.behind_count == 0 {
                            if is_vi { "  💡 Bạn có thể push lên remote" } else { "  💡 You can push to remote" }
                        } else if app.behind_count > 0 {
                            if is_vi { "  ⚠️  Hãy git pull trước khi push" } else { "  ⚠️  Run git pull before pushing" }
                        } else {
                            if is_vi { "  ✅ Đồng bộ với remote" } else { "  ✅ In sync with remote" }
                        },
                        Style::default().fg(Color::Rgb(241, 250, 140)).add_modifier(Modifier::ITALIC)
                    )
                ]),
                Line::from(""),
                Line::from(vec![
                    Span::styled(
                        if is_vi { "  [Esc] hoặc [Enter] để đóng" } else { "  [Esc] or [Enter] to close" },
                        Style::default().fg(Color::Rgb(98, 114, 164))
                    )
                ]),
            ];

            let block = Block::default()
                .title(Span::styled(
                    " 🌐 REMOTE INFO ",
                    Style::default().fg(Color::Rgb(139, 233, 253)).add_modifier(Modifier::BOLD)
                ))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Rgb(139, 233, 253)))
                .border_type(BorderType::Double);

            let paragraph = Paragraph::new(content)
                .alignment(ratatui::layout::Alignment::Left)
                .block(block);
            f.render_widget(paragraph, area);
        }
        ActiveModal::AmendCommit => {
            let area = centered_rect(68, 50, f.size());
            f.render_widget(Clear, area);

            let content = match &app.amend_step {
                AmendStep::Edit => {
                    let display_msg = if app.amend_message.len() > 70 {
                        format!("{}...", &app.amend_message[..67])
                    } else {
                        app.amend_message.clone()
                    };
                    vec![
                        Line::from(""),
                        Line::from(vec![
                            Span::styled(
                                if is_vi { "✏️  SỬA COMMIT CUỐI (AMEND)" } else { "✏️  AMEND LAST COMMIT" },
                                Style::default().fg(Color::Rgb(255, 184, 108)).add_modifier(Modifier::BOLD)
                            )
                        ]),
                        Line::from(""),
                        Line::from(vec![
                            Span::styled(
                                if is_vi { "  ⚠️  Lưu ý: Nếu đã push, cần force push sau khi amend!" }
                                else { "  ⚠️  Note: If already pushed, you'll need to force push after amend!" },
                                Style::default().fg(Color::Rgb(255, 85, 85)).add_modifier(Modifier::ITALIC)
                            )
                        ]),
                        Line::from(""),
                        Line::from(vec![
                            Span::styled(
                                if is_vi { "  Commit message mới (chỉnh sửa bên dưới):" }
                                else { "  New commit message (edit below):" },
                                Style::default().fg(Color::Rgb(98, 114, 164))
                            )
                        ]),
                        Line::from(vec![
                            Span::styled("  ┌─── ", Style::default().fg(Color::Rgb(255, 184, 108))),
                            Span::styled(
                                format!("{}_", display_msg),
                                Style::default().fg(Color::Rgb(248, 248, 242)).bg(Color::Rgb(40, 42, 54)).add_modifier(Modifier::BOLD)
                            ),
                        ]),
                        Line::from(""),
                        Line::from(vec![
                            Span::styled(
                                if is_vi { "  Nhập để chỉnh sửa, [Enter] để xác nhận, [Esc] để hủy" }
                                else { "  Type to edit, [Enter] to confirm, [Esc] to cancel" },
                                Style::default().fg(Color::Rgb(98, 114, 164))
                            )
                        ]),
                    ]
                }
                AmendStep::Pushing => {
                    vec![
                        Line::from(""),
                        Line::from(vec![
                            Span::styled(
                                if is_vi { "⚡ ĐANG AMEND..." } else { "⚡ AMENDING..." },
                                Style::default().fg(Color::Rgb(241, 250, 140)).add_modifier(Modifier::BOLD | Modifier::SLOW_BLINK)
                            )
                        ]),
                        Line::from(""),
                    ]
                }
                AmendStep::Done(result) => {
                    let color = if result.starts_with("✅") { Color::Rgb(80, 250, 123) } else { Color::Rgb(255, 85, 85) };
                    vec![
                        Line::from(""),
                        Line::from(vec![
                            Span::styled(result.clone(), Style::default().fg(color).add_modifier(Modifier::BOLD))
                        ]),
                        Line::from(""),
                        Line::from(vec![
                            Span::styled(
                                if is_vi { "  [Enter/Esc] để đóng" } else { "  [Enter/Esc] to close" },
                                Style::default().fg(Color::Rgb(98, 114, 164))
                            )
                        ]),
                    ]
                }
            };

            let block = Block::default()
                .title(Span::styled(
                    " ✏️  AMEND COMMIT ",
                    Style::default().fg(Color::Rgb(255, 184, 108)).add_modifier(Modifier::BOLD)
                ))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Rgb(255, 184, 108)))
                .border_type(BorderType::Double);

            let paragraph = Paragraph::new(content)
                .alignment(ratatui::layout::Alignment::Left)
                .block(block);
            f.render_widget(paragraph, area);
        }
        ActiveModal::CommitDiff(hash) => {
            let area = centered_rect(88, 88, f.size());
            f.render_widget(Clear, area);

            let hash = hash.clone();
            let lines: Vec<&str> = app.commit_diff_content.lines().collect();
            let max_scroll = lines.len().saturating_sub(5);
            let scroll = app.commit_diff_scroll.min(max_scroll);
            let visible_lines: Vec<&str> = lines.iter().skip(scroll).take(60).cloned().collect();

            let mut content = vec![
                Line::from(vec![
                    Span::styled(format!("  🔍 Commit: {}", hash), Style::default().fg(Color::Rgb(241, 250, 140)).add_modifier(Modifier::BOLD)),
                    Span::styled(
                        format!("  [{}/{}]", scroll + 1, lines.len().max(1)),
                        Style::default().fg(Color::Rgb(98, 114, 164))
                    ),
                ]),
                Line::from(""),
            ];

            for line in visible_lines {
                let (_text, color) = if line.starts_with('+') && !line.starts_with("+++") {
                    (line, Color::Rgb(80, 250, 123))
                } else if line.starts_with('-') && !line.starts_with("---") {
                    (line, Color::Rgb(255, 85, 85))
                } else if line.starts_with("@@") {
                    (line, Color::Rgb(139, 233, 253))
                } else if line.starts_with("commit ") || line.starts_with("Author:") || line.starts_with("Date:") {
                    (line, Color::Rgb(189, 147, 249))
                } else if line.starts_with("diff ") || line.starts_with("index ") || line.starts_with("---") || line.starts_with("+++") {
                    (line, Color::Rgb(98, 114, 164))
                } else {
                    (line, Color::Rgb(248, 248, 242))
                };
                content.push(Line::from(vec![
                    Span::styled(line.to_string(), Style::default().fg(color))
                ]));
            }

            content.push(Line::from(""));
            content.push(Line::from(vec![
                Span::styled(
                    if is_vi { "  ↑/↓ j/k cuộn  PgUp/PgDn  [Esc/q] Quay lại lịch sử" }
                    else { "  ↑/↓ j/k scroll  PgUp/PgDn  [Esc/q] Back to history" },
                    Style::default().fg(Color::Rgb(255, 184, 108)).add_modifier(Modifier::BOLD)
                )
            ]));

            let block = Block::default()
                .title(Span::styled(
                    format!(" 🔍 COMMIT DIFF — {} ", hash),
                    Style::default().fg(Color::Rgb(241, 250, 140)).add_modifier(Modifier::BOLD)
                ))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Rgb(241, 250, 140)))
                .border_type(BorderType::Double);

            let paragraph = Paragraph::new(content)
                .alignment(ratatui::layout::Alignment::Left)
                .block(block);
            f.render_widget(paragraph, area);
        }
        ActiveModal::None => {}
    }
}
