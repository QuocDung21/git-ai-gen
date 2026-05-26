use std::fs;
use std::process::Command;
use std::sync::Mutex;
use tempfile::TempDir;

use git_ai_core::git::branch::{self, DeleteBranchOptions};

/// Serialize all tests that mutate the process current working directory.
/// Prevents races when `cargo test` runs tests in parallel.
static GIT_TEST_LOCK: Mutex<()> = Mutex::new(());

fn setup_test_repo() -> TempDir {
    let tmp = tempfile::tempdir().unwrap();
    let repo = tmp.path();

    // Force initial branch name to "master" for test stability
    Command::new("git")
        .current_dir(repo)
        .args(["init", "-b", "master"])
        .output()
        .unwrap();
    Command::new("git")
        .current_dir(repo)
        .args(["config", "user.email", "test@git.com"])
        .output()
        .unwrap();
    Command::new("git")
        .current_dir(repo)
        .args(["config", "user.name", "Tester"])
        .output()
        .unwrap();
    Command::new("git")
        .current_dir(repo)
        .args(["commit", "--allow-empty", "-m", "Init"])
        .output()
        .unwrap();

    // Change process CWD so that bare `Command::new("git")` calls in the
    // functions under test operate inside this temporary repository.
    // Protected by GIT_TEST_LOCK in each test.
    std::env::set_current_dir(repo).unwrap();
    tmp
}

#[test]
fn test_force_delete_vs_normal_delete() {
    let _guard = GIT_TEST_LOCK.lock().unwrap_or_else(|p| p.into_inner());
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

    let options_normal = git_ai_core::git::branch::DeleteBranchOptions {
        local: true,
        remote: false,
        force: false,
    };
    let res_normal = git_ai_core::git::branch::delete_branch("unmerged-branch", options_normal);
    assert!(res_normal.is_ok(), "delete_branch should not hard-fail");
    let msg_normal = res_normal.unwrap();
    assert!(
        msg_normal.contains("Error ignored") || msg_normal.contains("Could not delete"),
        "Normal delete on unmerged branch should report error (got: {})",
        msg_normal
    );

    let options_force = git_ai_core::git::branch::DeleteBranchOptions {
        local: true,
        remote: false,
        force: true,
    };
    let res_force = git_ai_core::git::branch::delete_branch("unmerged-branch", options_force);
    assert!(res_force.is_ok(), "Force delete should succeed");
}

#[test]
fn test_merge_conflict_scenario() {
    let _guard = GIT_TEST_LOCK.lock().unwrap_or_else(|p| p.into_inner());
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
    let res = git_ai_core::git::branch::git_merge("branch-b");

    assert!(res.is_err(), "Merge should fail due to conflict");
    let err_msg = res.unwrap_err().to_string();
    assert!(
        err_msg.contains("CONFLICT") || err_msg.contains("conflict"),
        "Error message should contain conflict details, got: {}",
        err_msg
    );
}

#[test]
fn test_remote_auto_detection() {
    let _guard = GIT_TEST_LOCK.lock().unwrap_or_else(|p| p.into_inner());
    let _tmp = setup_test_repo();
    let options = git_ai_core::git::branch::DeleteBranchOptions {
        local: true,
        remote: false,
        force: false,
    };

    // Use a non-protected remote branch name so we don't hit the protected_branch early return.
    // After "origin/" stripping it becomes "feature-x" (not main/master).
    let res = git_ai_core::git::branch::delete_branch("origin/feature-x", options);

    assert!(res.is_ok());
    let msg = res.unwrap();
    assert!(
        msg.contains("Remote:") && msg.contains("does not exist on origin"),
        "Expected remote non-existence message, got: {}",
        msg
    );
}

#[test]
fn test_delete_non_existent_branch() {
    let _guard = GIT_TEST_LOCK.lock().unwrap_or_else(|p| p.into_inner());
    let _tmp = setup_test_repo();

    let options = git_ai_core::git::branch::DeleteBranchOptions {
        local: true,
        remote: false,
        force: false,
    };
    let res = git_ai_core::git::branch::delete_branch("ghost-branch", options);

    assert!(res.is_ok());
    assert!(res.unwrap().contains("does not exist, skipping"));
}

// ============================================================================
// get_branches TESTS
// ============================================================================

#[test]
fn test_get_branches_contains_master() {
    let _guard = GIT_TEST_LOCK.lock().unwrap_or_else(|p| p.into_inner());
    let _tmp = setup_test_repo();

    let branches = branch::get_branches();

    assert!(!branches.is_empty(), "Should have at least one branch");
    let master = branches.iter().any(|b| b.name == "master" && !b.is_remote);
    assert!(master, "Expected local branch 'master' in get_branches()");
}

