use super::App;

impl App {
    pub fn load_workspace_history(&mut self) {
        self.workspace_history = crate::helper::Helper::load_history_file("workspace_history.txt");
    }

    pub fn save_workspace_history(&self) {
        let _ = crate::helper::Helper::save_history_file(
            "workspace_history.txt",
            &self.workspace_history,
        );
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
        self.github_history = crate::helper::Helper::load_history_file("github_history.txt");
    }

    pub fn save_github_history(&self) {
        let _ =
            crate::helper::Helper::save_history_file("github_history.txt", &self.github_history);
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
