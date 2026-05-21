use std::env;
use std::process::Command;
use ratatui::style::Color;
pub mod events;
pub mod models;

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
    pub branches: Vec<BranchEntry>,
    pub selected_branch_index: usize,
    // Diff result modal
    pub diff_snapshot: String,
    pub diff_added_lines: usize,
    pub diff_removed_lines: usize,
    pub diff_kilo_generated: String,
    pub last_staged_diff: String,
    pub current_kilo_model: String,
    pub kilo_models: Vec<String>,
    pub selected_kilo_model_index: usize,
    pub kilo_generating: bool,
    pub kilo_generation_status: String,
    pub kilo_model_filter: String,
    pub kilo_model_search_mode: bool,
    // Manual commit
    pub manual_commit_message: String,
    pub selected_git_action: usize,
    // Go confirm modal
    pub commit_message_preview: String,
    pub go_step: GoStep,
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
    pub remotes: Vec<RemoteEntry>,
    // Amend commit
    pub amend_step: AmendStep,
    pub amend_message: String,
    // Commit diff viewer
    pub commit_diff_content: String,
    pub commit_diff_scroll: usize,
    // Conflict detection
    pub has_conflicts: bool,
    pub conflict_count: usize,
    pub new_branch_name: String,
    pub is_light_theme: bool,
    pub selected_theme_index: usize,
    pub focus_diff: bool,
    // Workspace history
    pub workspace_history: Vec<String>,
    pub selected_workspace_index: usize,
    pub prompt_text: String,
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

        let is_light_theme = {
            if let Ok(output) = std::process::Command::new("git")
                .args(["config", "--global", "--get", "git-ai.theme"])
                .output()
            {
                let text = String::from_utf8_lossy(&output.stdout).trim().to_lowercase();
                if text == "light" {
                    true
                } else if text == "dark" {
                    false
                } else {
                    crate::helper::Helper::get_os_theme() == dark_light::Mode::Light
                }
            } else {
                crate::helper::Helper::get_os_theme() == dark_light::Mode::Light
            }
        };

        let selected_theme_index = if is_light_theme { 1 } else { 0 };

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
            diff_kilo_generated: String::new(),
            last_staged_diff: String::new(),
            current_kilo_model: String::new(),
            kilo_models: Vec::new(),
            selected_kilo_model_index: 0,
            kilo_generating: false,
            kilo_generation_status: String::new(),
            kilo_model_filter: String::new(),
            kilo_model_search_mode: false,
            manual_commit_message: String::new(),
            selected_git_action: 0,
            commit_message_preview: String::new(),
            go_step: GoStep::Confirm,
            commit_input_mode: false,
            commit_input_text: String::new(),
            stash_entries: Vec::new(),
            selected_stash_index: 0,
            stash_step: StashStep::List,
            remote_url: String::new(),
            remote_tracking: String::new(),
            ahead_count: 0,
            behind_count: 0,
            remotes: Vec::new(),
            amend_step: AmendStep::Edit,
            amend_message: String::new(),
            commit_diff_content: String::new(),
            commit_diff_scroll: 0,
            has_conflicts: false,
            conflict_count: 0,
            new_branch_name: String::new(),
            is_light_theme,
            selected_theme_index,
            focus_diff: false,
            workspace_history: Vec::new(),
            selected_workspace_index: 0,
            prompt_text: String::new(),
        };
        app.load_workspace_history();
        app.add_to_workspace_history(&app.current_dir.clone());
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
            .args([
                "log",
                "--pretty=format:%H|%h|%an|%ae|%ar|%s",
                "--parents",
                "--topo-order",
                "-n",
                "25",
            ])
            .output()
        {
            let logs_text = String::from_utf8_lossy(&output.stdout);
            for line in logs_text.lines() {
                let parts: Vec<&str> = line.split('|').collect();
                if parts.len() >= 6 {
                    let full_hash = parts[0].to_string();
                    let short_hash = parts[1].to_string();
                    let author = parts[2].to_string();
                    let author_email = parts[3].to_string();
                    let time = parts[4].to_string();
                    let subject = parts[5..].join("|");

                    // parents are in the format after %H, before %h in some formats.
                    // Better way: use --parents and parse the line differently.
                    // For simplicity, we'll fetch parents separately or use a better format.
                    // Current simple approach: we'll improve later.
                    let parents: Vec<String> = vec![];

                    self.commit_logs.push(CommitLogEntry {
                        hash: full_hash,
                        short_hash,
                        author,
                        author_email,
                        time,
                        subject,
                        parents,
                    });
                }
            }
        }
    }

    pub fn fetch_commit_tree(&mut self) {
        self.commit_logs.clear();
        if let Ok(output) = std::process::Command::new("git")
            .args([
                "log",
                "--pretty=format:%H|%h|%P|%an|%ae|%ar|%s",
                "--topo-order",
                "-n",
                "30",
            ])
            .output()
        {
            let logs_text = String::from_utf8_lossy(&output.stdout);
            for line in logs_text.lines() {
                let parts: Vec<&str> = line.split('|').collect();
                if parts.len() >= 7 {
                    let full_hash = parts[0].to_string();
                    let short_hash = parts[1].to_string();
                    let parents_str = parts[2];
                    let parents: Vec<String> = if parents_str.is_empty() {
                        vec![]
                    } else {
                        parents_str
                            .split_whitespace()
                            .map(|p| p[..7.min(p.len())].to_string())
                            .collect()
                    };

                    self.commit_logs.push(CommitLogEntry {
                        hash: full_hash,
                        short_hash,
                        author: parts[3].to_string(),
                        author_email: parts[4].to_string(),
                        time: parts[5].to_string(),
                        subject: parts[6..].join("|"),
                        parents,
                    });
                }
            }
        }
    }

    pub fn fetch_branches(&mut self) {
        self.branches = crate::git::branch::get_branches();
        if let Some(idx) = self.branches.iter().position(|b| b.name == self.current_branch && !b.is_remote) {
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
        let (ahead, behind) =
            crate::git::remote::get_ahead_behind(&self.current_branch, &self.remote_tracking);
        self.ahead_count = ahead;
        self.behind_count = behind;
        self.remotes = crate::git::remote::get_remotes();
    }

    pub fn fetch_prompt(&mut self) {
        let ai_lang = crate::helper::Helper::get_ai_language();
        self.prompt_text = format!(
            "{}{}.",
            crate::constant::Constant::PROMPT_EXPERT,
            ai_lang
        );
    }

    pub fn try_generate_with_kilo(&mut self, full_diff: &str) -> Result<String, String> {
        let ai_lang = crate::helper::Helper::get_ai_language();
        let prompt = format!(
            "{} {}.\n\nDiff:\n\n{}",
            crate::constant::Constant::PROMPT_EXPERT,
            ai_lang,
            full_diff
        );

        let mut cmd = Command::new("kilo");
        cmd.args(["run", "--pure", "--auto"]);

        let model_to_use = if !self.current_kilo_model.is_empty() {
            self.current_kilo_model.clone()
        } else if let Ok(env_model) = std::env::var("KILO_MODEL") {
            env_model
        } else {
            String::new()
        };

        if !model_to_use.trim().is_empty() {
            cmd.args(["--model", &model_to_use]);
        }

        cmd.arg(prompt);

        let output = match cmd.output() {
            Ok(o) => o,
            Err(e) => {
                return Err(if self.current_lang == "vi" {
                    format!("Không tìm thấy lệnh 'kilo'. Hãy cài @kilocode/cli (npm i -g @kilocode/cli) hoặc đảm bảo 'kilo' có trong PATH. Lỗi: {}", e)
                } else {
                    format!("'kilo' command not found. Please install @kilocode/cli (npm i -g @kilocode/cli) and ensure it is in PATH. Error: {}", e)
                });
            }
        };

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(if self.current_lang == "vi" {
                format!("kilo run thất bại: {}", stderr)
            } else {
                format!("kilo run failed: {}", stderr)
            });
        }

        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();

        if stdout.is_empty() {
            return Err(if self.current_lang == "vi" {
                "kilo trả về kết quả rỗng.".to_string()
            } else {
                "kilo returned empty output.".to_string()
            });
        }

        Ok(stdout)
    }

    pub fn fetch_kilo_models(&mut self) {
        self.kilo_models.clear();
        if let Ok(output) = Command::new("kilo").arg("models").output() {
            let text = String::from_utf8_lossy(&output.stdout);
            for line in text.lines() {
                let line = line.trim();
                if line.is_empty() { continue; }
                if line.starts_with("kilo/") || line.contains('/') {
                    self.kilo_models.push(line.to_string());
                }
            }
        }
        if self.selected_kilo_model_index >= self.kilo_models.len() {
            self.selected_kilo_model_index = 0;
        }
    }

    pub fn fetch_amend_msg(&mut self) {
        self.amend_message = crate::git::commit::get_last_commit_subject();
        self.amend_step = AmendStep::Edit;
    }

    pub fn fetch_commit_diff(&mut self, hash: &str) {
        self.commit_diff_content = crate::git::commit::get_commit_diff(hash);
        self.commit_diff_scroll = 0;
    }

    pub fn load_workspace_history(&mut self) {
        self.workspace_history.clear();
        if let Ok(output) = std::process::Command::new("git")
            .args(["config", "--global", "--get", "git-ai.workspace-history"])
            .output()
        {
            let text = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !text.is_empty() {
                for entry in text.split('|') {
                    let trimmed = entry.trim().to_string();
                    if !trimmed.is_empty() {
                        self.workspace_history.push(trimmed);
                    }
                }
            }
        }
    }

    pub fn save_workspace_history(&self) {
        let value = self.workspace_history.join("|");
        let _ = std::process::Command::new("git")
            .args(["config", "--global", "git-ai.workspace-history", &value])
            .output();
    }

    pub fn add_to_workspace_history(&mut self, path: &str) {
        // Remove duplicate if exists
        self.workspace_history.retain(|p| p != path);
        // Insert at top (MRU)
        self.workspace_history.insert(0, path.to_string());
        // Keep max 10
        self.workspace_history.truncate(10);
        self.save_workspace_history();
    }

    pub fn remove_from_workspace_history(&mut self, index: usize) {
        if index < self.workspace_history.len() {
            self.workspace_history.remove(index);
            self.save_workspace_history();
            if self.selected_workspace_index >= self.workspace_history.len() && !self.workspace_history.is_empty() {
                self.selected_workspace_index = self.workspace_history.len() - 1;
            }
        }
    }

    pub fn theme(&self) -> AppTheme {
        if self.is_light_theme {
            AppTheme {
                fg: Color::Rgb(40, 42, 54),        // Dark charcoal
                border: Color::Rgb(140, 140, 140),  // Mid gray
                purple: Color::Rgb(109, 40, 217),  // Deep purple
                green: Color::Rgb(21, 128, 61),     // Deep green
                red: Color::Rgb(185, 28, 28),       // Deep red
                yellow: Color::Rgb(161, 98, 7),     // Amber/gold
                cyan: Color::Rgb(3, 105, 161),      // Deep sky blue
                orange: Color::Rgb(194, 65, 12),    // Rust orange
                select_bg: Color::Rgb(220, 224, 232), // Light slate gray background
                select_fg: Color::Rgb(17, 24, 39),   // Very dark gray/black text
                bg: Color::Rgb(248, 249, 250),      // Soft off-white
            }
        } else {
            AppTheme {
                fg: Color::Rgb(248, 248, 242),      // Dracula White
                border: Color::Rgb(98, 114, 164),   // Dracula Gray/Comment
                purple: Color::Rgb(189, 147, 249),  // Dracula Purple
                green: Color::Rgb(80, 250, 123),    // Dracula Green
                red: Color::Rgb(255, 85, 85),       // Dracula Red
                yellow: Color::Rgb(241, 250, 140),   // Dracula Yellow
                cyan: Color::Rgb(139, 233, 253),    // Dracula Cyan
                orange: Color::Rgb(255, 184, 108),  // Dracula Orange
                select_bg: Color::Rgb(68, 71, 90),  // Dracula Selection Background
                select_fg: Color::Rgb(248, 248, 242), // Dracula White
                bg: Color::Rgb(40, 42, 54),         // Dracula Background
            }
        }
    }
}
