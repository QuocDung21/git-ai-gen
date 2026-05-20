use arboard::Clipboard;
use colored::*;
use crossterm::event::{read, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::env;
use std::error::Error;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;

// Sử dụng Result chung để bắt lỗi an toàn, chống crash app
type Result<T> = std::result::Result<T, Box<dyn Error>>;

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

fn main() {
    if let Err(e) = run() {
        eprintln!("\n{} {}", "❌ Đã xảy ra lỗi:".red().bold(), e);
    }
}

// Router điều hướng lệnh
fn run() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let cmd = args.get(1).map(|s| s.as_str()).unwrap_or("");

    match cmd {
        "diff" => handle_diff()?,
        "go" => handle_go()?,
        "install" => handle_install()?,
        "uninstall" => handle_uninstall()?,
        "help" | "--help" | "-h" | "" => handle_help(), // Tự động gọi help khi gõ "git-ai"
        _ => println!(
            "\n{} {}\n",
            "❌ Lệnh không hợp lệ.".red().bold(),
            "Vui lòng gõ 'git-ai help' để xem danh sách lệnh.".yellow()
        ),
    }
    Ok(())
}

// =========================================================================
// MENU HƯỚNG DẪN SỬ DỤNG (HELP)
// =========================================================================
fn handle_help() {
    println!("\n{}", "🤖 ULTIMATE GIT-AI CLI".bold().cyan());
    println!(
        "{}",
        "Công cụ hỗ trợ viết Git Commit bằng AI nhanh chóng.\n".italic()
    );

    println!("{}", "📌 DANH SÁCH LỆNH:".bold().yellow());
    println!(
        "  {:<12} {}",
        "diff".green().bold(),
        ": Quét code thay đổi, tạo Prompt & copy vào Clipboard để gửi AI."
    );
    println!(
        "  {:<12} {}",
        "go".green().bold(),
        ": Đọc commit message từ Clipboard, tự động Add, Commit & Push."
    );
    println!(
        "  {:<12} {}",
        "install".green().bold(),
        ": Cấu hình phím tắt (git-copydiff, git-go) vào Terminal."
    );
    println!(
        "  {:<12} {}",
        "uninstall".green().bold(),
        ": Gỡ bỏ cấu hình phím tắt khỏi hệ thống."
    );
    println!(
        "  {:<12} {}",
        "help, -h".green().bold(),
        ": Hiển thị bảng hướng dẫn này.\n"
    );

    println!("{}", "💡 QUY TRÌNH SỬ DỤNG CHUẨN:".bold().magenta());
    println!(
        "  1. Sửa code xong, gõ `{}` để quét thay đổi.",
        "git-ai diff".cyan()
    );
    println!("  2. Dán (Ctrl+V) nội dung vào ChatGPT/Claude/Gemini.");
    println!("  3. Copy lại câu trả lời (commit message) của AI.");
    println!(
        "  4. Gõ `{}` để tool tự động đóng gói và đẩy code lên git.\n",
        "git-ai go".cyan()
    );
}

// =========================================================================
// TỰ ĐỘNG GỠ BỎ CẤU HÌNH (CROSS-PLATFORM UNINSTALL)
// =========================================================================
fn handle_uninstall() -> Result<()> {
    println!(
        "{}",
        "🗑️  Đang tiến hành gỡ bỏ cấu hình khỏi hệ thống..."
            .bold()
            .red()
    );

    #[cfg(target_family = "unix")]
    {
        let profiles = get_unix_profiles();
        let mut found = false;

        for profile in profiles {
            if clean_profile_file(&profile)? {
                println!(
                    "{} {}",
                    "✅ Đã gỡ bỏ thành công khỏi".green().bold(),
                    profile.display()
                );
                found = true;
            }
        }

        if found {
            println!(
                "{}",
                "👉 Vui lòng khởi động lại Terminal hoặc chạy 'source ~/.zshrc' để áp dụng."
                    .yellow()
            );
        } else {
            println!(
                "{}",
                "ℹ️ Không tìm thấy cấu hình git-ai nào để gỡ.".yellow()
            );
        }
    }

    #[cfg(target_os = "windows")]
    {
        let profile = get_windows_profile()?;
        if clean_profile_file(&profile)? {
            println!(
                "{}",
                "✅ Đã gỡ bỏ các chức năng khỏi PowerShell Profile!"
                    .green()
                    .bold()
            );
            println!(
                "{}",
                "👉 Vui lòng khởi động lại PowerShell để áp dụng thay đổi.".yellow()
            );
        } else {
            println!(
                "{}",
                "ℹ️ Không tìm thấy cấu hình PowerShell Profile để gỡ.".yellow()
            );
        }
    }
    Ok(())
}

