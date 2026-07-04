use std::process::Command;

fn run_git(args: &[&str]) -> Result<(), std::io::Error> {
    let output = Command::new("git").args(args).output()?;
    if output.status.success() {
        Ok(())
    } else {
        let err = String::from_utf8_lossy(&output.stderr).trim().to_string();
        Err(std::io::Error::other(err))
    }
}

pub fn get_git_status() -> Result<String, std::io::Error> {
    let output = Command::new("git").args(["status", "-s"]).output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

pub fn get_current_branch() -> String {
    if let Ok(output) = Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
    {
        let br = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if br.is_empty() {
            "detached".to_string()
        } else {
            br
        }
    } else {
        "detached".to_string()
    }
}

pub fn get_diff_head(path: &str) -> Option<String> {
    if let Ok(out) = Command::new("git")
        .args(["diff", "HEAD", "--", path])
        .output()
    {
        let diff = String::from_utf8_lossy(&out.stdout).to_string();
        if !diff.trim().is_empty() {
            return Some(diff);
        }
    }
    None
}

pub fn get_diff_unstaged(path: &str) -> Option<String> {
    if let Ok(out) = Command::new("git").args(["diff", "--", path]).output() {
        let diff = String::from_utf8_lossy(&out.stdout).to_string();
        if !diff.trim().is_empty() {
            return Some(diff);
        }
    }
    None
}

pub fn get_diff_staged(path: &str) -> Option<String> {
    if let Ok(out) = Command::new("git")
        .args(["diff", "--cached", "--", path])
        .output()
    {
        let diff = String::from_utf8_lossy(&out.stdout).to_string();
        if !diff.trim().is_empty() {
            return Some(diff);
        }
    }
    None
}

pub fn stage_all() -> Result<(), std::io::Error> {
    run_git(&["add", "-A"])
}

pub fn unstage_all() -> Result<(), std::io::Error> {
    run_git(&["reset"])
}

pub fn stage_file(path: &str) -> Result<(), std::io::Error> {
    run_git(&["add", "--", path])
}

pub fn unstage_file(path: &str) -> Result<(), std::io::Error> {
    run_git(&["restore", "--staged", "--", path])
}

pub fn revert_file(path: &str) -> Result<(), std::io::Error> {
    run_git(&["restore", "--", path])
}
