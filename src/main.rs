mod constant;
mod dashboard;
mod helper;
mod ui;

use crate::helper::Helper;
use crate::ui::logger;
use crate::ui::spinner::with_spinner;
use anyhow::{Context, Result};
use arboard::Clipboard;
use clap::{Parser, Subcommand};
use console::{style, Key, Term};
use std::env;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};

const MARKERS: &[&str] = &[
    "# ULTIMATE GIT-AI WORKFLOW",
    "alias git-copydiff",
    "alias git-go",
    "alias git-ai-uninstall",
    "alias git-ai=",
    "function git-copydiff",
    "function git-go",
    "function git-ai-uninstall",
    "function git-ai",
];

// =========================================================================
// LOCALIZATION (I18N)
// =========================================================================

#[derive(serde::Deserialize)]
pub struct Locales {
    pub help_title: String,
    pub help_desc: String,
    pub diff_success: String,
    pub error_prefix: String,
    pub press_enter: String,
    pub no_changes: String,
    pub prompt_expert: String,
    pub status_clean: String,
    pub status_pending: String,
    pub status_fail: String,
    pub preview_heading: String,
    pub commit_content: String,
    pub confirm_deploy: String,
    pub pushing: String,
    pub push_success: String,
    pub deploy_cancel: String,
    pub reset_heading: String,
    pub reset_success: String,
    pub reset_info: String,
    pub reset_clean: String,
    pub confirm_remove_alias: String,
    pub keep_alias: String,
    pub lang_set: String,
    pub lang_auto: String,
    pub lang_invalid: String,
    pub cmd_help_diff: String,
    pub cmd_help_go: String,
    pub cmd_help_un: String,
    pub cmd_help_base: String,
}

impl Locales {
    pub fn new(lang: &str) -> Self {
        let yaml_content = if lang == "Vietnamese" || lang == "vi" {
            include_str!("../locales/vi.yml")
        } else {
            include_str!("../locales/en.yml")
        };

        serde_yaml::from_str(yaml_content)
            .unwrap_or_else(|e| panic!("Trục trặc khi parse file ngôn ngữ '{}': {}", lang, e))
    }
}

// =========================================================================
// CLAP CLI CONFIGURATION
// =========================================================================

#[derive(Parser)]
#[command(
    name = "git-ai",
    version,
    about = "🤖 ULTIMATE GIT-AI CLI\nA tool to help you write Git Commits using AI rapidly."
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(visible_alias = "d")]
    Diff,

    #[command(visible_alias = "g")]
    Go,

    #[command(visible_alias = "l")]
    Lang { lang: String },

    #[command(visible_alias = "i")]
    Install,

    #[command(visible_alias = "u")]
    Uninstall,

    #[command(visible_alias = "r")]
    Reset,

    #[command(visible_alias = "t")]
    Test,
}

// =========================================================================
// MAIN & ROUTING
// =========================================================================

fn main() -> Result<()> {
    let cli = Cli::parse();
    let locales = Helper::get_locales();

    if let Err(e) = run(&cli, &locales) {
        logger::error(&format!("{} {}", locales.error_prefix, e));
    }
    Ok(())
}

fn run(cli: &Cli, locales: &Locales) -> Result<()> {
    match &cli.command {
        Some(Commands::Diff) => {
            let msg = handle_diff(locales)?;
            logger::system(&msg);
        }
        Some(Commands::Go) => handle_go(locales)?,
        Some(Commands::Lang { lang }) => {
            let msg = handle_lang(lang, locales)?;
            println!("{}", msg);
        }
        Some(Commands::Install) => handle_install()?,
        Some(Commands::Uninstall) => handle_uninstall()?,
        Some(Commands::Reset) => handle_restore(locales)?,
        Some(Commands::Test) => handle_test()?,
        None => {
            dashboard::run_dashboard()?;
        }
    }
    Ok(())
}

// =========================================================================
// COMMAND HANDLERS
// =========================================================================

