use serde::Serialize;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
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

    let diff_output = if is_untracked && !crate::git::status::get_diff_head(file_path).is_some() {
        if let Ok(content) = std::fs::read_to_string(file_path) {
            content
        } else {
            String::new()
        }
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
    let mut history = Vec::new();
    if let Ok(output) = Command::new("git")
        .args(["config", "--global", "--get", "git-ai.workspace-history"])
        .output()
    {
        let text = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !text.is_empty() {
            for entry in text.split('|') {
                let trimmed = entry.trim().to_string();
                if !trimmed.is_empty() {
                    history.push(trimmed);
                }
            }
        }
    }
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
    let mut history = Vec::new();
    if let Ok(output) = Command::new("git")
        .args(["config", "--global", "--get", "git-ai.workspace-history"])
        .output()
    {
        let text = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !text.is_empty() {
            for entry in text.split('|') {
                let trimmed = entry.trim().to_string();
                if !trimmed.is_empty() {
                    history.push(trimmed);
                }
            }
        }
    }
    history.retain(|p| p != &folder_path);
    history.insert(0, folder_path);
    history.truncate(10);
    let value = history.join("|");
    let _ = Command::new("git")
        .args(["config", "--global", "git-ai.workspace-history", &value])
        .output();
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
    let mut history = Vec::new();
    if let Ok(output) = Command::new("git")
        .args(["config", "--global", "--get", "git-ai.workspace-history"])
        .output()
    {
        let text = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !text.is_empty() {
            for entry in text.split('|') {
                let trimmed = entry.trim().to_string();
                if !trimmed.is_empty() {
                    history.push(trimmed);
                }
            }
        }
    }
    history.retain(|p| p != &folder_path);
    let value = history.join("|");
    let _ = Command::new("git")
        .args(["config", "--global", "git-ai.workspace-history", &value])
        .output();
}

#[no_mangle]
pub extern "C" fn get_ai_test() -> *mut c_char {
    let message = "Calling";
    let c_str = CString::new(message).unwrap();
    c_str.into_raw()
}
