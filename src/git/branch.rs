use crate::app::models::BranchEntry;
use std::process::Command;

/// Get a list of all branches (including local and remote) from the Git system.
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

/// Checkout the specified Git branch.
pub fn checkout_branch(branch: &str) -> Result<String, std::io::Error> {
    let output = Command::new("git").args(["checkout", branch]).output()?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        let err = String::from_utf8_lossy(&output.stderr).trim().to_string();
        Err(std::io::Error::new(std::io::ErrorKind::Other, err))
    }
}

/// Merge a branch into the current branch.
pub fn git_merge(branch: &str) -> Result<String, std::io::Error> {
    let output = Command::new("git").args(["merge", branch]).output()?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        let err = String::from_utf8_lossy(&output.stderr).trim().to_string();
        Err(std::io::Error::new(std::io::ErrorKind::Other, err))
    }
}

/// Create a new branch, checkout, and push it to the origin remote.
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
            "{}\nSuccessfully pushed branch '{}' to origin.",
            msg, branch
        )),
        Err(e) => Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Branch created, but failed to push:\n{}", e),
        )),
    }
}

/// Push a branch to the origin remote.
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

/// Configuration options for deleting a branch.
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

/// Get the name of the current working branch.
#[allow(dead_code)]
pub fn get_current_branch() -> Option<String> {
    if let Ok(output) = Command::new("git")
        .args(["branch", "--show-current"])
        .output()
    {
        let name = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !name.is_empty() {
            return Some(name);
        }
    }
    None
}

/// Delete a local or remote branch based on the DeleteBranchOptions.
#[allow(dead_code)]
pub fn delete_branch(
    mut branch: &str,
    mut options: DeleteBranchOptions,
) -> Result<String, std::io::Error> {
    // Automatically detect if the branch name is remote (starts with "origin/")
    if let Some(real_name) = branch.strip_prefix("origin/") {
        branch = real_name;
        options.local = false;
        options.remote = true;
    }

    // Check if the branch is protected
    if protected_branch(branch) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::PermissionDenied,
            format!(
                "WARNING: Branch '{}' is protected and cannot be deleted.",
                branch
            ),
        ));
    }

    // Check if the user is currently on the local branch to be deleted
    if options.local {
        if let Some(current) = get_current_branch() {
            if current == branch {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "WARNING: Cannot delete local branch '{}' while currently on it.",
                        branch
                    ),
                ));
            }
        }
    }

    let mut messages = Vec::new();

    // Handle local deletion
    if options.local {
        if !branch_exists(branch, false) {
            messages.push(format!(
                "Local: Branch '{}' does not exist, skipping.",
                branch
            ));
        } else {
            let delete_flag = if options.force { "-D" } else { "-d" };
            let output = Command::new("git")
                .args(["branch", delete_flag, branch])
                .output()?;

            if output.status.success() {
                let msg = String::from_utf8_lossy(&output.stdout).trim().to_string();
                messages.push(format!("Local: {}", msg));
            } else {
                let err = String::from_utf8_lossy(&output.stderr).trim().to_string();
                messages.push(format!(
                    "Local (Error ignored): Could not delete '{}' - {}",
                    branch, err
                ));
            }
        }
    }

    // Handle remote deletion
    if options.remote {
        if !branch_exists(branch, true) {
            messages.push(format!(
                "Remote: Branch '{}' does not exist on origin, skipping.",
                branch
            ));
        } else {
            let output = Command::new("git")
                .env("LC_ALL", "C")
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
                messages.push(format!(
                    "Remote (Error ignored): Could not delete '{}' - {}",
                    branch, err
                ));
            }
        }
    }

    if messages.is_empty() {
        Ok("No deletion actions required.".to_string())
    } else {
        Ok(messages.join("\n"))
    }
}

/// Check if the branch is in the list of protected branches (cannot be deleted).
#[allow(dead_code)]
pub fn protected_branch(branch: &str) -> bool {
    matches!(branch, "main" | "master" | "origin/main" | "origin/master")
}

/// Check if a branch exists.
/// Set `check_remote = false` to check locally, or `true` to check on the origin remote.
#[allow(dead_code)]
pub fn branch_exists(branch: &str, check_remote: bool) -> bool {
    if check_remote {
        if let Ok(output) = Command::new("git")
            .args(["ls-remote", "--heads", "origin", branch])
            .output()
        {
            let stdout = String::from_utf8_lossy(&output.stdout);
            !stdout.trim().is_empty()
        } else {
            false
        }
    } else {
        let ref_path = format!("refs/heads/{}", branch);

        if let Ok(output) = Command::new("git")
            .args(["rev-parse", "--verify", "--quiet", &ref_path])
            .output()
        {
            output.status.success()
        } else {
            false
        }
    }
}

