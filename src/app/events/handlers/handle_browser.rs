#[cfg(not(target_os = "macos"))]
use enigo::{Direction, Enigo, Key, Keyboard, Settings};
#[cfg(target_os = "macos")]
use std::process::Command;
use std::{thread, time::Duration};

#[allow(dead_code)]
pub enum BrowserAction<'a> {
    PasteTextAndEnter { text: &'a str },
    ClickElement { selector: &'a str },
}

pub fn handle_browser(url: &str, actions: &[BrowserAction<'_>]) -> Result<(), String> {
    open::that(url).map_err(|e| e.to_string())?;
    thread::sleep(Duration::from_millis(2000));

    for action in actions {
        match action {
            BrowserAction::PasteTextAndEnter { text } => {
                set_clipboard(text)?;
                send_text()?;
            }
            BrowserAction::ClickElement { selector } => {
                click_element(selector)?;
            }
        }
    }

    Ok(())
}

fn set_clipboard(text: &str) -> Result<(), String> {
    let mut cb = arboard::Clipboard::new().map_err(|e| e.to_string())?;
    cb.set_text(text.to_string()).map_err(|e| e.to_string())
}

fn send_text() -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        native_send_text()
    }

    #[cfg(not(target_os = "macos"))]
    {
        enigo_send_text()
    }
}

fn click_element(selector: &str) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        native_click_element(selector)
    }

    #[cfg(not(target_os = "macos"))]
    {
        Err(format!(
            "click element is not implemented on this platform: {}",
            selector
        ))
    }
}

#[cfg(not(target_os = "macos"))]
fn enigo_send_text() -> Result<(), String> {
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

    release_result?;
    paste_result?;
    thread::sleep(Duration::from_millis(300));
    enigo
        .key(Key::Return, Direction::Click)
        .map_err(|e| e.to_string())
}

#[cfg(target_os = "macos")]
fn native_send_text() -> Result<(), String> {
    let output = Command::new("osascript")
        .args([
            "-e",
            "delay 0.3",
            "-e",
            "tell application \"System Events\"",
            "-e",
            "key code 9 using {command down}",
            "-e",
            "delay 0.3",
            "-e",
            "key code 36",
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

#[cfg(target_os = "macos")]
fn native_click_element(selector: &str) -> Result<(), String> {
    let selector_json = serde_json::to_string(selector).map_err(|e| e.to_string())?;
    let js = format!(
        r#"(() => {{
        const el = document.querySelector({});
        if (!el) return "ELEMENT_NOT_FOUND";
        el.scrollIntoView({{ block: "center", inline: "center" }});
        el.dispatchEvent(new MouseEvent("click", {{ bubbles: true, cancelable: true, view: window }}));
        return "OK";
        }})()"#,
        selector_json
    );

    let js_literal = serde_json::to_string(&js).map_err(|e| e.to_string())?;
    let apps = [
        "Google Chrome",
        "Arc",
        "Brave Browser",
        "Microsoft Edge",
        "Safari",
    ];

    let mut last_err = String::new();
    for app_name in apps {
        let script = if app_name == "Safari" {
            format!(
                r#"tell application "{}" to do JavaScript {} in current tab of front window"#,
                app_name, js_literal
            )
        } else {
            format!(
                r#"tell application "{}" to execute javascript {} in active tab of front window"#,
                app_name, js_literal
            )
        };

        let output = Command::new("osascript")
            .args(["-e", &script])
            .output()
            .map_err(|e| e.to_string())?;

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if stdout == "ELEMENT_NOT_FOUND" {
                return Err(format!("element not found: {}", selector));
            }
            return Ok(());
        }

        last_err = String::from_utf8_lossy(&output.stderr).trim().to_string();
    }

    Err(last_err)
}
