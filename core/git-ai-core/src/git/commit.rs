use std::process::Command;

pub fn commit(message: &str) -> Result<String, std::io::Error> {
    let output = Command::new("git")
        .args(["commit", "-m", message])
        .output()?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        let err = String::from_utf8_lossy(&output.stderr).trim().to_string();
        Err(std::io::Error::other(err))
    }
}

pub fn get_last_commit_subject() -> String {
    if let Ok(out) = Command::new("git")
        .args(["log", "-1", "--pretty=format:%s"])
        .output()
    {
        String::from_utf8_lossy(&out.stdout).trim().to_string()
    } else {
        String::new()
    }
}

pub fn amend_commit(message: &str) -> Result<String, std::io::Error> {
    let output = Command::new("git")
        .args(["commit", "--amend", "-m", message])
        .output()?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        let err = String::from_utf8_lossy(&output.stderr).trim().to_string();
        Err(std::io::Error::other(err))
    }
}

pub fn get_commit_diff(hash: &str) -> String {
    if let Ok(out) = Command::new("git")
        .args(["show", "--stat", "--patch", hash])
        .output()
    {
        String::from_utf8_lossy(&out.stdout).to_string()
    } else {
        format!("Error: could not get diff for {}", hash)
    }
}
