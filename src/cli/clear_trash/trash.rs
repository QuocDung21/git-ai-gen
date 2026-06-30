use anyhow::Result;
use rust_i18n::t;

#[cfg(target_os = "macos")]
pub(super) fn confirm_empty_trash() -> Result<bool> {
    show_macos_confirm_dialog(
        "git-ai-clean",
        &t!("clear_trash_confirm").to_string(),
        "Empty Trash",
        "Cancel",
    )
}

#[cfg(target_os = "macos")]
fn show_macos_confirm_dialog(
    title: &str,
    message: &str,
    confirm_button: &str,
    cancel_button: &str,
) -> Result<bool> {
    use anyhow::Context;
    use std::process::Command;

    let title = escape_applescript_string(title);
    let message = escape_applescript_string(message);
    let confirm_button = escape_applescript_string(confirm_button);
    let cancel_button = escape_applescript_string(cancel_button);

    let script = format!(
        r#"
        display dialog "{}" ¬
        with title "{}" ¬
        buttons {{"{}", "{}"}} ¬
        default button "{}" ¬
        cancel button "{}" ¬
        with icon caution
        "#,
        message, title, cancel_button, confirm_button, cancel_button, cancel_button
    );

    let output = Command::new("osascript")
        .arg("-e")
        .arg(script)
        .output()
        .context("failed to show macOS confirmation dialog")?;

    Ok(output.status.success())
}

#[cfg(target_os = "macos")]
fn escape_applescript_string(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}

#[cfg(target_os = "macos")]
pub fn empty_macos_trash() -> Result<()> {
    use anyhow::{bail, Context};
    use std::process::Command;

    let output = Command::new("osascript")
        .arg("-e")
        .arg(r#"tell application "Finder" to empty trash"#)
        .output()
        .context("failed to run osascript")?;

    if output.status.success() {
        return Ok(());
    }

    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_owned();

    if stderr.contains("is in use") {
        bail!(
            "{}\n{}",
            stderr,
            "Trash contains an item currently in use. Close the related app/process, or use force clean."
        );
    }

    if stderr.is_empty() {
        bail!("{}", t!("clear_trash_failed"));
    }

    bail!("{} {}", t!("clear_trash_failed"), stderr);
}

#[cfg(not(target_os = "macos"))]
pub fn empty_macos_trash() -> Result<()> {
    Ok(())
}
