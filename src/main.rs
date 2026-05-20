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
// CẤU HÌNH CLAP CLI
// =========================================================================

#[derive(Parser)]
#[command(
    name = "git-ai",
    version,
    about = "🤖 ULTIMATE GIT-AI CLI\nCông cụ hỗ trợ viết Git Commit bằng AI nhanh chóng.",
    after_help = "💡 QUY TRÌNH SỬ DỤNG CHUẨN:\n  1. Sửa code xong, gõ `git-ai diff` để quét thay đổi.\n  2. Dán (Ctrl+V) nội dung vào ChatGPT/Claude/Gemini.\n  3. Copy lại câu trả lời (commit message) của AI.\n  4. Gõ `git-ai go` để tool tự động đóng gói và đẩy code lên git.\n"
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
    Lang {
        lang: String,
    },

    #[command(visible_alias = "i")]
    Install,

    #[command(visible_alias = "u")]
    Uninstall,

    #[command(visible_alias = "r")]
    Reset,

    Test,
}

// =========================================================================
// HÀM MAIN VÀ ĐIỀU HƯỚNG
// =========================================================================

fn main() -> Result<()> {
    let cli = Cli::parse();

    if let Err(e) = run(&cli) {
        logger::error(&format!("Đã xảy ra lỗi: {}", e));
    }
    Ok(())
}

fn run(cli: &Cli) -> Result<()> {
    match &cli.command {
        Some(Commands::Diff) => {
            let msg = handle_diff()?;
            logger::system(&msg);
        }
        Some(Commands::Go) => handle_go()?,
        Some(Commands::Lang { lang }) => {
            let msg = handle_lang(lang)?;
            println!("{}", msg);
        }
        Some(Commands::Install) => handle_install()?,
        Some(Commands::Uninstall) => handle_uninstall()?,
        Some(Commands::Reset) => handle_restore()?,
        Some(Commands::Test) => handle_test()?,
        None => {
            dashboard::run_dashboard()?;
        }
    }
    Ok(())
}

// =========================================================================
// CÁC HÀM XỬ LÝ LỆNH TƯƠNG ỨNG
// =========================================================================

fn handle_lang(lang: &str) -> Result<String> {
    match lang {
        "vi" | "en" => {
            Command::new("git")
                .args(["config", "--global", "git-ai.lang", lang])
                .status()?;
            Ok(format!("✅ Đã thiết lập ngôn ngữ: {}", lang))
        }
        "auto" => {
            let _ = Command::new("git")
                .args(["config", "--global", "--unset", "git-ai.lang"])
                .status();
            Ok("✅ Đã về chế độ tự động.".to_string())
        }
        _ => Ok("❌ Lệnh không hợp lệ.".to_string()),
    }
}
fn handle_uninstall() -> Result<()> {
    logger::warn("🗑️  Đang tiến hành gỡ bỏ cấu hình khỏi hệ thống...");

    #[cfg(target_family = "unix")]
    {
        let profile = get_active_unix_profile();
        if clean_profile_file(&profile)? {
            logger::success(&format!("Đã gỡ bỏ thành công khỏi: {}", profile.display()));
            logger::note(&format!(
                "👉 Vui lòng khởi động lại Terminal hoặc chạy 'source {}' để áp dụng.",
                profile.display()
            ));
        } else {
            logger::info("Không tìm thấy cấu hình git-ai nào để gỡ.");
        }
    }

    #[cfg(target_os = "windows")]
    {
        let profile = get_windows_profile()?;
        if clean_profile_file(&profile)? {
            logger::success("Đã gỡ bỏ các chức năng khỏi PowerShell Profile!");
            logger::note("👉 Vui lòng khởi động lại PowerShell để áp dụng thay đổi.");
        } else {
            logger::info("Không tìm thấy cấu hình PowerShell Profile để gỡ.");
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
                "Đang tiến hành tự động cấu hình hệ thống...".to_string(),
                || -> anyhow::Result<String> {
                    let raw = fs::read_to_string(&target_profile)?;
                    Ok(raw)
                },
            )?;
            if content.contains("# ULTIMATE GIT-AI WORKFLOW") {
                logger::path(
                    "⚠️  Cấu hình đã tồn tại sẵn trong:",
                    &target_profile.display().to_string(),
                );

                let prompt = "🔄 Bạn có muốn ghi đè (xóa cũ, cài mới) cấu hình này không? (y/N): ";
                if ask_confirm_default_no(prompt)? {
                    clean_profile_file(&target_profile)?;
                    logger::info("🧹 Đã dọn dẹp cấu hình cũ.");
                } else {
                    logger::success("Đã hủy quá trình cài đặt. Giữ nguyên cấu hình hiện tại.");
                    return Ok(());
                }
            }
        }

        let alias_lines = format!(
            "\n# ULTIMATE GIT-AI WORKFLOW\nalias git-copydiff=\"'{}' diff\"\nalias git-go=\"'{}' go\"\nalias git-ai-uninstall=\"'{}' uninstall\"\nalias git-ai=\"'{}'\"\n",
            exe_str, exe_str, exe_str, exe_str
        );

        append_to_file(&target_profile, &alias_lines)?;

        logger::success("Cấu hình thành công! Đã thêm các phím tắt:");
        print_commands_help();
        logger::note(&format!(
            "\n👉 Vui lòng chạy lệnh: source {}",
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
                    "⚠️  Cấu hình đã tồn tại sẵn trong:",
                    &profile_path.display().to_string(),
                );

                let prompt = "🔄 Bạn có muốn ghi đè (xóa cũ, cài mới) cấu hình này không? (y/N): ";
                if ask_confirm_default_no(prompt)? {
                    clean_profile_file(&profile_path)?;
                    logger::info("🧹 Đã dọn dẹp cấu hình cũ.");
                } else {
                    logger::success("Đã hủy quá trình cài đặt. Giữ nguyên cấu hình hiện tại.");
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

        logger::success("Cấu hình thành công! Đã thêm các phím tắt:");
        print_commands_help();
        logger::note("\n👉 Vui lòng khởi động lại PowerShell để kích hoạt các lệnh mới.");
    }
    Ok(())
}

#[allow(dead_code)]
fn handle_test() -> Result<()> {
    handle_check_status()?;
    Ok(())
}

pub fn handle_diff() -> Result<String> {
    let output = Command::new("git").args(["diff"]).output()?;
    let diff_str = String::from_utf8_lossy(&output.stdout);

    if diff_str.trim().is_empty() {
        return Ok(
            "⚠️ [HỆ THỐNG]: Không phát hiện thay đổi nào. Đã hủy chụp snapshot!".to_string(),
        );
    }

    let ai_lang = get_ai_language();

    let prompt = format!(
        "Act as an expert developer. Output ONLY the raw commit message for the diff below. Rules: 1. Subject line: Conventional Commits, < 50 chars. 2. Blank line. 3. Body: 1-2 extremely short sentences explaining WHAT and WHY. Be direct, no fluff. STRICTLY NO markdown formatting (no ```), NO preamble, NO greetings. Write the commit message in {}.\n\nDiff:\n\n{}",
        ai_lang, diff_str
    );

    let mut clipboard = Clipboard::new()?;
    clipboard.set_text(prompt)?;

    Ok(format!(
        "✨ Đã chụp snapshot (Yêu cầu AI viết bằng {}). Dán vào AI ngay! 🤖",
        ai_lang
    ))
}

pub fn handle_check_status() -> Result<bool> {
    let output = Command::new("git").args(["status", "-s"]).output()?;
    let status_text = String::from_utf8_lossy(&output.stdout);
    if status_text.trim().is_empty() {
        logger::info("✅ Thư mục hiện tại đang sạch (không có thay đổi).");
        return Ok(false);
    }
    logger::info("📂 Các thay đổi đang chờ:");
    logger::text(&status_text);
    Ok(true)
}

pub fn handle_go() -> Result<()> {
    logger::heading("📂 [XEM TRƯỚC THAY ĐỔI]");

    if !handle_check_status()? {
        return Ok(());
    }

    let mut clipboard = Clipboard::new()?;
    let commit_msg = clipboard.get_text().unwrap_or_default();

    logger::system("💬 [NỘI DUNG COMMIT TỪ AI]");
    logger::green_text(&commit_msg);
    logger::text("");

    if ask_confirm("🚀 Xác nhận deploy (commit & push)? (Y/n): ")? {
        logger::heading("⚡ Đang tiến hành đẩy code...");

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
            logger::success("Hoàn tất. Code đã được đẩy lên mây thành công! ☁️");
        }
    } else {
        logger::error("Đã hủy quá trình deploy.");
    }
    Ok(())
}