#[test]
fn test_get_branches_includes_new_branch() {
    let _guard = GIT_TEST_LOCK.lock().unwrap_or_else(|p| p.into_inner());
    let _tmp = setup_test_repo();

    Command::new("git")
        .args(["branch", "feature/test-1"])
        .output()
        .unwrap();

    let branches = branch::get_branches();
    let found = branches
        .iter()
        .any(|b| b.name == "feature/test-1" && !b.is_remote);
    assert!(
        found,
        "Expected new branch 'feature/test-1' in get_branches()"
    );
}

#[test]
fn test_get_branches_filters_head() {
    let _guard = GIT_TEST_LOCK.lock().unwrap_or_else(|p| p.into_inner());
    let _tmp = setup_test_repo();

    let branches = branch::get_branches();
    let has_head = branches.iter().any(|b| b.name.ends_with("/HEAD"));
    assert!(
        !has_head,
        "get_branches() should filter out remote HEAD references"
    );
}

// ============================================================================
// checkout_branch TESTS
// ============================================================================

#[test]
fn test_checkout_branch_switches_branch() {
    let _guard = GIT_TEST_LOCK.lock().unwrap_or_else(|p| p.into_inner());
    let _tmp = setup_test_repo();

    Command::new("git")
        .args(["branch", "feature/checkout-test"])
        .output()
        .unwrap();

    let res = branch::checkout_branch("feature/checkout-test");
    assert!(res.is_ok(), "Checkout should succeed");

    let current = branch::get_current_branch();
    assert_eq!(current, Some("feature/checkout-test".to_string()));
}

#[test]
fn test_checkout_branch_fails_for_nonexistent() {
    let _guard = GIT_TEST_LOCK.lock().unwrap_or_else(|p| p.into_inner());
    let _tmp = setup_test_repo();

    let res = branch::checkout_branch("branch-does-not-exist");
    assert!(res.is_err(), "Checkout of non-existent branch should fail");
}

// ============================================================================
// get_current_branch TESTS
// ============================================================================

#[test]
fn test_get_current_branch_returns_correct_name() {
    let _guard = GIT_TEST_LOCK.lock().unwrap_or_else(|p| p.into_inner());
    let _tmp = setup_test_repo();

    let current = branch::get_current_branch();
    assert_eq!(current, Some("master".to_string()));
}

#[test]
fn test_get_current_branch_after_checkout() {
    let _guard = GIT_TEST_LOCK.lock().unwrap_or_else(|p| p.into_inner());
    let _tmp = setup_test_repo();

    Command::new("git")
        .args(["branch", "other-branch"])
        .output()
        .unwrap();
    Command::new("git")
        .args(["checkout", "other-branch"])
        .output()
        .unwrap();

    let current = branch::get_current_branch();
    assert_eq!(current, Some("other-branch".to_string()));
}

// ============================================================================
// protected_branch TESTS
// ============================================================================

#[test]
fn test_protected_branch_returns_true_for_main() {
    assert!(branch::protected_branch("main"));
}

#[test]
fn test_protected_branch_returns_true_for_master() {
    assert!(branch::protected_branch("master"));
}

#[test]
fn test_protected_branch_returns_true_for_origin_main() {
    assert!(branch::protected_branch("origin/main"));
}

#[test]
fn test_protected_branch_returns_false_for_feature() {
    assert!(!branch::protected_branch("feature/awesome"));
}

#[test]
fn test_protected_branch_returns_false_for_empty() {
    assert!(!branch::protected_branch(""));
}

// ============================================================================
// branch_exists TESTS
// ============================================================================

#[test]
fn test_branch_exists_locally() {
    let _guard = GIT_TEST_LOCK.lock().unwrap_or_else(|p| p.into_inner());
    let _tmp = setup_test_repo();

    assert!(
        branch::branch_exists("master", false),
        "master should exist locally"
    );
}

#[test]
fn test_branch_does_not_exist_locally() {
    let _guard = GIT_TEST_LOCK.lock().unwrap_or_else(|p| p.into_inner());
    let _tmp = setup_test_repo();

    assert!(
        !branch::branch_exists("ghost-branch", false),
        "ghost-branch should not exist"
    );
}

#[test]
fn test_branch_exists_after_creation() {
    let _guard = GIT_TEST_LOCK.lock().unwrap_or_else(|p| p.into_inner());
    let _tmp = setup_test_repo();

    Command::new("git")
        .args(["branch", "newly-created"])
        .output()
        .unwrap();
    assert!(branch::branch_exists("newly-created", false));
}

