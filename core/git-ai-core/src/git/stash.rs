use std::process::Command;

pub fn get_stash_list() -> Result<String, std::io::Error> {
    let output = Command::new("git")
        .args(["stash", "list", "--format=%gd|%gs"])
        .output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

pub fn stash_push() -> Result<String, std::io::Error> {
    let output = Command::new("git")
        .args(["stash", "push", "-m", "WIP stash"])
        .output()?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        let err = String::from_utf8_lossy(&output.stderr).trim().to_string();
        Err(std::io::Error::other(err))
    }
}

pub fn stash_pop(stash_ref: &str) -> Result<String, std::io::Error> {
    let output = Command::new("git")
        .args(["stash", "pop", stash_ref])
        .output()?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        let err = String::from_utf8_lossy(&output.stderr).trim().to_string();
        Err(std::io::Error::other(err))
    }
}

pub fn stash_apply(stash_ref: &str) -> Result<String, std::io::Error> {
    let output = Command::new("git")
        .args(["stash", "apply", stash_ref])
        .output()?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        let err = String::from_utf8_lossy(&output.stderr).trim().to_string();
        Err(std::io::Error::other(err))
    }
}

pub fn stash_drop(stash_ref: &str) -> Result<String, std::io::Error> {
    let output = Command::new("git")
        .args(["stash", "drop", stash_ref])
        .output()?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        let err = String::from_utf8_lossy(&output.stderr).trim().to_string();
        Err(std::io::Error::other(err))
    }
}