pub fn handle_lang(lang: &str, locales: &Locales) -> Result<String> {
    match lang {
        "vi" | "en" => {
            Command::new("git")
                .args(["config", "--global", "git-ai.lang", lang])
                .status()?;
            let new_locales = Locales::new(lang);
            Ok(format!("{} {}", new_locales.lang_set, lang))
        }
        "auto" => {
            let _ = Command::new("git")
                .args(["config", "--global", "--unset", "git-ai.lang"])
                .status();
            let resolved_lang = Helper::get_ai_language();
            let new_locales = Locales::new(&resolved_lang);
            Ok(new_locales.lang_auto)
        }
        _ => Ok(locales.lang_invalid.clone()),
    }
}

fn handle_uninstall() -> Result<()> {
    logger::warn("🗑️  Uninstalling configuration from system...");

    #[cfg(target_family = "unix")]
    {
        let profile = get_active_unix_profile();
        if clean_profile_file(&profile)? {
            logger::success(&format!("Successfully removed from: {}", profile.display()));
            logger::note(&format!(
                "👉 Please restart Terminal or run 'source {}' to apply.",
                profile.display()
            ));
        } else {
            logger::info("No git-ai configuration found to remove.");
        }
    }

    #[cfg(target_os = "windows")]
    {
        let profile = get_windows_profile()?;
        if clean_profile_file(&profile)? {
            logger::success("Removed functions from PowerShell Profile!");
            logger::note("👉 Please restart PowerShell to apply changes.");
        } else {
            logger::info("No PowerShell Profile configuration found to remove.");
        }
    }
    Ok(())
}

fn handle_install() -> Result<()> {
    let exe_path = env::current_exe()?;
    let exe_str = exe_path.to_string_lossy();

    #[cfg(target_family = "unix")]
    {
        let target_profile = get_active_unix_profile();

        if target_profile.exists() {
            let content = with_spinner(
                "Auto-configuring system...".to_string(),
                || -> anyhow::Result<String> {
                    let raw = fs::read_to_string(&target_profile)?;
                    Ok(raw)
                },
            )?;
            if content.contains("# ULTIMATE GIT-AI WORKFLOW") {
                logger::path(
                    "⚠️  Configuration already exists in:",
                    &target_profile.display().to_string(),
                );

                let prompt = "🔄 Overwrite existing configuration? (y/N): ";
                if ask_confirm_default_no(prompt)? {
                    clean_profile_file(&target_profile)?;
                    logger::info("🧹 Cleaned old configuration.");
                } else {
                    logger::success("Install cancelled. Kept existing config.");
                    return Ok(());
                }
            }
        }

        let alias_lines = format!(
                "\n# ULTIMATE GIT-AI WORKFLOW\nalias git-copydiff=\"'{}' diff\"\nalias git-go=\"'{}' go\"\nalias git-ai-uninstall=\"'{}' uninstall\"\nalias git-ai=\"'{}'\"\n",
                exe_str, exe_str, exe_str, exe_str
            );

        append_to_file(&target_profile, &alias_lines)?;

        logger::success("Configuration successful! Added aliases:");
        let dummy_locales = Locales::new("English");
        print_commands_help(&dummy_locales);
        logger::note(&format!(
            "\n👉 Please run command: source {}",
            target_profile.display()
        ));
    }

    #[cfg(target_os = "windows")]
    {
        let profile_path = get_windows_profile()?;

        if profile_path.exists() {
            let content = fs::read_to_string(&profile_path)?;
            if content.contains("# ULTIMATE GIT-AI WORKFLOW") {
                logger::path(
                    "⚠️  Configuration already exists in:",
                    &profile_path.display().to_string(),
                );

                let prompt = "🔄 Overwrite existing configuration? (y/N): ";
                if ask_confirm_default_no(prompt)? {
                    clean_profile_file(&profile_path)?;
                    logger::info("🧹 Cleaned old configuration.");
                } else {
                    logger::success("Install cancelled. Kept existing config.");
                    return Ok(());
                }
            }
        }

        if let Some(parent) = profile_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let func_lines = format!(
                "\n# ULTIMATE GIT-AI WORKFLOW\nfunction git-copydiff {{ & \"{}\" diff }}\nfunction git-go {{ & \"{}\" go }}\nfunction git-ai-uninstall {{ & \"{}\" uninstall }}\nfunction git-ai {{ & \"{}\" }}\n",
                exe_str, exe_str, exe_str, exe_str
            );

        append_to_file(&profile_path, &func_lines)?;

        logger::success("Configuration successful! Added aliases:");
        let dummy_locales = Locales::new("English");
        print_commands_help(&dummy_locales);
        logger::note("\n👉 Please restart PowerShell to apply new commands.");
    }
    Ok(())
}

