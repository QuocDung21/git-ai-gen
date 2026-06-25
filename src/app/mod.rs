use rust_i18n::t;
use std::env;
use std::process::Command;

pub mod events;
pub mod fetch;
pub mod github;
pub mod history;

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
    pub selected_file_diff_lines: Vec<DiffViewLine>,
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
    pub last_staged_diff: String,
    pub ai_temp: AiTemp,
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
    pub workspace_path_input: String,
    pub is_light_theme: bool,
    pub focus_diff: bool,
    pub workspace_history: Vec<String>,
    pub language_stats: Vec<crate::models::LanguageStat>,
    pub language_analysis_pending: bool,
    pub locales: crate::locales::Locales,
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
    pub show_splash: bool,
    pub splash_enabled: bool,
    pub editor: String,
    pub selected_editor_index: usize,
    pub diff_captured_unstaged: bool,
    pub diff_copy_failed: bool,
    pub diff_snapshot_scroll: usize,
}

#[cfg(target_os = "windows")]
pub const DEFAULT_OPEN_CMD: &str = "explorer";
#[cfg(target_os = "macos")]
pub const DEFAULT_OPEN_CMD: &str = "open";
#[cfg(not(any(target_os = "windows", target_os = "macos")))]
pub const DEFAULT_OPEN_CMD: &str = "xdg-open";

impl App {
    pub fn tr<'a>(&self, vi: &'a str, en: &'a str) -> &'a str {
        if self.current_lang == "vi" {
            vi
        } else {
            en
        }
    }

    pub fn refresh_locales(&mut self) {
        self.locales = crate::locales::Locales::new(&self.current_lang);
    }

    pub fn new() -> Self {
        let current_dir = env::current_dir()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|_| "Unknown".to_string());

        let current_lang = crate::helper::Helper::get_ai_language();
        let init_msg = t!("init_ready").to_string();

        let theme_id = "midnight".to_string();
        let is_light_theme = false;

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

        let splash_enabled = {
            if let Ok(output) = Command::new("git")
                .args(["config", "--global", "--get", "git-ai.splash"])
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

        let editor = {
            if let Ok(output) = Command::new("git")
                .args(["config", "--global", "--get", "git-ai.editor"])
                .output()
            {
                let text = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if text.is_empty() {
                    "code".to_string()
                } else {
                    text
                }
            } else {
                "code".to_string()
            }
        };

        let selected_editor_index = match editor.as_str() {
            "code" => 0,
            "cursor" => 1,
            "zed" => 2,
            "subl" => 3,
            _ => 4,
        };

        let mut app = App {
            status_message: init_msg.to_string(),
            git_status_lines: Vec::new(),
            current_lang: current_lang.clone(),
            current_dir,
            files: Vec::new(),
            selected_index: 0,
            selected_file_diff: String::new(),
            selected_file_diff_lines: Vec::new(),
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
            last_staged_diff: String::new(),
            ai_temp: AiTemp::None,
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
            workspace_path_input: String::new(),
            is_light_theme,
            focus_diff: false,
            workspace_history: Vec::new(),
            language_stats: Vec::new(),
            language_analysis_pending: false,
            locales: crate::locales::Locales::new(&current_lang),
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
            show_splash: splash_enabled,
            splash_enabled,
            editor,
            selected_editor_index,
            diff_captured_unstaged: false,
            diff_copy_failed: false,
            diff_snapshot_scroll: 0,
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
                self.git_status_lines.push(t!("status_clean").to_string());
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
            self.git_status_lines.push(t!("status_fail").to_string());
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
            self.selected_file_diff_lines.clear();
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
            self.selected_file_diff_lines.clear();
            return;
        }

        let file = &self.files[self.selected_index];
        let is_untracked = file.status.starts_with("??") || file.status.contains("??");

        let output = if is_untracked {
            if let Ok(content) = std::fs::read_to_string(&file.path) {
                let lines: Vec<&str> = content.lines().take(500).collect();
                let heading = t!("untracked_file_heading").to_string();
                heading + &lines.join("\n")
            } else {
                t!("cannot_read_file").to_string()
            }
        } else {
            let mut diff_output = crate::git::status::get_diff_head(&file.path);

            if diff_output.is_none() {
                diff_output = crate::git::status::get_diff_unstaged(&file.path);
            }

            if diff_output.is_none() {
                diff_output = crate::git::status::get_diff_staged(&file.path);
            }

            diff_output.unwrap_or_else(|| t!("no_changes_compared_to_last_commit").to_string())
        };
        self.selected_file_diff = output;
        self.rebuild_selected_diff_lines();
    }

    fn rebuild_selected_diff_lines(&mut self) {
        self.selected_file_diff_lines = self
            .selected_file_diff
            .lines()
            .map(|line| {
                let text = line.replace('\t', "    ");
                let kind = if text.starts_with('+') && !text.starts_with("+++") {
                    DiffLineKind::Added
                } else if text.starts_with('-') && !text.starts_with("---") {
                    DiffLineKind::Removed
                } else if text.starts_with("@@") {
                    DiffLineKind::Hunk
                } else if text.starts_with("diff --git") || text.starts_with("index") {
                    DiffLineKind::Header
                } else {
                    DiffLineKind::Normal
                };
                DiffViewLine { text, kind }
            })
            .collect();
    }

    pub fn theme(&self) -> AppTheme {
        crate::theme::get_theme(&self.theme_id)
    }
}
