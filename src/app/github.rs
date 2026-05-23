use super::App;
use crate::models::GithubTreeEntry;

impl App {
    pub fn visit_repo_dir(&mut self) -> std::io::Result<()> {
        self.github_tree_entries.clear();
        self.github_expanded_dirs.clear();
        self.github_selected_paths.clear();
        let temp_dir = if let Some(ref dir) = self.github_temp_dir {
            dir.path()
        } else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Không tìm thấy thư mục tạm",
            ));
        };

        let output = std::process::Command::new("git")
            .args(["ls-tree", "-r", "-t", "HEAD"])
            .current_dir(&temp_dir)
            .output()?;

        if !output.status.success() {
            let err_msg = String::from_utf8_lossy(&output.stderr).to_string();
            return Err(std::io::Error::new(std::io::ErrorKind::Other, err_msg));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut entries = Vec::new();

        #[derive(Clone)]
        struct ParsedEntry {
            path: String,
            name: String,
            is_dir: bool,
        }

        for line in stdout.lines() {
            let parts: Vec<&str> = line.split('\t').collect();
            if parts.len() < 2 {
                continue;
            }
            let meta = parts[0];
            let path_str = parts[1].to_string();
            let is_dir = meta.contains(" tree ");
            let name = if let Some(idx) = path_str.rfind('/') {
                path_str[idx + 1..].to_string()
            } else {
                path_str.clone()
            };
            entries.push(ParsedEntry {
                path: path_str,
                name,
                is_dir,
            });
        }

        let mut parent_to_children: std::collections::HashMap<String, Vec<ParsedEntry>> =
            std::collections::HashMap::new();
        for entry in entries {
            let parent = if let Some(idx) = entry.path.rfind('/') {
                entry.path[..idx].to_string()
            } else {
                "".to_string()
            };
            parent_to_children.entry(parent).or_default().push(entry);
        }

        fn build_tree(
            parent: &str,
            depth: usize,
            parent_to_children: &std::collections::HashMap<String, Vec<ParsedEntry>>,
            result: &mut Vec<GithubTreeEntry>,
        ) {
            if let Some(children) = parent_to_children.get(parent) {
                let mut sorted_children = children.clone();
                sorted_children.sort_by(|a, b| {
                    if a.is_dir != b.is_dir {
                        b.is_dir.cmp(&a.is_dir)
                    } else {
                        a.name.cmp(&b.name)
                    }
                });
                for child in sorted_children {
                    result.push(GithubTreeEntry {
                        path: child.path.clone(),
                        name: child.name.clone(),
                        is_dir: child.is_dir,
                        depth,
                    });
                    if child.is_dir {
                        build_tree(&child.path, depth + 1, parent_to_children, result);
                    }
                }
            }
        }

        let mut sorted_entries = Vec::new();
        build_tree("", 0, &parent_to_children, &mut sorted_entries);
        self.github_tree_entries = sorted_entries;
        Ok(())
    }

    pub fn get_visible_github_tree_entries(&self) -> Vec<&GithubTreeEntry> {
        self.github_tree_entries
            .iter()
            .filter(|entry| {
                let parts: Vec<&str> = entry.path.split('/').collect();
                if parts.len() <= 1 {
                    return true;
                }
                let mut current = String::new();
                for i in 0..(parts.len() - 1) {
                    if i > 0 {
                        current.push('/');
                    }
                    current.push_str(parts[i]);
                    if !self.github_expanded_dirs.contains(&current) {
                        return false;
                    }
                }
                true
            })
            .collect()
    }

    pub fn toggle_github_tree_selection(&mut self, index: usize) {
        let visible = self.get_visible_github_tree_entries();
        if index >= visible.len() {
            return;
        }
        let entry = visible[index].clone();
        let is_selected = self.github_selected_paths.contains(&entry.path);
        if entry.is_dir {
            let prefix = format!("{}/", entry.path);
            if is_selected {
                self.github_selected_paths.remove(&entry.path);
                self.github_selected_paths
                    .retain(|p| !p.starts_with(&prefix));
            } else {
                self.github_selected_paths.insert(entry.path.clone());
                for e in &self.github_tree_entries {
                    if e.path.starts_with(&prefix) {
                        self.github_selected_paths.insert(e.path.clone());
                    }
                }
            }
        } else if is_selected {
            self.github_selected_paths.remove(&entry.path);
        } else {
            self.github_selected_paths.insert(entry.path.clone());
        }
    }

    pub fn copy_github_download_item(&self) -> std::io::Result<()> {
        let mut items_to_download = Vec::new();
        if self.github_selected_paths.is_empty() {
            let visible = self.get_visible_github_tree_entries();
            if !visible.is_empty() && self.selected_github_tree_index < visible.len() {
                items_to_download.push((*visible[self.selected_github_tree_index]).clone());
            }
        } else {
            for entry in &self.github_tree_entries {
                if self.github_selected_paths.contains(&entry.path) {
                    let mut has_selected_ancestor = false;
                    let parts: Vec<&str> = entry.path.split('/').collect();
                    let mut current = String::new();
                    for i in 0..(parts.len().saturating_sub(1)) {
                        if i > 0 {
                            current.push('/');
                        }
                        current.push_str(parts[i]);
                        if self.github_selected_paths.contains(&current) {
                            has_selected_ancestor = true;
                            break;
                        }
                    }
                    if !has_selected_ancestor {
                        items_to_download.push(entry.clone());
                    }
                }
            }
        }

        if items_to_download.is_empty() {
            return Ok(());
        }

        let temp_dir = if let Some(ref dir) = self.github_temp_dir {
            dir.path()
        } else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Không tìm thấy thư mục tạm",
            ));
        };
        let dst_dir = std::path::Path::new(&self.github_download_target_path);
        std::fs::create_dir_all(dst_dir)?;

        for entry in items_to_download {
            let output = std::process::Command::new("git")
                .args(["checkout", "HEAD", &entry.path])
                .current_dir(&temp_dir)
                .output()?;

            if !output.status.success() {
                let err_msg = String::from_utf8_lossy(&output.stderr).to_string();
                return Err(std::io::Error::new(std::io::ErrorKind::Other, err_msg));
            }

            let src_path = temp_dir.join(&entry.path);
            let dst_path = dst_dir.join(&entry.path);
            if entry.is_dir {
                self.copy_dir_rec(&src_path, &dst_path)?;
            } else {
                if let Some(parent) = dst_path.parent() {
                    std::fs::create_dir_all(parent)?;
                }
                std::fs::copy(&src_path, &dst_path)?;
            }
        }

        Ok(())
    }

    fn copy_dir_rec(&self, src: &std::path::Path, dst: &std::path::Path) -> std::io::Result<()> {
        std::fs::create_dir_all(dst)?;
        for entry in std::fs::read_dir(src)? {
            let entry = entry?;
            let file_type = entry.file_type()?;
            let new_dst = dst.join(entry.file_name());
            if file_type.is_dir() {
                self.copy_dir_rec(&entry.path(), &new_dst)?;
            } else {
                std::fs::copy(entry.path(), new_dst)?;
            }
        }
        Ok(())
    }

    pub fn fetch_github_branches(&mut self) -> std::io::Result<()> {
        let temp_dir = if let Some(ref dir) = self.github_temp_dir {
            dir.path()
        } else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Không tìm thấy thư mục tạm",
            ));
        };

        let output = std::process::Command::new("git")
            .args(["ls-remote", "--heads", "origin"])
            .current_dir(temp_dir)
            .output()?;

        if !output.status.success() {
            let err_msg = String::from_utf8_lossy(&output.stderr).to_string();
            return Err(std::io::Error::new(std::io::ErrorKind::Other, err_msg));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut branches = Vec::new();
        for line in stdout.lines() {
            let parts: Vec<&str> = line.split('\t').collect();
            if parts.len() >= 2 {
                let ref_name = parts[1];
                if let Some(branch_name) = ref_name.strip_prefix("refs/heads/") {
                    branches.push(branch_name.to_string());
                }
            }
        }

        branches.sort();

        self.github_branches = branches;
        Ok(())
    }
}
