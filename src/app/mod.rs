use std::env;
use std::process::Command;

pub mod events;
pub mod fetch;
pub mod github;
pub mod history;
pub mod kilo;

pub use crate::models::*;

use crate::theme::AppTheme;

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
    pub current_branch: String,
    pub staged_count: usize,
    pub unstaged_count: usize,
    pub untracked_count: usize,
    pub commit_logs: Vec<CommitLogEntry>,
    pub selected_log_index: usize,
    pub branches: Vec<BranchEntry>,
    pub selected_branch_index: usize,
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
    pub manual_commit_message: String,
    pub selected_git_action: usize,
    pub feature_groups: Vec<FeatureGroup>,
    pub selected_feature_index: usize,
    pub commit_message_preview: String,
    pub go_step: GoStep,
    pub commit_input_mode: bool,
    pub commit_input_text: String,
    pub stash_entries: Vec<StashEntry>,
    pub selected_stash_index: usize,
    pub stash_step: StashStep,
    pub remote_url: String,
    pub remote_tracking: String,
    pub ahead_count: i32,
    pub behind_count: i32,
    pub remotes: Vec<RemoteEntry>,
    pub amend_step: AmendStep,
    pub amend_message: String,
    pub commit_diff_content: String,
    pub commit_diff_scroll: usize,
    pub has_conflicts: bool,
    pub conflict_count: usize,
    pub new_branch_name: String,
    pub is_light_theme: bool,
    pub selected_theme_index: usize,
    pub focus_diff: bool,
    pub workspace_history: Vec<String>,
    pub selected_workspace_index: usize,
    pub prompt_text: String,
    pub github_download_url: String,
    pub github_cloning: bool,
    pub github_tree_entries: Vec<GithubTreeEntry>,
    pub selected_github_tree_index: usize,
    pub github_download_target_path: String,
    pub github_cloning_error: Option<String>,
    pub github_expanded_dirs: std::collections::HashSet<String>,
    pub github_selected_paths: std::collections::HashSet<String>,
    pub github_history: Vec<String>,
    pub selected_github_history_index: Option<usize>,
    pub github_download_url_temp: String,
    pub github_temp_dir: Option<tempfile::TempDir>,
    pub github_branches: Vec<String>,
    pub selected_github_branch_index: usize,
    pub current_github_branch: String,
    pub github_quickview_scroll: usize,
    pub github_quickview_search: String,
    pub github_quickview_searching: bool,
    pub auto_push: bool,
    pub auto_stage_all: bool,
    pub kilo_ai_enabled: bool,
    pub theme_id: String,
    pub selected_setting_index: usize,
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

        let theme_id = {
            if let Ok(output) = Command::new("git")
                .args(["config", "--global", "--get", "git-ai.theme"])
                .output()
            {
                let text = String::from_utf8_lossy(&output.stdout)
                    .trim()
                    .to_lowercase();
                if text.is_empty() {
                    if crate::helper::Helper::get_os_theme() == dark_light::Mode::Light {
                        "light".to_string()
                    } else {
                        "dark".to_string()
                    }
                } else {
                    text
                }
            } else {
                if crate::helper::Helper::get_os_theme() == dark_light::Mode::Light {
                    "light".to_string()
                } else {
                    "dark".to_string()
                }
            }
        };

        let is_light_theme = theme_id == "light";

        let selected_theme_index = match theme_id.as_str() {
            "dark" => 0,
            "light" => 1,
            "nord" => 2,
            "gruvbox" => 3,
            _ => 0,
        };

        let auto_push = {
            if let Ok(output) = Command::new("git")
                .args(["config", "--global", "--get", "git-ai.auto-push"])
                .output()
            {
                let text = String::from_utf8_lossy(&output.stdout)
                    .trim()
                    .to_lowercase();
                text != "false"
            } else {
                true
            }
        };

        let auto_stage_all = {
            if let Ok(output) = Command::new("git")
                .args(["config", "--global", "--get", "git-ai.auto-stage-all"])
                .output()
            {
                let text = String::from_utf8_lossy(&output.stdout)
                    .trim()
                    .to_lowercase();
                text == "true"
            } else {
                false
            }
        };

        let kilo_ai_enabled = {
            if let Ok(output) = Command::new("git")
                .args(["config", "--global", "--get", "git-ai.kilo-ai"])
                .output()
            {
                let text = String::from_utf8_lossy(&output.stdout)
                    .trim()
                    .to_lowercase();
                text != "false"
            } else {
                true
            }
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
            feature_groups: Vec::new(),
            selected_feature_index: 0,
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
            github_download_url: String::new(),
            github_cloning: false,
            github_tree_entries: Vec::new(),
            selected_github_tree_index: 0,
            github_download_target_path: String::new(),
            github_cloning_error: None,
            github_expanded_dirs: std::collections::HashSet::new(),
            github_selected_paths: std::collections::HashSet::new(),
            github_history: Vec::new(),
            selected_github_history_index: None,
            github_download_url_temp: String::new(),
            github_temp_dir: None,
            github_branches: Vec::new(),
            selected_github_branch_index: 0,
            current_github_branch: String::new(),
            github_quickview_scroll: 0,
            github_quickview_search: String::new(),
            github_quickview_searching: false,
            auto_push,
            auto_stage_all,
            kilo_ai_enabled,
            theme_id,
            selected_setting_index: 0,
        };
        app.load_workspace_history();
        app.load_github_history();
        app.add_to_workspace_history(&app.current_dir.clone());
        app.refresh_git_status();
        app
    }

    pub fn auto_stage_all_if_enabled(&mut self) {
        if self.auto_stage_all {
            let _ = crate::git::status::stage_all();
            self.refresh_git_status();
        }
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

        self.current_branch = crate::git::status::get_current_branch();

        self.staged_count = 0;
        self.unstaged_count = 0;
        self.untracked_count = 0;
        self.conflict_count = 0;

        for file in &self.files {
            let first_char = file.status.chars().next().unwrap_or(' ');
            let second_char = file.status.chars().nth(1).unwrap_or(' ');
            let status_trimmed = file.status.trim();

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
                } else if self.selected_index >= self.files.len() {
                    self.selected_index = self.files.len() - 1;
                }
            } else if self.selected_index >= self.files.len() {
                self.selected_index = self.files.len() - 1;
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

    pub fn theme(&self) -> AppTheme {
        crate::theme::get_theme(&self.theme_id)
    }
}
