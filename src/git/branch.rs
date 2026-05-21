use std::process::Command;

pub fn get_branches() -> Vec<String> {
    let mut branches = Vec::new();
    if let Ok(output) = Command::new("git")
        .args(["branch", "--format=%(refname:short)"])
        .output()
    {
        let branches_text = String::from_utf8_lossy(&output.stdout);
        for line in branches_text.lines() {
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                branches.push(trimmed.to_string());
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
