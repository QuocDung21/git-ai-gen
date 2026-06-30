use anyhow::Result;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

const MARKERS: &[&str] = &[
    "# ULTIMATE GIT-AI WORKFLOW",
    "alias git-copydiff",
    "alias git-go",
    "alias git-clear-trash",
    "alias git-ai-uninstall",
    "alias git-ai=",
    "function git-copydiff",
    "function git-go",
    "function git-clear-trash",
    "function git-ai-uninstall",
    "function git-ai",
];

#[cfg(target_family = "unix")]
pub fn get_active_unix_profile() -> PathBuf {
    use std::env;
    let home = env::var("HOME").unwrap_or_else(|_| "~".to_string());
    let shell = env::var("SHELL").unwrap_or_default().to_lowercase();
    if shell.contains("zsh") {
        PathBuf::from(format!("{}/.zshrc", home))
    } else if shell.contains("bash") {
        let bashrc = PathBuf::from(format!("{}/.bashrc", home));
        if bashrc.exists() {
            bashrc
        } else {
            PathBuf::from(format!("{}/.bash_profile", home))
        }
    } else if shell.contains("fish") {
        PathBuf::from(format!("{}/.config/fish/config.fish", home))
    } else {
        let zsh_path = PathBuf::from(format!("{}/.zshrc", home));
        if zsh_path.exists() {
            zsh_path
        } else {
            PathBuf::from(format!("{}/.bashrc", home))
        }
    }
}

#[cfg(target_os = "windows")]
pub fn get_windows_profile() -> Result<PathBuf> {
    use std::process::Command;
    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command", "Write-Output $PROFILE"])
        .output()?;
    let path_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
    Ok(PathBuf::from(path_str))
}

pub fn clean_profile_file(path: &PathBuf) -> Result<bool> {
    if !path.exists() {
        return Ok(false);
    }

    let content = fs::read_to_string(path)?;
    let lines: Vec<&str> = content.lines().collect();
    let mut filtered_lines = Vec::new();
    let mut changed = false;

    for line in lines {
        if MARKERS.iter().any(|&m| line.contains(m)) {
            changed = true;
        } else {
            filtered_lines.push(line);
        }
    }

    if changed {
        let new_content = filtered_lines.join("\n") + "\n";
        fs::write(path, new_content)?;
        Ok(true)
    } else {
        Ok(false)
    }
}

pub fn append_to_file(path: &PathBuf, content: &str) -> Result<()> {
    let mut file = OpenOptions::new().append(true).create(true).open(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
