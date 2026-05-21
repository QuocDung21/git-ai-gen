use std::env;
pub mod models;
pub mod events;

use models::*;

pub struct App {
    pub status_message: String,
    pub git_status_lines: Vec<String>,
    pub current_lang: String,
    pub current_dir: String,
    pub files: Vec<ChangedFile>,
    pub selected_index: usize,
    pub selected_file_diff: String,
    pub diff_scroll_offset: usize,
    pub active_modal: ActiveModal,
    pub selected_lang_index: usize,
    // Real-time Git Statistics
    pub current_branch: String,
    pub staged_count: usize,
    pub unstaged_count: usize,
    pub untracked_count: usize,
    // Commit history state
    pub commit_logs: Vec<CommitLogEntry>,
    pub selected_log_index: usize,
    // Branch switcher state
    pub branches: Vec<String>,
    pub selected_branch_index: usize,
    // Diff result modal
    pub diff_snapshot: String,
    pub diff_added_lines: usize,
    pub diff_removed_lines: usize,
    // Go confirm modal
    pub commit_message_preview: String,
    pub go_step: GoStep,
    pub go_result: String,
    // Inline commit input (Tab to toggle)
    pub commit_input_mode: bool,
    pub commit_input_text: String,
    // Stash manager
    pub stash_entries: Vec<StashEntry>,
    pub selected_stash_index: usize,
    pub stash_step: StashStep,
    // Remote info
    pub remote_url: String,
    pub remote_tracking: String,
    pub ahead_count: i32,
    pub behind_count: i32,
    // Amend commit
    pub amend_step: AmendStep,
    pub amend_message: String,
    // Commit diff viewer
    pub commit_diff_content: String,
    pub commit_diff_scroll: usize,
    // Conflict detection
    pub has_conflicts: bool,
    pub conflict_count: usize,
}

impl App {
    pub fn new() -> Self {
        let current_dir = env::current_dir()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|_| "Unknown".to_string());

        let current_lang = crate::helper::Helper::get_ai_language();
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

    pub fn refresh_git_status(&mut self) {
        let prev_selected_path = if self.files.is_empty() || self.selected_index >= self.files.len()
        {
            None
        } else {
            Some(self.files[self.selected_index].path.clone())
        };

        self.git_status_lines.clear();
        self.files.clear();
        if let Ok(status_text) = crate::git::status::get_git_status() {
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
                        let status = line[..3].to_string();
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
        self.current_branch = crate::git::status::get_current_branch();

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
            let is_conflict = matches!(
                status_trimmed,
                "UU" | "AA" | "DD" | "DU" | "UD" | "AU" | "UA"
            );
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

    pub fn update_selected_diff(&mut self) {
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
                }
                .to_string()
            }
        } else {
            let mut diff_output = crate::git::status::get_diff_head(&file.path);

            if diff_output.is_none() {
                diff_output = crate::git::status::get_diff_unstaged(&file.path);
            }

            if diff_output.is_none() {
                diff_output = crate::git::status::get_diff_staged(&file.path);
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

    pub fn fetch_commit_logs(&mut self) {
        self.commit_logs.clear();
        if let Ok(output) = std::process::Command::new("git")
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

    pub fn fetch_branches(&mut self) {
        self.branches = crate::git::branch::get_branches();
        if let Some(idx) = self.branches.iter().position(|b| b == &self.current_branch) {
            self.selected_branch_index = idx;
        } else {
            self.selected_branch_index = 0;
        }
    }

    pub fn fetch_stash(&mut self) {
        self.stash_entries.clear();
        if let Ok(text) = crate::git::stash::get_stash_list() {
            for (i, line) in text.lines().enumerate() {
                let parts: Vec<&str> = line.splitn(2, '|').collect();
                if parts.len() == 2 {
                    let info = parts[1];
                    let (branch, message) = if let Some(rest) = info.strip_prefix("On ") {
                        if let Some(colon_idx) = rest.find(": ") {
                            (
                                rest[..colon_idx].to_string(),
                                rest[colon_idx + 2..].to_string(),
                            )
                        } else {
                            ("?".to_string(), info.to_string())
                        }
                    } else if let Some(rest) = info.strip_prefix("WIP on ") {
                        if let Some(colon_idx) = rest.find(": ") {
                            (
                                rest[..colon_idx].to_string(),
                                rest[colon_idx + 2..].to_string(),
                            )
                        } else {
                            ("?".to_string(), info.to_string())
                        }
                    } else {
                        ("?".to_string(), info.to_string())
                    };
                    self.stash_entries.push(StashEntry {
                        index: i,
                        branch,
                        message,
                    });
                }
            }
        }
        if self.selected_stash_index >= self.stash_entries.len() {
            self.selected_stash_index = 0;
        }
        self.stash_step = StashStep::List;
    }

    pub fn fetch_remote_info(&mut self) {
        self.remote_url = crate::git::remote::get_remote_url();
        let remote_name = crate::git::remote::get_remote_name(&self.current_branch);
        self.remote_tracking = format!("{}/{}", remote_name, self.current_branch);
        let (ahead, behind) = crate::git::remote::get_ahead_behind(&self.current_branch, &self.remote_tracking);
        self.ahead_count = ahead;
        self.behind_count = behind;
    }

    pub fn fetch_amend_msg(&mut self) {
        self.amend_message = crate::git::commit::get_last_commit_subject();
        self.amend_step = AmendStep::Edit;
    }

    pub fn fetch_commit_diff(&mut self, hash: &str) {
        self.commit_diff_content = crate::git::commit::get_commit_diff(hash);
        self.commit_diff_scroll = 0;
    }
}
