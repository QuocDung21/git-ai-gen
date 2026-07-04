use std::process::Command;

use super::App;
use crate::models::{AmendStep, CommitLogEntry, FeatureGroup, StashEntry, StashStep};

impl App {
    pub fn fetch_commit_logs(&mut self) {
        self.commit_logs.clear();
        if let Ok(output) = Command::new("git")
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
        if let Ok(output) = Command::new("git")
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
        if let Some(idx) = self
            .branches
            .iter()
            .position(|b| b.name == self.current_branch && !b.is_remote)
        {
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
        let ai_lang = crate::helper::Helper::get_ai_language_name();
        self.prompt_text = format!("{}{}.", crate::constant::Constant::PROMPT_EXPERT, ai_lang);
    }

    pub fn fetch_amend_msg(&mut self) {
        self.amend_message = crate::git::commit::get_last_commit_subject();
        self.amend_step = AmendStep::Edit;
    }

    pub fn fetch_commit_diff(&mut self, hash: &str) {
        self.commit_diff_content = crate::git::commit::get_commit_diff(hash);
        self.commit_diff_scroll = 0;
    }

    pub fn compute_feature_groups(&mut self) {
        self.feature_groups.clear();
        self.selected_feature_index = 0;

        if self.files.is_empty() {
            return;
        }

        use std::collections::HashMap;
        let mut groups: HashMap<String, Vec<String>> = HashMap::new();

        for file in &self.files {
            let path = &file.path;
            let mut feature = if path.contains('/') {
                let parts: Vec<&str> = path.split('/').collect();
                if parts.len() > 1
                    && (parts[0] == "src"
                        || parts[0] == "crates"
                        || parts[0] == "packages"
                        || parts[0] == "libs"
                        || parts[0] == "apps")
                {
                    parts[1].to_string()
                } else {
                    parts[0].to_string()
                }
            } else {
                "root".to_string()
            };
            if feature.is_empty() {
                feature = "root".to_string();
            }
            groups.entry(feature).or_default().push(path.clone());
        }

        for (name, files) in groups {
            let count = files.len();
            self.feature_groups.push(FeatureGroup {
                name,
                files,
                file_count: count,
            });
        }

        self.feature_groups
            .sort_by_key(|group| std::cmp::Reverse(group.file_count));
    }
}
