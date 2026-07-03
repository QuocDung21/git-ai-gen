use serde::Serialize;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::os::raw::c_void;
use std::path::Path;
use std::process::Command;

fn setup_env_path() {
    static INITIALIZED: std::sync::Once = std::sync::Once::new();
    INITIALIZED.call_once(|| {
        let current_path = std::env::var("PATH").unwrap_or_default();
        if let Ok(output) = Command::new("/bin/zsh")
            .args(["-l", "-c", "echo $PATH"])
            .output()
        {
            if output.status.success() {
                let shell_path = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !shell_path.is_empty() {
                    let combined = format!("{}:{}", shell_path, current_path);
                    std::env::set_var("PATH", combined);
                    return;
                }
            }
        }
        let home = std::env::var("HOME").unwrap_or_default();
        let fallback = format!(
            "{}/.nvm/versions/node/v22.14.0/bin:{}/.cargo/bin:/opt/homebrew/bin:/usr/local/bin:/usr/bin:/bin:/usr/sbin:/sbin:{}",
            home, home, current_path
        );
        std::env::set_var("PATH", fallback);
    });
}

#[derive(Serialize)]
struct ChangedFileJson {
    status: String,
    path: String,
}

#[derive(Serialize)]
struct CleanupScanJson {
    items: Vec<CleanupItemJson>,
}

#[derive(Serialize)]
struct CleanupItemJson {
    path: String,
    target: String,
    size_bytes: u64,
}

#[derive(Serialize)]
struct CleanupDeleteJson {
    reports: Vec<crate::cleanup::DeleteReport>,
}

#[derive(Serialize)]
struct CleanupStreamDoneJson {
    done: bool,
}

#[derive(Serialize)]
struct FfiErrorJson {
    error: String,
}

type CleanupScanCallback = Option<
    extern "C" fn(
        path: *const c_char,
        target: *const c_char,
        size_bytes: u64,
        user_data: *mut c_void,
    ),
>;
type CleanupShouldCancelCallback = Option<extern "C" fn(user_data: *mut c_void) -> bool>;

fn string_from_c_char(value: *const c_char) -> Option<String> {
    if value.is_null() {
        return None;
    }

    let c_str = unsafe { CStr::from_ptr(value) };
    c_str.to_str().ok().map(str::to_string)
}