#[allow(dead_code)]
fn handle_test() -> Result<()> {
    print!("Testing...");
    Ok(())
}

pub fn handle_diff(locales: &Locales) -> Result<String> {
    let output = Command::new("git").args(["diff"]).output()?;
    let diff_str = String::from_utf8_lossy(&output.stdout);

    if diff_str.trim().is_empty() {
        return Ok(locales.no_changes.clone());
    }

    let ai_lang = Helper::get_ai_language();

    let prompt = format!(
        "{} {}.\n\nDiff:\n\n{}",
        locales.prompt_expert, ai_lang, diff_str
    );

    let mut clipboard = Clipboard::new()?;
    clipboard.set_text(prompt)?;

    Ok(locales.diff_success.clone())
}

pub fn handle_check_status(locales: &Locales) -> Result<bool> {
    let output = Command::new("git").args(["status", "-s"]).output()?;
    let status_text = String::from_utf8_lossy(&output.stdout);
    if status_text.trim().is_empty() {
        logger::info(&locales.status_clean);
        return Ok(false);
    }
    logger::info(&locales.status_pending);
    logger::text(&status_text);
    Ok(true)
}

pub fn handle_go(locales: &Locales) -> Result<()> {
    logger::heading(&locales.preview_heading);

    if !handle_check_status(locales)? {
        return Ok(());
    }

    let mut clipboard = Clipboard::new()?;
    let commit_msg = clipboard.get_text().unwrap_or_default();

    logger::system(&locales.commit_content);
    logger::green_text(&commit_msg);
    logger::text("");

    if ask_confirm(&locales.confirm_deploy)? {
        logger::heading(&locales.pushing);

        if !Command::new("git").args(["add", "."]).status()?.success() {
            return Ok(());
        }
        if !Command::new("git")
            .args(["commit", "-m", &commit_msg])
            .status()?
            .success()
        {
            return Ok(());
        }

        if Command::new("git").args(["push"]).status()?.success() {
            logger::success(&locales.push_success);
        }
    } else {
        logger::error(&locales.deploy_cancel);
    }
    Ok(())
}

pub fn handle_restore(locales: &Locales) -> Result<()> {
    logger::heading(&locales.reset_heading);
    let status = Command::new("git")
        .args(["config", "--global", "--remove-section", "git-ai"])
        .status();

    match status {
        Ok(s) if s.success() => {
            logger::success(&locales.reset_success);
            logger::info(&locales.reset_info);
        }
        _ => {
            logger::info(&locales.reset_clean);
        }
    }

    if ask_confirm_default_no(&locales.confirm_remove_alias)? {
        handle_uninstall()?;
    } else {
        logger::success(&locales.keep_alias);
    }
    Ok(())
}

// =========================================================================
// HELPER FUNCTIONS
// =========================================================================

// pub fn get_ai_language() -> String {
//     if let Ok(output) = Command::new("git")
//         .args(["config", "--global", "--get", "git-ai.lang"])
//         .output()
//     {
//         let stdout = String::from_utf8_lossy(&output.stdout)
//             .trim()
//             .to_lowercase();
//         if stdout == "vi" || stdout == "en" {
//             return stdout;
//         }
//     }
//     let locale = get_locale().unwrap_or_else(|| String::from("en-US"));
//     if locale.starts_with("vi") {
//         "vi".to_string()
//     } else {
//         "en".to_string()
//     }
// }