#[test]
fn test_exits_path() {
    let path_to_verify;
    {
        let tmp = tempfile::tempdir().unwrap();
        path_to_verify = tmp.path().to_path_buf();
        println!("Inside scope: Path là {:?}", path_to_verify);
        assert!(path_to_verify.exists(), "Thư mục phải tồn tại trong scope!");
    }

    println!(
        "Outside scope: Đường dẫn {:?} còn tồn tại không? -> {}",
        path_to_verify,
        path_to_verify.exists()
    );

    assert!(!path_to_verify.exists(), "Thư mục đã bị Drop (xóa) rồi!");
}

#[cfg(test)]
mod additional_tests {
    use super::*;
    use std::fs;
    use std::process::Command;
    use tempfile::TempDir;

    fn setup_test_repo() -> TempDir {
        let tmp = tempfile::tempdir().unwrap();

        std::env::set_current_dir(tmp.path()).unwrap();
        Command::new("git").args(["init"]).output().unwrap();
        Command::new("git")
            .args(["config", "user.email", "test@git.com"])
            .output()
            .unwrap();
        Command::new("git")
            .args(["config", "user.name", "Tester"])
            .output()
            .unwrap();
        Command::new("git")
            .args(["commit", "--allow-empty", "-m", "Init"])
            .output()
            .unwrap();
        tmp
    }

    #[test]
    fn test_force_delete_vs_normal_delete() {
        let _tmp = setup_test_repo();
        Command::new("git")
            .args(["checkout", "-b", "unmerged-branch"])
            .output()
            .unwrap();
        fs::write("test.txt", "data").unwrap();
        Command::new("git").args(["add", "."]).output().unwrap();
        Command::new("git")
            .args(["commit", "-m", "commit"])
            .output()
            .unwrap();

        Command::new("git")
            .args(["checkout", "master"])
            .output()
            .unwrap();

        let options_normal = DeleteBranchOptions {
            local: true,
            remote: false,
            force: false,
        };
        let res_normal = delete_branch("unmerged-branch", options_normal);
        assert!(
            res_normal.is_err(),
            "Normal delete should fail for unmerged branch"
        );

        let options_force = DeleteBranchOptions {
            local: true,
            remote: false,
            force: true,
        };
        let res_force = delete_branch("unmerged-branch", options_force);
        assert!(res_force.is_ok(), "Force delete should succeed");
    }

    #[test]
    fn test_merge_conflict_scenario() {
        let _tmp = setup_test_repo();

        Command::new("git")
            .args(["checkout", "-b", "branch-a"])
            .output()
            .unwrap();
        fs::write("file.txt", "A").unwrap();
        Command::new("git").args(["add", "."]).output().unwrap();
        Command::new("git")
            .args(["commit", "-m", "A"])
            .output()
            .unwrap();

        Command::new("git")
            .args(["checkout", "master"])
            .output()
            .unwrap();
        Command::new("git")
            .args(["checkout", "-b", "branch-b"])
            .output()
            .unwrap();
        fs::write("file.txt", "B").unwrap();
        Command::new("git").args(["add", "."]).output().unwrap();
        Command::new("git")
            .args(["commit", "-m", "B"])
            .output()
            .unwrap();

        Command::new("git")
            .args(["checkout", "branch-a"])
            .output()
            .unwrap();
        let res = git_merge("branch-b");

        assert!(res.is_err(), "Merge should fail due to conflict");
    }

    #[test]
    fn test_remote_auto_detection() {
        let _tmp = setup_test_repo();
        let options = DeleteBranchOptions {
            local: true,
            remote: false,
            force: false,
        };

        let res = delete_branch("origin/main", options);

        assert!(res.is_ok());
        assert!(res
            .unwrap()
            .contains("Remote: Branch 'my-branch' does not exist on origin"));
    }

    #[test]
    fn test_delete_non_existent_branch() {
        let _tmp = setup_test_repo();

        let options = DeleteBranchOptions {
            local: true,
            remote: false,
            force: false,
        };
        let res = delete_branch("ghost-branch", options);

        assert!(res.is_ok());
        assert!(res.unwrap().contains("does not exist, skipping"));
    }
}
