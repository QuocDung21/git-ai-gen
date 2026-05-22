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

    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr).trim().to_string();
        return Err(std::io::Error::new(std::io::ErrorKind::Other, err));
    }

    let mut msg = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if msg.is_empty() {
        msg = String::from_utf8_lossy(&output.stderr).trim().to_string();
    }
    match push_branch(branch) {
        Ok(_) => Ok(format!(
            "{}\nĐã push nhánh '{}' lên origin thành công.",
            msg, branch
        )),
        Err(e) => Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Tạo nhánh thành công nhưng push thất bại:\n{}", e),
        )),
    }
}

fn push_branch(branch: &str) -> Result<String, std::io::Error> {
    let output = Command::new("git")
        .args(["push", "-u", "origin", branch])
        .output()?;

    if output.status.success() {
        let mut msg = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if msg.is_empty() {
            msg = String::from_utf8_lossy(&output.stderr).trim().to_string();
        }
        Ok(msg)
    } else {
        let err = String::from_utf8_lossy(&output.stderr).trim().to_string();
        Err(std::io::Error::new(std::io::ErrorKind::Other, err))
    }
}

pub struct DeleteBranchOptions {
    pub local: bool,
    pub remote: bool,
    pub force: bool,
}

impl Default for DeleteBranchOptions {
    fn default() -> Self {
        Self {
            local: true,
            remote: false,
            force: false,
        }
    }
}

// Delete branch
#[allow(dead_code)]
pub fn delete_branch(branch: &str, options: DeleteBranchOptions) -> Result<String, std::io::Error> {
    let mut messages = Vec::new();

    if options.local {
        let delete_flag = if options.force { "-D" } else { "-d" };
        let output = Command::new("git")
            .args(["branch", delete_flag, branch])
            .output()?;

        if output.status.success() {
            let msg = String::from_utf8_lossy(&output.stdout).trim().to_string();
            messages.push(format!("Local: {}", msg));
        } else {
            let err = String::from_utf8_lossy(&output.stderr).trim().to_string();
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Lỗi xóa nhánh local '{}':\n{}", branch, err),
            ));
        }
    }

    if options.remote {
        let output = Command::new("git")
            .args(["push", "origin", "--delete", branch])
            .output()?;

        if output.status.success() {
            let mut msg = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if msg.is_empty() {
                msg = String::from_utf8_lossy(&output.stderr).trim().to_string();
            }
            messages.push(format!("Remote: {}", msg));
        } else {
            let err = String::from_utf8_lossy(&output.stderr).trim().to_string();
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Lỗi xóa nhánh remote '{}':\n{}", branch, err),
            ));
        }
    }

    if messages.is_empty() {
        Ok("Không có hành động xóa nào được yêu cầu.".to_string())
    } else {
        Ok(messages.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_branches() {
        let branches = get_branches();
        println!("Danh sách nhánh: {:#?}", branches);
        assert!(
            branches.len() > 0,
            "Nên có ít nhất 1 nhánh (thường là main/master)"
        );
    }

    #[test]
    fn test_delete_branch() {
        let result = delete_branch("", DeleteBranchOptions::default());
        assert!(
            result.is_ok(),
            "Xóa nhánh không phải là hành động quan trọng nên không cần phải trả về lỗi"
        );
    }
}