#[allow(dead_code)]
fn call_external_app(cmd: &str, args: &[&str], input: &str) -> Result<String> {
    let mut child = Command::new(cmd)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .with_context(|| format!("Application not found: {}", cmd))?;

    {
        let mut stdin = child.stdin.take().context("Could not capture stdin")?;
        stdin.write_all(input.as_bytes())?;
    }

    let output = child.wait_with_output()?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        let err = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Application {} reported error: {}", cmd, err);
    }
}

fn ask_confirm_default_no(prompt: &str) -> Result<bool> {
    print!("{}", style(prompt).yellow().bold());
    io::stdout().flush()?;

    let term = Term::stdout();
    loop {
        match term.read_key()? {
            Key::Char('y') | Key::Char('Y') => {
                logger::text("");
                return Ok(true);
            }
            Key::Enter | Key::Char('n') | Key::Char('N') | Key::Escape => {
                logger::text("");
                return Ok(false);
            }
            _ => {}
        }
    }
}

fn ask_confirm(prompt: &str) -> Result<bool> {
    print!("{}", style(prompt).yellow().bold());
    io::stdout().flush()?;

    let term = Term::stdout();
    loop {
        match term.read_key()? {
            Key::Enter | Key::Char('y') | Key::Char('Y') => {
                logger::text("");
                return Ok(true);
            }
            Key::Char('n') | Key::Char('N') | Key::Escape => {
                logger::text("");
                return Ok(false);
            }
            _ => {}
        }
    }
}

#[cfg(target_family = "unix")]
fn get_active_unix_profile() -> PathBuf {
    let home = env::var("HOME").unwrap_or_else(|_| "~".to_string());
    let shell = env::var("SHELL").unwrap_or_default().to_lowercase();
    if shell.contains("zsh") {
        PathBuf::from(format!("{}/.zshrc", home))
    } else if shell.contains("bash") {
        let bashrc = PathBuf::from(format!("{}/.bashrc", home));
        if bashrc.exists() {
            bashrc
        } else {
            PathBuf::from(format!("{}/.bash_profile", home))
        }
    } else if shell.contains("fish") {
        PathBuf::from(format!("{}/.config/fish/config.fish", home))
    } else {
        let zsh_path = PathBuf::from(format!("{}/.zshrc", home));
        if zsh_path.exists() {
            zsh_path
        } else {
            PathBuf::from(format!("{}/.bashrc", home))
        }
    }
}

#[cfg(target_os = "windows")]
fn get_windows_profile() -> Result<PathBuf> {
    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command", "Write-Output $PROFILE"])
        .output()?;
    let path_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
    Ok(PathBuf::from(path_str))
}

fn clean_profile_file(path: &PathBuf) -> Result<bool> {
    if !path.exists() {
        return Ok(false);
    }

    let content = fs::read_to_string(path)?;
    let lines: Vec<&str> = content.lines().collect();
    let mut filtered_lines = Vec::new();
    let mut changed = false;

    for line in lines {
        if MARKERS.iter().any(|&m| line.contains(m)) {
            changed = true;
        } else {
            filtered_lines.push(line);
        }
    }

    if changed {
        let new_content = filtered_lines.join("\n") + "\n";
        fs::write(path, new_content)?;
        Ok(true)
    } else {
        Ok(false)
    }
}

fn append_to_file(path: &PathBuf, content: &str) -> Result<()> {
    let mut file = OpenOptions::new().append(true).create(true).open(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn print_commands_help(locales: &Locales) {
    logger::info(&locales.cmd_help_diff);
    logger::info(&locales.cmd_help_go);
    logger::info(&locales.cmd_help_un);
    logger::info(&locales.cmd_help_base);
}
