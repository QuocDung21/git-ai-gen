mod dashboard;
mod ui;

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
use sys_locale::get_locale;

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

pub struct Locales {
    pub help_title: &'static str,
    pub help_desc: &'static str,
    pub diff_success: &'static str,
    pub error_prefix: &'static str,
    pub press_enter: &'static str,
    pub no_changes: &'static str,
    pub prompt_expert: &'static str,
    pub status_clean: &'static str,
    pub status_pending: &'static str,
    pub status_fail: &'static str,
    pub preview_heading: &'static str,
    pub commit_content: &'static str,
    pub confirm_deploy: &'static str,
    pub pushing: &'static str,
    pub push_success: &'static str,
    pub deploy_cancel: &'static str,
    pub reset_heading: &'static str,
    pub reset_success: &'static str,
    pub reset_info: &'static str,
    pub reset_clean: &'static str,
    pub confirm_remove_alias: &'static str,
    pub keep_alias: &'static str,
    pub lang_set: &'static str,
    pub lang_auto: &'static str,
    pub lang_invalid: &'static str,
    pub cmd_help_diff: &'static str,
    pub cmd_help_go: &'static str,
    pub cmd_help_un: &'static str,
    pub cmd_help_base: &'static str,
}

impl Locales {
    pub fn new(lang: &str) -> Self {
        if lang == "Vietnamese" || lang == "vi" {
            Self {
                help_title: "🤖 ULTIMATE GIT-AI CLI",
                help_desc: "Công cụ hỗ trợ viết Git Commit bằng AI nhanh chóng.",
                diff_success: "✨ [HỆ THỐNG]: Đã chụp snapshot. Hãy dán vào AI...",
                error_prefix: "❌ Đã xảy ra lỗi:",
                press_enter: "👉 Nhấn Enter để quay lại Dashboard...",
                no_changes: "⚠️ [HỆ THỐNG]: Không phát hiện thay đổi nào. Đã hủy chụp snapshot!",
                prompt_expert: "Act as an expert developer. Output ONLY the raw commit message for the diff below. Rules: 1. Subject line: Conventional Commits, < 50 chars. 2. Blank line. 3. Body: 1-2 extremely short sentences explaining WHAT and WHY. Be direct, no fluff. STRICTLY NO markdown formatting (no ```), NO preamble, NO greetings. Write the commit message in",
                status_clean: "✅ Thư mục hiện tại đang sạch (không có thay đổi).",
                status_pending: "📂 Các thay đổi đang chờ:",
                status_fail: "❌ Không thể đọc trạng thái Git.",
                preview_heading: "📂 [XEM TRƯỚC THAY ĐỔI]",
                commit_content: "💬 [NỘI DUNG COMMIT TỪ AI]",
                confirm_deploy: "🚀 Xác nhận deploy (commit & push)? (Y/n): ",
                pushing: "⚡ Đang tiến hành đẩy code...",
                push_success: "Hoàn tất. Code đã được đẩy lên mây thành công! ☁️",
                deploy_cancel: "Đã hủy quá trình deploy.",
                reset_heading: "🔄 Đang khôi phục cài đặt gốc...",
                reset_success: "Đã xóa toàn bộ cấu hình của git-ai khỏi hệ thống Git.",
                reset_info: "👉 Công cụ đã được đưa về trạng thái mặc định ban đầu.",
                reset_clean: "Máy tính của bạn rất sạch sẽ. Không có cấu hình git-ai nào cần xóa!",
                confirm_remove_alias: "🗑️  Bạn có muốn gỡ bỏ luôn các phím tắt (alias) khỏi Terminal không? (y/N): ",
                keep_alias: "Đã bỏ qua. Giữ lại các phím tắt trong Terminal.",
                lang_set: "✅ Đã thiết lập ngôn ngữ:",
                lang_auto: "✅ Đã về chế độ tự động.",
                lang_invalid: "❌ Lệnh không hợp lệ. Dùng: vi, en, hoặc auto",
                cmd_help_diff: "  -> git-copydiff     : Chụp snapshot code diff",
                cmd_help_go: "  -> git-go           : Đóng gói và đẩy code lên git",
                cmd_help_un: "  -> git-ai-uninstall : Gỡ cài đặt bộ tool này",
                cmd_help_base: "  -> git-ai           : Lệnh gốc (ví dụ: git-ai help)",
            }
        } else {
            Self {
                help_title: "🤖 ULTIMATE GIT-AI CLI",
                help_desc: "A tool to help you write Git Commits using AI rapidly.",
                diff_success: "✨ [SYSTEM]: Snapshot captured. Paste it to AI...",
                error_prefix: "❌ An error occurred:",
                press_enter: "👉 Press Enter to return to Dashboard...",
                no_changes: "⚠️ [SYSTEM]: No changes detected. Snapshot cancelled!",
                prompt_expert: "Act as an expert developer. Output ONLY the raw commit message for the diff below. Rules: 1. Subject line: Conventional Commits, < 50 chars. 2. Blank line. 3. Body: 1-2 extremely short sentences explaining WHAT and WHY. Be direct, no fluff. STRICTLY NO markdown formatting (no ```), NO preamble, NO greetings. Write the commit message in",
                status_clean: "✅ Working tree clean (No changes).",
                status_pending: "📂 Pending changes:",
                status_fail: "❌ Failed to read Git status.",
                preview_heading: "📂 [PREVIEW CHANGES]",
                commit_content: "💬 [COMMIT CONTENT FROM AI]",
                confirm_deploy: "🚀 Confirm deploy (commit & push)? (Y/n): ",
                pushing: "⚡ Pushing code...",
                push_success: "Done. Code pushed to cloud successfully! ☁️",
                deploy_cancel: "Deploy cancelled.",
                reset_heading: "🔄 Restoring factory settings...",
                reset_success: "All git-ai configurations removed from Git system.",
                reset_info: "👉 Tool restored to initial default state.",
                reset_clean: "System clean. No git-ai configurations to remove!",
                confirm_remove_alias: "🗑️  Remove aliases from Terminal? (y/N): ",
                keep_alias: "Skipped. Aliases kept in Terminal.",
                lang_set: "✅ Language set to:",
                lang_auto: "✅ Reverted to auto mode.",
                lang_invalid: "❌ Invalid command. Use: vi, en, or auto",
                cmd_help_diff: "  -> git-copydiff     : Capture code diff snapshot",
                cmd_help_go: "  -> git-go           : Package and push code to git",
                cmd_help_un: "  -> git-ai-uninstall : Uninstall this toolset",
                cmd_help_base: "  -> git-ai           : Base command (e.g., git-ai help)",
            }
        }
    }
}