fn json_string<T: Serialize>(value: &T) -> *mut c_char {
    let json = serde_json::to_string(value).unwrap_or_else(|_| "{}".to_string());
    CString::new(json).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn git_ai_get_status() -> *mut c_char {
    setup_env_path();
    let mut files = Vec::new();
    if let Ok(status_text) = crate::git::status::get_git_status() {
        for line in status_text.lines() {
            let trimmed = line.trim();
            if trimmed.len() >= 3 {
                let status = line[..3].to_string();
                let path = line[3..].trim().to_string();
                files.push(ChangedFileJson { status, path });
            }
        }
    }
    let json = serde_json::to_string(&files).unwrap_or_else(|_| "[]".to_string());
    CString::new(json).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn git_ai_cleanup_scan_node_modules(path: *const c_char) -> *mut c_char {
    cleanup_scan(path, crate::cleanup::CleanupTarget::NodeModules)
}

#[no_mangle]
pub extern "C" fn git_ai_cleanup_scan_build_folders(path: *const c_char) -> *mut c_char {
    cleanup_scan(path, crate::cleanup::CleanupTarget::BuildFolders)
}

#[no_mangle]
pub extern "C" fn git_ai_cleanup_scan_devcleaner(path: *const c_char) -> *mut c_char {
    cleanup_scan(path, crate::cleanup::CleanupTarget::DevCleaner)
}

#[no_mangle]
pub extern "C" fn git_ai_cleanup_scan_node_modules_stream(
    path: *const c_char,
    callback: CleanupScanCallback,
    user_data: *mut c_void,
) -> *mut c_char {
    cleanup_scan_stream(
        path,
        crate::cleanup::CleanupTarget::NodeModules,
        callback,
        user_data,
    )
}

#[no_mangle]
pub extern "C" fn git_ai_cleanup_scan_node_modules_stream_cancellable(
    path: *const c_char,
    callback: CleanupScanCallback,
    should_cancel: CleanupShouldCancelCallback,
    user_data: *mut c_void,
) -> *mut c_char {
    cleanup_scan_stream_cancellable(
        path,
        crate::cleanup::CleanupTarget::NodeModules,
        callback,
        should_cancel,
        user_data,
    )
}

#[no_mangle]
pub extern "C" fn git_ai_cleanup_scan_build_folders_stream(
    path: *const c_char,
    callback: CleanupScanCallback,
    user_data: *mut c_void,
) -> *mut c_char {
    cleanup_scan_stream(
        path,
        crate::cleanup::CleanupTarget::BuildFolders,
        callback,
        user_data,
    )
}

#[no_mangle]
pub extern "C" fn git_ai_cleanup_scan_build_folders_stream_cancellable(
    path: *const c_char,
    callback: CleanupScanCallback,
    should_cancel: CleanupShouldCancelCallback,
    user_data: *mut c_void,
) -> *mut c_char {
    cleanup_scan_stream_cancellable(
        path,
        crate::cleanup::CleanupTarget::BuildFolders,
        callback,
        should_cancel,
        user_data,
    )
}

#[no_mangle]
pub extern "C" fn git_ai_cleanup_scan_devcleaner_stream(
    path: *const c_char,
    callback: CleanupScanCallback,
    user_data: *mut c_void,
) -> *mut c_char {
    cleanup_scan_stream(
        path,
        crate::cleanup::CleanupTarget::DevCleaner,
        callback,
        user_data,
    )
}

#[no_mangle]
pub extern "C" fn git_ai_cleanup_scan_devcleaner_stream_cancellable(
    path: *const c_char,
    callback: CleanupScanCallback,
    should_cancel: CleanupShouldCancelCallback,
    user_data: *mut c_void,
) -> *mut c_char {
    cleanup_scan_stream_cancellable(
        path,
        crate::cleanup::CleanupTarget::DevCleaner,
        callback,
        should_cancel,
        user_data,
    )
}

#[no_mangle]
pub extern "C" fn git_ai_cleanup_delete_paths(paths_json: *const c_char) -> *mut c_char {
    let Some(paths_json) = string_from_c_char(paths_json) else {
        return json_string(&FfiErrorJson {
            error: "Invalid paths JSON pointer".to_string(),
        });
    };

    let paths = match serde_json::from_str::<Vec<String>>(&paths_json) {
        Ok(paths) => paths,
        Err(error) => {
            return json_string(&FfiErrorJson {
                error: error.to_string(),
            });
        }
    };

    let reports = crate::cleanup::delete_paths(&paths);
    json_string(&CleanupDeleteJson { reports })
}

fn cleanup_scan(path: *const c_char, target: crate::cleanup::CleanupTarget) -> *mut c_char {
    let Some(path) = string_from_c_char(path) else {
        return json_string(&FfiErrorJson {
            error: "Invalid path pointer".to_string(),
        });
    };

    let items = crate::cleanup::scan_folders(Path::new(&path), target)
        .into_iter()
        .map(|task| CleanupItemJson {
            path: task.path.display().to_string(),
            target: format!("{:?}", task.target),
            size_bytes: task.size_bytes,
        })
        .collect::<Vec<_>>();

    json_string(&CleanupScanJson { items })
}

fn cleanup_scan_stream(
    path: *const c_char,
    target: crate::cleanup::CleanupTarget,
    callback: CleanupScanCallback,
    user_data: *mut c_void,
) -> *mut c_char {
    let Some(path) = string_from_c_char(path) else {
        return json_string(&FfiErrorJson {
            error: "Invalid path pointer".to_string(),
        });
    };

    let Some(callback) = callback else {
        return json_string(&FfiErrorJson {
            error: "Invalid scan callback".to_string(),
        });
    };

    crate::cleanup::scan_folders_each(Path::new(&path), target, |task| {
        let Ok(path) = CString::new(task.path.display().to_string()) else {
            return;
        };
        let Ok(target) = CString::new(format!("{:?}", task.target)) else {
            return;
        };
        callback(path.as_ptr(), target.as_ptr(), task.size_bytes, user_data);
    });

    json_string(&CleanupStreamDoneJson { done: true })
}

fn cleanup_scan_stream_cancellable(
    path: *const c_char,
    target: crate::cleanup::CleanupTarget,
    callback: CleanupScanCallback,
    should_cancel: CleanupShouldCancelCallback,
    user_data: *mut c_void,
) -> *mut c_char {
    let Some(path) = string_from_c_char(path) else {
        return json_string(&FfiErrorJson {
            error: "Invalid path pointer".to_string(),
        });
    };

    let Some(callback) = callback else {
        return json_string(&FfiErrorJson {
            error: "Invalid scan callback".to_string(),
        });
    };

    crate::cleanup::scan_folders_each_until(Path::new(&path), target, |task| {
        if should_cancel.is_some_and(|should_cancel| should_cancel(user_data)) {
            return false;
        }

        let Ok(path) = CString::new(task.path.display().to_string()) else {
            return true;
        };
        let Ok(target) = CString::new(format!("{:?}", task.target)) else {
            return true;
        };
        callback(path.as_ptr(), target.as_ptr(), task.size_bytes, user_data);

        !should_cancel.is_some_and(|should_cancel| should_cancel(user_data))
    });

    json_string(&CleanupStreamDoneJson { done: true })
}

#[no_mangle]
pub extern "C" fn git_ai_get_diff(path: *const c_char) -> *mut c_char {
    setup_env_path();
    if path.is_null() {
        return CString::new("").unwrap().into_raw();
    }
    let c_str = unsafe { CStr::from_ptr(path) };
    let file_path = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return CString::new("").unwrap().into_raw(),
    };

    let is_untracked = std::fs::metadata(file_path)
        .map(|m| m.is_file())
        .unwrap_or(false);

    let diff_output = if is_untracked && crate::git::status::get_diff_head(file_path).is_none() {
        std::fs::read_to_string(file_path).unwrap_or_default()
    } else {
        let mut out = crate::git::status::get_diff_head(file_path);
        if out.is_none() {
            out = crate::git::status::get_diff_unstaged(file_path);
        }
        if out.is_none() {
            out = crate::git::status::get_diff_staged(file_path);
        }
        out.unwrap_or_default()
    };

    CString::new(diff_output).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn git_ai_stage_file(path: *const c_char) {
    setup_env_path();
    if path.is_null() {
        return;
    }
    let c_str = unsafe { CStr::from_ptr(path) };
    if let Ok(file_path) = c_str.to_str() {
        let _ = crate::git::status::stage_file(file_path);
    }
}

#[no_mangle]
pub extern "C" fn git_ai_unstage_file(path: *const c_char) {
    setup_env_path();
    if path.is_null() {
        return;
    }
    let c_str = unsafe { CStr::from_ptr(path) };
    if let Ok(file_path) = c_str.to_str() {
        let _ = crate::git::status::unstage_file(file_path);
    }
}

#[no_mangle]
pub extern "C" fn git_ai_generate_commit_message(diff: *const c_char) -> *mut c_char {
    setup_env_path();
    if diff.is_null() {
        return CString::new("").unwrap().into_raw();
    }
    let c_str = unsafe { CStr::from_ptr(diff) };
    let full_diff = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return CString::new("").unwrap().into_raw(),
    };

    let ai_lang = crate::helper::Helper::get_ai_language_name();
    let prompt = format!(
        "{} {}.\n\nDiff:\n\n{}",
        crate::constant::Constant::PROMPT_EXPERT,
        ai_lang,
        full_diff
    );

    let mut cmd = Command::new("kilo");
    cmd.args(["run", "--pure", "--auto"]);

    let model_to_use = if let Ok(output) = Command::new("git")
        .args(["config", "--global", "--get", "git-ai.kilo-model"])
        .output()
    {
        String::from_utf8_lossy(&output.stdout).trim().to_string()
    } else {
        String::new()
    };

    if !model_to_use.is_empty() {
        cmd.args(["--model", &model_to_use]);
    }

    cmd.arg(prompt);

    let res = match cmd.output() {
        Ok(output) => {
            if output.status.success() {
                String::from_utf8_lossy(&output.stdout).trim().to_string()
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                format!("Error: {}", stderr)
            }
        }
        Err(e) => format!("Error finding kilo: {}", e),
    };

    CString::new(res).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn git_ai_commit(message: *const c_char) -> *mut c_char {
    setup_env_path();
    if message.is_null() {
        return CString::new("Empty message").unwrap().into_raw();
    }
    let c_str = unsafe { CStr::from_ptr(message) };
    let msg = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return CString::new("Invalid UTF-8 message").unwrap().into_raw(),
    };

    let res = match crate::git::commit::commit(msg) {
        Ok(_) => "Success".to_string(),
        Err(e) => format!("Error: {}", e),
    };

    CString::new(res).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn git_ai_push() -> *mut c_char {
    setup_env_path();
    let res = match crate::git::remote::git_push() {
        Ok(_) => "Success".to_string(),
        Err(e) => format!("Error: {}", e),
    };
    CString::new(res).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn git_ai_free_string(s: *mut c_char) {
    if !s.is_null() {
        unsafe {
            let _ = CString::from_raw(s);
        }
    }
}

#[no_mangle]
pub extern "C" fn git_ai_set_current_dir(path: *const c_char) -> bool {
    setup_env_path();
    if path.is_null() {
        return false;
    }
    let c_str = unsafe { CStr::from_ptr(path) };
    let new_path = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return false,
    };

    if std::env::set_current_dir(new_path).is_err() {
        return false;
    }

    let output = Command::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .output();

    match output {
        Ok(out) => out.status.success(),
        Err(_) => false,
    }
}

#[no_mangle]
pub extern "C" fn git_ai_get_global_diff() -> *mut c_char {
    setup_env_path();
    let output = Command::new("git").args(["diff", "HEAD"]).output();
    let res = match output {
        Ok(out) => {
            if out.status.success() {
                String::from_utf8_lossy(&out.stdout).to_string()
            } else {
                String::from_utf8_lossy(&out.stderr).to_string()
            }
        }
        Err(e) => format!("Error running git diff: {}", e),
    };
    CString::new(res).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn git_ai_get_ai_prompt() -> *mut c_char {
    setup_env_path();
    let output = Command::new("git").args(["diff", "HEAD"]).output();
    let diff_str = match output {
        Ok(out) if out.status.success() => String::from_utf8_lossy(&out.stdout).to_string(),
        _ => String::new(),
    };
    let ai_lang = crate::helper::Helper::get_ai_language_name();
    let prompt = format!(
        "{} {}.\n\nDiff:\n\n{}",
        crate::constant::Constant::PROMPT_EXPERT,
        ai_lang,
        diff_str
    );
    CString::new(prompt).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn git_ai_get_workspace_history() -> *mut c_char {
    setup_env_path();
    let history = crate::helper::Helper::load_history_file("workspace_history.txt");
    let json = serde_json::to_string(&history).unwrap_or_else(|_| "[]".to_string());
    CString::new(json).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn git_ai_add_to_workspace_history(path: *const c_char) {
    setup_env_path();
    if path.is_null() {
        return;
    }
    let c_str = unsafe { CStr::from_ptr(path) };
    let folder_path = match c_str.to_str() {
        Ok(s) => s.trim().to_string(),
        Err(_) => return,
    };
    if folder_path.is_empty() {
        return;
    }
    let mut history = crate::helper::Helper::load_history_file("workspace_history.txt");
    history.retain(|p| p != &folder_path);
    history.insert(0, folder_path);
    history.truncate(10);
    let _ = crate::helper::Helper::save_history_file("workspace_history.txt", &history);
}

#[no_mangle]
pub extern "C" fn git_ai_remove_from_workspace_history(path: *const c_char) {
    setup_env_path();
    if path.is_null() {
        return;
    }
    let c_str = unsafe { CStr::from_ptr(path) };
    let folder_path = match c_str.to_str() {
        Ok(s) => s.trim().to_string(),
        Err(_) => return,
    };
    if folder_path.is_empty() {
        return;
    }
    let mut history = crate::helper::Helper::load_history_file("workspace_history.txt");
    history.retain(|p| p != &folder_path);
    let _ = crate::helper::Helper::save_history_file("workspace_history.txt", &history);
}

#[no_mangle]
pub extern "C" fn get_ai_test() -> *mut c_char {
    let message = "Calling";
    let c_str = CString::new(message).unwrap();
    c_str.into_raw()
}
