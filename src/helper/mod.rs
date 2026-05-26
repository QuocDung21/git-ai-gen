use crate::locales::Locales;
use crate::models::LanguageStat;
use std::path::Path;
use std::process::Command;

mod project_languages;

pub struct Helper;

impl Helper {
    pub fn get_os_theme() -> dark_light::Mode {
        match dark_light::detect() {
            Ok(mode) => mode,
            Err(_) => dark_light::Mode::Unspecified,
        }
    }

    pub fn get_ai_language() -> String {
        if let Ok(output) = Command::new("git")
            .args(["config", "--global", "--get", "git-ai.lang"])
            .output()
        {
            let stdout = String::from_utf8_lossy(&output.stdout)
                .trim()
                .to_lowercase();
            if stdout == "vi" || stdout == "en" {
                return stdout;
            }
        }

        let locale = sys_locale::get_locale().unwrap_or_else(|| String::from("en-US"));
        if locale.starts_with("vi") {
            "vi".to_string()
        } else {
            "en".to_string()
        }
    }

    pub fn get_ai_language_name() -> String {
        match Helper::get_ai_language().as_str() {
            "vi" => "Vietnamese".to_string(),
            _ => "English".to_string(),
        }
    }

    pub fn get_locales() -> Locales {
        let lang = Helper::get_ai_language();
        Locales::new(&lang)
    }

    pub fn get_git_chill_dir() -> std::path::PathBuf {
        let home = std::env::var("HOME")
            .or_else(|_| std::env::var("USERPROFILE"))
            .unwrap_or_else(|_| ".".to_string());
        std::path::PathBuf::from(home).join(".git-chill")
    }

    pub fn load_history_file(filename: &str) -> Vec<String> {
        let chill_dir = Self::get_git_chill_dir();
        let file_path = chill_dir.join(filename);

        if file_path.exists() {
            if let Ok(content) = std::fs::read_to_string(&file_path) {
                return content
                    .lines()
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
            }
        }

        let git_config_key = match filename {
            "workspace_history.txt" => "git-ai.workspace-history",
            "github_history.txt" => "git-ai.github-history",
            _ => "",
        };

        let mut history = Vec::new();
        if !git_config_key.is_empty() {
            if let Ok(output) = Command::new("git")
                .args(["config", "--global", "--get", git_config_key])
                .output()
            {
                if output.status.success() {
                    let text = String::from_utf8_lossy(&output.stdout).trim().to_string();
                    if !text.is_empty() {
                        for entry in text.split('|') {
                            let trimmed = entry.trim().to_string();
                            if !trimmed.is_empty() {
                                history.push(trimmed);
                            }
                        }
                    }
                }
            }
        }

        if !history.is_empty() {
            let _ = Self::save_history_file(filename, &history);
            let _ = Command::new("git")
                .args(["config", "--global", "--unset", git_config_key])
                .output();
        }

        history
    }

    pub fn save_history_file(filename: &str, history: &[String]) -> Result<(), std::io::Error> {
        let chill_dir = Self::get_git_chill_dir();
        if !chill_dir.exists() {
            std::fs::create_dir_all(&chill_dir)?;
        }
        let file_path = chill_dir.join(filename);
        let content = history.join("\n");
        std::fs::write(file_path, content)?;
        Ok(())
    }

    pub fn detect_project_languages<P: AsRef<Path>>(dir: P) -> Vec<LanguageStat> {
        project_languages::detect_project_languages(dir)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_git_chill_history() {
        let temp_dir = tempfile::tempdir().unwrap();
        let old_home = std::env::var("HOME").ok();
        std::env::set_var("HOME", temp_dir.path());

        let filename = "test_history.txt";
        let test_data = vec!["/path/one".to_string(), "/path/two".to_string()];

        let save_res = Helper::save_history_file(filename, &test_data);
        assert!(save_res.is_ok());

        let loaded = Helper::load_history_file(filename);
        assert_eq!(loaded, test_data);

        if let Some(h) = old_home {
            std::env::set_var("HOME", h);
        } else {
            std::env::remove_var("HOME");
        }
    }
}
