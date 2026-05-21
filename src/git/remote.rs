use std::collections::HashSet;
use std::process::Command;

use crate::app::models::RemoteEntry;

pub fn get_remote_url() -> String {
    if let Ok(out) = Command::new("git")
        .args(["remote", "get-url", "origin"])
        .output()
    {
        String::from_utf8_lossy(&out.stdout).trim().to_string()
    } else {
        "(no remote)".to_string()
    }
}

pub fn get_remote_name(branch: &str) -> String {
    let tracking_key = format!("branch.{}.remote", branch);
    if let Ok(out) = Command::new("git")
        .args(["config", "--get", &tracking_key])
        .output()
    {
        String::from_utf8_lossy(&out.stdout).trim().to_string()
    } else {
        "origin".to_string()
    }
}

pub fn get_ahead_behind(branch: &str, remote_tracking: &str) -> (i32, i32) {
    let rev_range = format!("{}...{}", branch, remote_tracking);
    if let Ok(out) = Command::new("git")
        .args(["rev-list", "--left-right", "--count", &rev_range])
        .output()
    {
        let text = String::from_utf8_lossy(&out.stdout);
        let nums: Vec<&str> = text.trim().split_whitespace().collect();
        if nums.len() == 2 {
            let ahead = nums[0].parse().unwrap_or(0);
            let behind = nums[1].parse().unwrap_or(0);
            return (ahead, behind);
        }
    }
    (0, 0)
}

pub fn git_push() -> Result<String, std::io::Error> {
    let output = Command::new("git").arg("push").output()?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        let err = String::from_utf8_lossy(&output.stderr).trim().to_string();
        Err(std::io::Error::new(std::io::ErrorKind::Other, err))
    }
}

pub fn git_fetch() -> Result<String, std::io::Error> {
    let output = Command::new("git").arg("fetch").output()?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        let err = String::from_utf8_lossy(&output.stderr).trim().to_string();
        Err(std::io::Error::new(std::io::ErrorKind::Other, err))
    }
}

pub fn git_pull() -> Result<String, std::io::Error> {
    let output = Command::new("git").arg("pull").output()?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        let err = String::from_utf8_lossy(&output.stderr).trim().to_string();
        Err(std::io::Error::new(std::io::ErrorKind::Other, err))
    }
}

pub fn get_remotes() -> Vec<RemoteEntry> {
    let mut remotes = Vec::new();
    if let Ok(output) = Command::new("git")
        .args(["remote", "-v"])
        .output()
    {
        let text = String::from_utf8_lossy(&output.stdout);
        let mut seen = HashSet::new();
        for line in text.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                let name = parts[0].to_string();
                let url = parts[1].to_string();
                if !seen.contains(&name) {
                    seen.insert(name.clone());
                    remotes.push(RemoteEntry { name, url });
                }
            }
        }
    }
    remotes
}
