use crate::app::models::BranchEntry;
use std::process::Command;

pub fn get_branches() -> Vec<BranchEntry> {
    let mut branches = Vec::new();
    if let Ok(output) = Command::new("git")
        .args(["branch", "-a", "--format=%(refname)"])
        .output()
    {
        let text = String::from_utf8_lossy(&output.stdout);
        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            if line.ends_with("/HEAD") {
                continue;
            }
            if let Some(local_name) = line.strip_prefix("refs/heads/") {
                branches.push(BranchEntry {
                    name: local_name.to_string(),
                    is_remote: false,
                });
            } else if let Some(remote_name) = line.strip_prefix("refs/remotes/") {
                branches.push(BranchEntry {
                    name: remote_name.to_string(),
                    is_remote: true,
                });
            }
        }
    }
    branches
}

pub fn checkout_branch(branch: &str) -> Result<String, std::io::Error> {
    let output = Command::new("git").args(["checkout", branch]).output()?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        let err = String::from_utf8_lossy(&output.stderr).trim().to_string();
        Err(std::io::Error::new(std::io::ErrorKind::Other, err))
    }
}

pub fn git_merge(branch: &str) -> Result<String, std::io::Error> {
    let output = Command::new("git").args(["merge", branch]).output()?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        let err = String::from_utf8_lossy(&output.stderr).trim().to_string();
        Err(std::io::Error::new(std::io::ErrorKind::Other, err))
    }
}

pub fn create_and_checkout_branch(branch: &str) -> Result<String, std::io::Error> {
    let output = Command::new("git")
        .args(["checkout", "-b", branch])
        .output()?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        let err = String::from_utf8_lossy(&output.stderr).trim().to_string();
        Err(std::io::Error::new(std::io::ErrorKind::Other, err))
    }
}
