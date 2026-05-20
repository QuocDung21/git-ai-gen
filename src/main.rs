use arboard::Clipboard;
use colored::*;
use crossterm::event::{read, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::env;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!(
            "{}",
            "❌ Vui lòng chọn lệnh: 'diff', 'go', 'install' hoặc 'uninstall'".red()
        );
        return;
    }

    match args[1].as_str() {
        "diff" => handle_diff(),
        "go" => handle_go(),
        "install" => handle_install(),
        "uninstall" => handle_uninstall(),
        _ => println!(
            "{}",
            "❌ Lệnh không hợp lệ. Chỉ dùng 'diff', 'go', 'install' hoặc 'uninstall'.".red()
        ),
    }
}

// =========================================================================
// TỰ ĐỘNG GỠ BỎ CẤU HÌNH (CROSS-PLATFORM UNINSTALL)
// =========================================================================
fn handle_uninstall() {
    println!(
        "{}",
        "🗑️  Đang tiến hành gỡ bỏ cấu hình khỏi hệ thống..."
            .bold()
            .red()
    );

    // CẤU HÌNH GỠ CHO MACOS / LINUX
    #[cfg(target_family = "unix")]
    {
        let home = env::var("HOME").expect("Không tìm thấy thư mục HOME");
        let zshrc_path = format!("{}/.zshrc", home);

        if std::path::Path::new(&zshrc_path).exists() {
            let content =
                std::fs::read_to_string(&zshrc_path).expect("Không thể đọc file ~/.zshrc");
            let lines: Vec<&str> = content.lines().collect();

            // Lọc bỏ sạch sẽ tất cả các dòng liên quan đến git-ai
            let mut filtered_lines = Vec::new();
            for line in lines {
                if !line.contains("# ULTIMATE GIT-AI WORKFLOW")
                    && !line.contains("alias git-copydiff=")
                    && !line.contains("alias git-go=")
                    && !line.contains("alias git-ai-uninstall=")
                    && !line.contains("alias git-ai=")
                {
                    filtered_lines.push(line);
                }
            }

            let new_content = filtered_lines.join("\n") + "\n";
            std::fs::write(&zshrc_path, new_content).expect("Không thể ghi lại file ~/.zshrc");

            println!(
                "{}",
                "✅ Đã gỡ bỏ hoàn toàn các alias khỏi ~/.zshrc!"
                    .green()
                    .bold()
            );
            println!(
                "{}",
                "👉 Vui lòng chạy lệnh: source ~/.zshrc để áp dụng.".yellow()
            );
        } else {
            println!(
                "{}",
                "ℹ️ Không tìm thấy file ~/.zshrc để gỡ cấu hình.".yellow()
            );
        }
    }

    // CẤU HÌNH GỠ CHO WINDOWS (POWERSHELL)
    #[cfg(target_os = "windows")]
    {
        let output = Command::new("powershell")
            .args(["-Command", "$PROFILE"])
            .output()
            .expect("Không thể kiểm tra Profile của PowerShell");

        let profile_path_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
        let profile_path = std::path::Path::new(&profile_path_str);

        if profile_path.exists() {
            let content = std::fs::read_to_string(profile_path)
                .expect("Không thể đọc file PowerShell Profile");
            let lines: Vec<&str> = content.lines().collect();

            let mut filtered_lines = Vec::new();
            for line in lines {
                if !line.contains("# ULTIMATE GIT-AI WORKFLOW")
                    && !line.contains("function git-copydiff")
                    && !line.contains("function git-go")
                    && !line.contains("function git-ai-uninstall")
                    && !line.contains("function git-ai")
                {
                    filtered_lines.push(line);
                }
            }

            let new_content = filtered_lines.join("\n") + "\n";
            std::fs::write(profile_path, new_content)
                .expect("Không thể ghi lại file PowerShell Profile");

            println!(
                "{}",
                "✅ Đã gỡ bỏ các functions khỏi PowerShell Profile!"
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
}

// =========================================================================
// TỰ ĐỘNG CẤU HÌNH HỆ THỐNG (CROSS-PLATFORM AUTO INSTALL)
// =========================================================================
fn handle_install() {
    let exe_path = env::current_exe().expect("Không thể xác định đường dẫn file thực thi");
    let exe_str = exe_path.to_string_lossy();

    println!(
        "{}",
        "🛠️  Đang tiến hành tự động cấu hình hệ thống..."
            .bold()
            .blue()
    );

    // CẤU HÌNH CHO MACOS (Thêm luôn lệnh gốc git-ai và lệnh gỡ cài đặt nhanh)
    #[cfg(target_family = "unix")]
    {
        let home = env::var("HOME").expect("Không tìm thấy thư mục HOME");
        let zshrc_path = format!("{}/.zshrc", home);

        let alias_lines = format!(
            "\n# ULTIMATE GIT-AI WORKFLOW\nalias git-copydiff=\"'{}' diff\"\nalias git-go=\"'{}' go\"\nalias git-ai-uninstall=\"'{}' uninstall\"\nalias git-ai=\"'{}'\"\n",
            exe_str, exe_str, exe_str, exe_str
        );

        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&zshrc_path)
            .expect("Không thể mở file ~/.zshrc");

        file.write_all(alias_lines.as_bytes())
            .expect("Không thể ghi vào ~/.zshrc");

        println!(
            "{}",
            "✅ Cấu hình thành công! Đã thêm các lệnh vào ~/.zshrc:"
                .green()
                .bold()
        );
        println!(
            "{}",
            "  -> git-copydiff       : Chụp snapshot code diff".cyan()
        );
        println!(
            "{}",
            "  -> git-go             : Tiến hành deploy lên mây".cyan()
        );
        println!(
            "{}",
            "  -> git-ai-uninstall   : Gỡ cài đặt bộ tool này".cyan()
        );
        println!(
            "{}",
            "  -> git-ai             : Lệnh gốc (ví dụ: git-ai diff)".cyan()
        );
        println!(
            "\n{}",
            "👉 Vui lòng chạy lệnh: source ~/.zshrc để kích hoạt ngay lập tức.".yellow()
        );
    }

    // CẤU HÌNH CHO WINDOWS (POWERSHELL)
    #[cfg(target_os = "windows")]
    {
        let output = Command::new("powershell")
            .args(["-Command", "$PROFILE"])
            .output()
            .expect("Không thể kiểm tra Profile của PowerShell");

        let profile_path_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
        let profile_path = std::path::Path::new(&profile_path_str);

        if let Some(parent) = profile_path.parent() {
            std::fs::create_dir_all(parent).expect("Không thể tạo thư mục Profile");
        }

        let func_lines = format!(
            "\n# ULTIMATE GIT-AI WORKFLOW\nfunction git-copydiff {{ & \"{}\" diff }}\nfunction git-go {{ & \"{}\" go }}\nfunction git-ai-uninstall {{ & \"{}\" uninstall }}\nfunction git-ai {{ & \"{}\" }}\n",
            exe_str, exe_str, exe_str, exe_str
        );

        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(profile_path)
            .expect("Không thể mở file PowerShell Profile");

        file.write_all(func_lines.as_bytes())
            .expect("Không thể ghi vào PowerShell Profile");

        println!(
            "{}",
            "✅ Cấu hình thành công! Đã thêm các lệnh vào PowerShell Profile:"
                .green()
                .bold()
        );
        println!(
            "{}",
            "  -> git-copydiff       : Chụp snapshot code diff".cyan()
        );
        println!(
            "{}",
            "  -> git-go             : Tiến hành deploy lên mây".cyan()
        );
        println!(
            "{}",
            "  -> git-ai-uninstall   : Gỡ cài đặt bộ tool này".cyan()
        );
        println!("{}", "  -> git-ai             : Lệnh gốc".cyan());
        println!(
            "\n{}",
            "👉 Vui lòng khởi động lại PowerShell để kích hoạt các lệnh mới.".yellow()
        );
    }
}

// 1. CHỨC NĂNG SNAPSHOT CAPTURE (git-ai diff)
fn handle_diff() {
    let output = Command::new("git")
        .args(["diff"])
        .output()
        .expect("Không thể chạy lệnh git. Hãy chắc chắn git đã được cài đặt.");

    let diff_str = String::from_utf8_lossy(&output.stdout);

    if diff_str.trim().is_empty() {
        println!(
            "\n{}",
            "⚠️ [SYSTEM]: Không phát hiện thay đổi nào trong code. Hủy chụp snapshot!"
                .yellow()
                .bold()
        );
        return;
    }

    let prompt = format!(
        r#"Act as an expert developer. Output ONLY the raw commit message for the diff below. Rules: 1. Subject line: Conventional Commits, < 50 chars. 2. Blank line. 3. Body: 1-2 extremely short sentences explaining WHAT and WHY. Be direct, no fluff. STRICTLY NO markdown formatting (no ```), NO preamble, NO greetings. Diff:

{}"#,
        diff_str
    );

    let mut clipboard = Clipboard::new().expect("Không thể kết nối tới Clipboard hệ thống.");
    clipboard
        .set_text(prompt)
        .expect("Không thể ghi vào Clipboard.");

    println!(
        "\n{}",
        "✨ [SYSTEM]: Snapshot captured (Short & Direct). Feed the AI beast! 🤖"
            .magenta()
            .bold()
    );
}

// 2. CHỨC NĂNG AUTO-DEPLOY (git-ai go)
fn handle_go() {
    println!("\n{}", "📂 [CHANGELOG PREVIEW]".cyan().bold());

    let status_cmd = Command::new("git")
        .args(["status", "-s"])
        .status()
        .expect("Không thể lấy trạng thái git.");

    if !status_cmd.success() {
        return;
    }

    let mut clipboard = Clipboard::new().expect("Không thể mở Clipboard.");
    let commit_msg = clipboard.get_text().unwrap_or_else(|_| "".to_string());

    println!("\n{}", "💬 [AI-CRAFTED MESSAGE]".magenta().bold());
    println!("{}", commit_msg.green());
    println!();

    print!("{}", "🚀 Execute deployment? (Y/n): ".yellow().bold());
    io::stdout().flush().unwrap();

    enable_raw_mode().unwrap();

    let execute_deploy = loop {
        if let Ok(Event::Key(key_event)) = read() {
            match key_event.code {
                KeyCode::Enter | KeyCode::Char('y') | KeyCode::Char('Y') => {
                    break true;
                }
                KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                    break false;
                }
                _ => {}
            }
        }
    };

    disable_raw_mode().unwrap();
    println!();

    if execute_deploy {
        println!("{}", "⚡ Priming engines...".blue().bold());

        let add_ok = Command::new("git")
            .args(["add", "."])
            .status()
            .map_or(false, |s| s.success());
        if !add_ok {
            return;
        }

        let commit_ok = Command::new("git")
            .args(["commit", "-m", &commit_msg])
            .status()
            .map_or(false, |s| s.success());
        if !commit_ok {
            return;
        }

        let push_ok = Command::new("git")
            .args(["push"])
            .status()
            .map_or(false, |s| s.success());
        if push_ok {
            println!(
                "\n{}",
                "✅ Mission Accomplished. Code is in the clouds! ☁️"
                    .green()
                    .bold()
            );
        }
    } else {
        println!("\n{}", "❌ Mission Aborted. Stand down.".red().bold());
    }
}
