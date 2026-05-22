use std::process::Command;

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
    if let Ok(out) = Command::new("git")
        .args(["diff", "--", path])
        .output()
    {
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
    Command::new("git").args(["add", "-A"]).output()?;
    Ok(())
}

pub fn unstage_all() -> Result<(), std::io::Error> {
    Command::new("git").args(["reset"]).output()?;
    Ok(())
}

pub fn stage_file(path: &str) -> Result<(), std::io::Error> {
    Command::new("git").args(["add", "--", path]).output()?;
    Ok(())
}

pub fn unstage_file(path: &str) -> Result<(), std::io::Error> {
    Command::new("git").args(["restore", "--staged", "--", path]).output()?;
    Ok(())
}

pub fn revert_file(path: &str) -> Result<(), std::io::Error> {
    Command::new("git").args(["restore", "--", path]).output()?;
    Ok(())
}
