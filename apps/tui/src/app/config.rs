use std::process::Command;

pub(super) fn load_bool_config(key: &str, default: bool) -> bool {
    if let Ok(output) = Command::new("git")
        .args(["config", "--global", "--get", key])
        .output()
    {
        let text = String::from_utf8_lossy(&output.stdout)
            .trim()
            .to_lowercase();
        if text.is_empty() {
            default
        } else if default {
            text != "false"
        } else {
            text == "true"
        }
    } else {
        default
    }
}

pub(super) fn load_editor() -> String {
    if let Ok(output) = Command::new("git")
        .args(["config", "--global", "--get", "git-ai.editor"])
        .output()
    {
        let text = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if text.is_empty() {
            "code".to_string()
        } else {
            text
        }
    } else {
        "code".to_string()
    }
}

pub(super) fn selected_editor_index(editor: &str) -> usize {
    match editor {
        "code" => 0,
        "cursor" => 1,
        "zed" => 2,
        "subl" => 3,
        _ => 4,
    }
}