fn handle_restore() -> Result<()> {
    logger::heading("🔄 Đang khôi phục cài đặt gốc...");
    let status = Command::new("git")
        .args(["config", "--global", "--remove-section", "git-ai"])
        .status();

    match status {
        Ok(s) if s.success() => {
            logger::success("Đã xóa toàn bộ cấu hình của git-ai khỏi hệ thống Git.");
            logger::info("👉 Công cụ đã được đưa về trạng thái mặc định ban đầu.");
        }
        _ => {
            logger::info("Máy tính của bạn rất sạch sẽ. Không có cấu hình git-ai nào cần xóa!");
        }
    }

    let prompt = "🗑️  Bạn có muốn gỡ bỏ luôn các phím tắt (alias) khỏi Terminal không? (y/N): ";
    if ask_confirm_default_no(prompt)? {
        handle_uninstall()?;
    } else {
        logger::success("Đã bỏ qua. Giữ lại các phím tắt trong Terminal.");
    }
    Ok(())
}

// =========================================================================
// CÁC HÀM TIỆN ÍCH (HELPER FUNCTIONS)
// =========================================================================

fn get_ai_language() -> String {
    if let Ok(output) = Command::new("git")
        .args(["config", "--global", "--get", "git-ai.lang"])
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        match stdout.trim().to_lowercase().as_str() {
            "vi" => return "Vietnamese".to_string(),
            "en" => return "English".to_string(),
            _ => {}
        }
    }

    let locale = get_locale().unwrap_or_else(|| String::from("en-US"));
    if locale.starts_with("vi") {
        "Vietnamese".to_string()
    } else if locale.starts_with("ja") {
        "Japanese".to_string()
    } else if locale.starts_with("zh") {
        "Chinese".to_string()
    } else if locale.starts_with("fr") {
        "French".to_string()
    } else if locale.starts_with("es") {
        "Spanish".to_string()
    } else if locale.starts_with("de") {
        "German".to_string()
    } else if locale.starts_with("ko") {
        "Korean".to_string()
    } else {
        "English".to_string()
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
        .with_context(|| format!("Không tìm thấy ứng dụng: {}", cmd))?;

    {
        let mut stdin = child.stdin.take().context("Không thể lấy stdin")?;
        stdin.write_all(input.as_bytes())?;
    }

    let output = child.wait_with_output()?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        let err = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Ứng dụng {} báo lỗi: {}", cmd, err);
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

fn print_commands_help() {
    logger::info("  -> git-copydiff     : Chụp snapshot code diff");
    logger::info("  -> git-go           : Đóng gói và đẩy code lên git");
    logger::info("  -> git-ai-uninstall : Gỡ cài đặt bộ tool này");
    logger::info("  -> git-ai           : Lệnh gốc (ví dụ: git-ai help)");
}