#[test]
fn test_branch_not_found_in_empty_repo() {
    let _guard = GIT_TEST_LOCK.lock().unwrap_or_else(|p| p.into_inner());
    let tmp = tempfile::tempdir().unwrap();
    Command::new("git")
        .current_dir(tmp.path())
        .args(["init", "-b", "main"])
        .output()
        .unwrap();
    // No commit yet, so no real branches
    std::env::set_current_dir(tmp.path()).unwrap();

    assert!(
        !branch::branch_exists("main", false),
        "Branch should not be resolvable before any commit"
    );
}

// ============================================================================
// delete_branch — EDGE CASES
// ============================================================================

#[test]
fn test_delete_protected_branch_main_fails() {
    let _guard = GIT_TEST_LOCK.lock().unwrap_or_else(|p| p.into_inner());
    let _tmp = setup_test_repo();

    let res = branch::delete_branch("main", DeleteBranchOptions::default());
    assert!(res.is_err(), "Deleting 'main' should be rejected");
    let err = res.unwrap_err().to_string();
    assert!(err.contains("protected") || err.contains("WARNING"));
}

#[test]
fn test_delete_protected_branch_master_fails() {
    let _guard = GIT_TEST_LOCK.lock().unwrap_or_else(|p| p.into_inner());
    let _tmp = setup_test_repo();

    let res = branch::delete_branch("master", DeleteBranchOptions::default());
    assert!(res.is_err(), "Deleting 'master' should be rejected");
    let err = res.unwrap_err().to_string();
    assert!(err.contains("protected") || err.contains("WARNING"));
}

#[test]
fn test_delete_current_branch_fails() {
    let _guard = GIT_TEST_LOCK.lock().unwrap_or_else(|p| p.into_inner());
    let _tmp = setup_test_repo();

    Command::new("git")
        .args(["checkout", "-b", "temporary"])
        .output()
        .unwrap();

    let res = branch::delete_branch(
        "temporary",
        DeleteBranchOptions {
            local: true,
            remote: false,
            force: true,
        },
    );
    assert!(res.is_err(), "Deleting the current branch should fail");
}

#[test]
fn test_successful_force_delete_non_current_branch() {
    let _guard = GIT_TEST_LOCK.lock().unwrap_or_else(|p| p.into_inner());
    let _tmp = setup_test_repo();

    Command::new("git")
        .args(["checkout", "-b", "to-delete"])
        .output()
        .unwrap();
    fs::write("temp.txt", "data").unwrap();
    Command::new("git").args(["add", "."]).output().unwrap();
    Command::new("git")
        .args(["commit", "-m", "temp"])
        .output()
        .unwrap();

    Command::new("git")
        .args(["checkout", "master"])
        .output()
        .unwrap();

    let res = branch::delete_branch(
        "to-delete",
        DeleteBranchOptions {
            local: true,
            remote: false,
            force: true,
        },
    );
    assert!(res.is_ok(), "Force delete should succeed: {:?}", res);

    assert!(!branch::branch_exists("to-delete", false));
}

// ============================================================================
// git_merge — HAPPY PATH EDGE CASES
// ============================================================================

#[test]
fn test_merge_fast_forward() {
    let _guard = GIT_TEST_LOCK.lock().unwrap_or_else(|p| p.into_inner());
    let _tmp = setup_test_repo();

    Command::new("git")
        .args(["checkout", "-b", "ff-branch"])
        .output()
        .unwrap();
    fs::write("ff.txt", "fast-forward").unwrap();
    Command::new("git").args(["add", "."]).output().unwrap();
    Command::new("git")
        .args(["commit", "-m", "FF commit"])
        .output()
        .unwrap();

    Command::new("git")
        .args(["checkout", "master"])
        .output()
        .unwrap();
    let res = branch::git_merge("ff-branch");
    assert!(res.is_ok(), "Fast-forward merge should succeed");
    let msg = res.unwrap();
    assert!(
        msg.contains("Fast-forward") || msg.contains("ff-branch"),
        "Merge output should mention fast-forward, got: {}",
        msg
    );
}

#[test]
fn test_merge_up_to_date() {
    let _guard = GIT_TEST_LOCK.lock().unwrap_or_else(|p| p.into_inner());
    let _tmp = setup_test_repo();

    let res = branch::git_merge("master");
    assert!(res.is_ok());
    let msg = res.unwrap();
    assert!(
        msg.contains("Already up to date") || msg.contains("Already up-to-date"),
        "Merge master into master should say up-to-date, got: {}",
        msg
    );
}