pub fn get_locales() -> Locales {
    let lang = get_ai_language();
    Locales::new(&lang)
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
    let locales = get_locales();

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
        Some(Commands::Install) => handle_install()?, // Install is generally system level, keeping minimal localization
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
            let resolved_lang = get_ai_language();
            let new_locales = Locales::new(&resolved_lang);
            Ok(new_locales.lang_auto.to_string())
        }
        _ => Ok(locales.lang_invalid.to_string()),
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
        let dummy_locales = Locales::new("English"); // Fallback for install log
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
        let dummy_locales = Locales::new("English"); // Fallback for install log
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
        return Ok(locales.no_changes.to_string());
    }

    let ai_lang = get_ai_language();

    let prompt = format!(
        "{} {}.\n\nDiff:\n\n{}",
        locales.prompt_expert, ai_lang, diff_str
    );

    let mut clipboard = Clipboard::new()?;
    clipboard.set_text(prompt)?;

    Ok(locales.diff_success.to_string())
}

pub fn handle_check_status(locales: &Locales) -> Result<bool> {
    let output = Command::new("git").args(["status", "-s"]).output()?;
    let status_text = String::from_utf8_lossy(&output.stdout);
    if status_text.trim().is_empty() {
        logger::info(locales.status_clean);
        return Ok(false);
    }
    logger::info(locales.status_pending);
    logger::text(&status_text);
    Ok(true)
}

pub fn handle_go(locales: &Locales) -> Result<()> {
    logger::heading(locales.preview_heading);

    if !handle_check_status(locales)? {
        return Ok(());
    }

    let mut clipboard = Clipboard::new()?;
    let commit_msg = clipboard.get_text().unwrap_or_default();

    logger::system(locales.commit_content);
    logger::green_text(&commit_msg);
    logger::text("");

    if ask_confirm(locales.confirm_deploy)? {
        logger::heading(locales.pushing);

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
            logger::success(locales.push_success);
        }
    } else {
        logger::error(locales.deploy_cancel);
    }
    Ok(())
}

pub fn handle_restore(locales: &Locales) -> Result<()> {
    logger::heading(locales.reset_heading);
    let status = Command::new("git")
        .args(["config", "--global", "--remove-section", "git-ai"])
        .status();

    match status {
        Ok(s) if s.success() => {
            logger::success(locales.reset_success);
            logger::info(locales.reset_info);
        }
        _ => {
            logger::info(locales.reset_clean);
        }
    }

    if ask_confirm_default_no(locales.confirm_remove_alias)? {
        handle_uninstall()?;
    } else {
        logger::success(locales.keep_alias);
    }
    Ok(())
}

// =========================================================================
// HELPER FUNCTIONS
// =========================================================================

pub fn get_ai_language() -> String {
    if let Ok(output) = Command::new("git")
        .args(["config", "--global", "--get", "git-ai.lang"])
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout)
            .trim()
            .to_lowercase();
        if stdout == "vi" || stdout == "en" {
            return stdout;
        }
    }
    let locale = get_locale().unwrap_or_else(|| String::from("en-US"));
    if locale.starts_with("vi") {
        "vi".to_string()
    } else {
        "en".to_string()
    }
}

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
    logger::info(locales.cmd_help_diff);
    logger::info(locales.cmd_help_go);
    logger::info(locales.cmd_help_un);
    logger::info(locales.cmd_help_base);
}