// =========================================================================
// TỰ ĐỘNG CẤU HÌNH HỆ THỐNG (CROSS-PLATFORM AUTO INSTALL)
// =========================================================================
fn handle_install() -> Result<()> {
    let exe_path = env::current_exe()?;
    let exe_str = exe_path.to_string_lossy();

    println!(
        "{}",
        "🛠️  Đang tiến hành tự động cấu hình hệ thống..."
            .bold()
            .blue()
    );

    #[cfg(target_family = "unix")]
    {
        let profiles = get_unix_profiles();
        // Ưu tiên cài vào file profile đang tồn tại
        let target_profile = profiles.iter().find(|p| p.exists()).unwrap_or(&profiles[0]);

        let alias_lines = format!(
            "\n# ULTIMATE GIT-AI WORKFLOW\nalias git-copydiff=\"'{}' diff\"\nalias git-go=\"'{}' go\"\nalias git-ai-uninstall=\"'{}' uninstall\"\nalias git-ai=\"'{}'\"\n",
            exe_str, exe_str, exe_str, exe_str
        );

        append_to_file(target_profile, &alias_lines)?;

        println!(
            "{}",
            "✅ Cấu hình thành công! Đã thêm các phím tắt:"
                .green()
                .bold()
        );
        print_commands_help();
        println!(
            "\n{} {}",
            "👉 Vui lòng chạy lệnh:".yellow(),
            format!("source {}", target_profile.display()).cyan()
        );
    }

    #[cfg(target_os = "windows")]
    {
        let profile_path = get_windows_profile()?;

        if let Some(parent) = profile_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let func_lines = format!(
            "\n# ULTIMATE GIT-AI WORKFLOW\nfunction git-copydiff {{ & \"{}\" diff }}\nfunction git-go {{ & \"{}\" go }}\nfunction git-ai-uninstall {{ & \"{}\" uninstall }}\nfunction git-ai {{ & \"{}\" }}\n",
            exe_str, exe_str, exe_str, exe_str
        );

        append_to_file(&profile_path, &func_lines)?;

        println!(
            "{}",
            "✅ Cấu hình thành công! Đã thêm các phím tắt:"
                .green()
                .bold()
        );
        print_commands_help();
        println!(
            "\n{}",
            "👉 Vui lòng khởi động lại PowerShell để kích hoạt các lệnh mới.".yellow()
        );
    }
    Ok(())
}

// 1. CHỨC NĂNG SNAPSHOT CAPTURE (git-ai diff)
fn handle_diff() -> Result<()> {
    let output = Command::new("git").args(["diff"]).output()?;
    let diff_str = String::from_utf8_lossy(&output.stdout);

    if diff_str.trim().is_empty() {
        println!(
            "\n{}",
            "⚠️ [HỆ THỐNG]: Không phát hiện thay đổi nào trong code. Đã hủy chụp snapshot!"
                .yellow()
                .bold()
        );
        return Ok(());
    }

    let prompt = format!(
        "Act as an expert developer. Output ONLY the raw commit message for the diff below. Rules: 1. Subject line: Conventional Commits, < 50 chars. 2. Blank line. 3. Body: 1-2 extremely short sentences explaining WHAT and WHY. Be direct, no fluff. STRICTLY NO markdown formatting (no ```), NO preamble, NO greetings. Diff:\n\n{}",
        diff_str
    );

    let mut clipboard = Clipboard::new()?;
    clipboard.set_text(prompt)?;

    println!(
        "\n{}",
        "✨ [HỆ THỐNG]: Đã chụp snapshot. Hãy dán (Ctrl+V) vào AI để lấy commit message! 🤖"
            .magenta()
            .bold()
    );
    Ok(())
}

// 2. CHỨC NĂNG AUTO-DEPLOY (git-ai go)
fn handle_go() -> Result<()> {
    println!("\n{}", "📂 [XEM TRƯỚC THAY ĐỔI]".cyan().bold());

    let status_cmd = Command::new("git").args(["status", "-s"]).status()?;
    if !status_cmd.success() {
        return Ok(());
    }

    let mut clipboard = Clipboard::new()?;
    let commit_msg = clipboard.get_text().unwrap_or_default();

    println!("\n{}", "💬 [NỘI DUNG COMMIT TỪ AI]".magenta().bold());
    println!("{}", commit_msg.green());
    println!();

    if ask_confirm("🚀 Xác nhận deploy (commit & push)? (Y/n): ")? {
        println!("{}", "⚡ Đang tiến hành đẩy code...".blue().bold());

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
            println!(
                "\n{}",
                "✅ Hoàn tất. Code đã được đẩy lên mây thành công! ☁️"
                    .green()
                    .bold()
            );
        }
    } else {
        println!("\n{}", "❌ Đã hủy quá trình deploy.".red().bold());
    }
    Ok(())
}

// =========================================================================
// HELPER FUNCTIONS (CÁC HÀM HỖ TRỢ CHUẨN DRY)
// =========================================================================

#[cfg(target_family = "unix")]
fn get_unix_profiles() -> Vec<PathBuf> {
    let home = env::var("HOME").unwrap_or_else(|_| "~".to_string());
    // Hỗ trợ cả zsh và bash
    vec![
        PathBuf::from(format!("{}/.zshrc", home)),
        PathBuf::from(format!("{}/.bashrc", home)),
    ]
}

#[cfg(target_os = "windows")]
fn get_windows_profile() -> Result<PathBuf> {
    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command", "Write-Output $PROFILE"]) // Dùng -NoProfile để load nhanh hơn
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

fn ask_confirm(prompt: &str) -> Result<bool> {
    print!("{}", prompt.yellow().bold());
    io::stdout().flush()?;

    enable_raw_mode()?;
    let mut confirm = false;

    loop {
        if let Event::Key(key_event) = read()? {
            match key_event.code {
                KeyCode::Enter | KeyCode::Char('y') | KeyCode::Char('Y') => {
                    confirm = true;
                    break;
                }
                KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                    break;
                }
                _ => {}
            }
        }
    }

    disable_raw_mode()?;
    println!();
    Ok(confirm)
}

fn print_commands_help() {
    println!(
        "{}",
        "  -> git-copydiff       : Chụp snapshot code diff".cyan()
    );
    println!(
        "{}",
        "  -> git-go             : Đóng gói và đẩy code lên git".cyan()
    );
    println!(
        "{}",
        "  -> git-ai-uninstall   : Gỡ cài đặt bộ tool này".cyan()
    );
    println!(
        "{}",
        "  -> git-ai             : Lệnh gốc (ví dụ: git-ai help)".cyan()
    );
}
