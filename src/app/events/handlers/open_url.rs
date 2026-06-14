#[cfg(not(target_os = "macos"))]
use enigo::{Direction, Enigo, Key, Keyboard, Settings};
use std::{process::Command, thread, time::Duration};

fn set_clipboard(text: &str) -> Result<(), String> {
    let mut cb = arboard::Clipboard::new().map_err(|e| e.to_string())?;
    cb.set_text(text.to_string()).map_err(|e| e.to_string())
}

fn send_paste_shortcut() -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        native_paste()
    }

    #[cfg(not(target_os = "macos"))]
    {
        enigo_paste()
    }
}

#[cfg(not(target_os = "macos"))]
fn enigo_paste() -> Result<(), String> {
    let mut enigo = Enigo::new(&Settings::default()).map_err(|e| e.to_string())?;

    enigo
        .key(Key::Meta, Direction::Press)
        .map_err(|e| e.to_string())?;
    let paste_result = enigo
        .key(Key::Unicode('v'), Direction::Click)
        .map_err(|e| e.to_string());
    let release_result = enigo
        .key(Key::Meta, Direction::Release)
        .map_err(|e| e.to_string());
a
    release_result?;
    paste_result?;

    Ok(())
}

#[cfg(target_os = "macos")]
fn native_paste() -> Result<(), String> {
    let output = Command::new("osascript")
        .args([
            "-e",
            "delay 0.3",
            "-e",
            "tell application \"System Events\"",
            "-e",
            "key code 9 using {command down}",
            "-e",
            "end tell",
        ])
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).trim().to_string())
    }
}

pub fn open_url(url: &str, text: &str) -> Result<(), String> {
    set_clipboard(text)?;
    open::that(url).map_err(|e| e.to_string())?;
    thread::sleep(Duration::from_millis(2000));
    send_paste_shortcut()
}
