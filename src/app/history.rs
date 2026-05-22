use super::App;

impl App {
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
        self.workspace_history.retain(|p| p != path);
        self.workspace_history.insert(0, path.to_string());
        self.workspace_history.truncate(10);
        self.save_workspace_history();
    }

    pub fn remove_from_workspace_history(&mut self, index: usize) {
        if index < self.workspace_history.len() {
            self.workspace_history.remove(index);
            self.save_workspace_history();
            if self.selected_workspace_index >= self.workspace_history.len()
                && !self.workspace_history.is_empty()
            {
                self.selected_workspace_index = self.workspace_history.len() - 1;
            }
        }
    }

    pub fn load_github_history(&mut self) {
        self.github_history.clear();
        if let Ok(output) = std::process::Command::new("git")
            .args(["config", "--global", "--get", "git-ai.github-history"])
            .output()
        {
            let text = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !text.is_empty() {
                for entry in text.split('|') {
                    let trimmed = entry.trim().to_string();
                    if !trimmed.is_empty() {
                        self.github_history.push(trimmed);
                    }
                }
            }
        }
    }

    pub fn save_github_history(&self) {
        let value = self.github_history.join("|");
        let _ = std::process::Command::new("git")
            .args(["config", "--global", "git-ai.github-history", &value])
            .output();
    }

    pub fn add_to_github_history(&mut self, url: &str) {
        self.github_history.retain(|u| u != url);
        self.github_history.insert(0, url.to_string());
        self.github_history.truncate(10);
        self.save_github_history();
    }

    pub fn remove_from_github_history(&mut self, index: usize) {
        if index < self.github_history.len() {
            self.github_history.remove(index);
            self.save_github_history();
            if let Some(sel) = self.selected_github_history_index {
                if sel >= self.github_history.len() && !self.github_history.is_empty() {
                    self.selected_github_history_index = Some(self.github_history.len() - 1);
                } else if self.github_history.is_empty() {
                    self.selected_github_history_index = None;
                }
            }
        }
    }
}
