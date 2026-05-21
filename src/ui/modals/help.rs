use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};
use crate::app::App;

pub fn render_help_modal(f: &mut Frame, app: &App, area: Rect) {
    let is_vi = app.current_lang == "vi";
    let theme = app.theme();
    let mut content = vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            if is_vi {
                "🤖 BẢNG HƯỚNG DẪN PHÍM TẮT HỆ THỐNG 🤖"
            } else {
                "🤖 SYSTEM MANUAL & KEYBOARD LEGEND 🤖"
            },
            Style::default()
                .fg(theme.cyan)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
    ];

    let shortcut_groups = vec![
        (
            "Navigation ",
            vec![
                (
                    "↑/↓ / j/k",
                    if is_vi {
                        "Chọn tập tin trong danh sách"
                    } else {
                        "Navigate/Select file in change list"
                    },
                ),
                (
                    "PgUp/PgDn",
                    if is_vi {
                        "Cuộn xem Diff chi tiết"
                    } else {
                        "Scroll detailed code diff viewer"
                    },
                ),
            ],
        ),
        (
            "Git Operations ",
            vec![
                (
                    "Space",
                    if is_vi {
                        "Stage / Unstage (git add / restore)"
                    } else {
                        "Stage / Unstage file (git add / restore)"
                    },
                ),
                (
                    "Backspace",
                    if is_vi {
                        "Khôi phục / Xóa bỏ thay đổi (git restore)"
                    } else {
                        "Revert / Delete changes (git restore)"
                    },
                ),
                (
                    "a",
                    if is_vi {
                        "Stage toàn bộ thay đổi (git add .)"
                    } else {
                        "Stage all changes (git add .)"
                    },
                ),
                (
                    "u",
                    if is_vi {
                        "Unstage toàn bộ thay đổi (git reset)"
                    } else {
                        "Unstage all changes (git reset)"
                    },
                ),
                (
                    "b",
                    if is_vi {
                        "Quản lý chi nhánh ([m] merge, [c] tạo chi nhánh mới)"
                    } else {
                        "Manage branches ([m] merge, [c] create new branch)"
                    },
                ),
                (
                    "v",
                    if is_vi {
                        "Xem lịch sử commit timeline"
                    } else {
                        "View timeline of last 15 commits"
                    },
                ),
                (
                    "f",
                    if is_vi {
                        "Tìm nạp toàn bộ metadata từ máy chủ (git fetch)"
                    } else {
                        "Fetch all remote branch metadata (git fetch)"
                    },
                ),
                (
                    "p",
                    if is_vi {
                        "Cập nhật thay đổi từ máy chủ về máy (git pull)"
                    } else {
                        "Pull latest changes from remote (git pull)"
                    },
                ),
                (
                    "d",
                    if is_vi {
                        "Chụp ảnh Diff chuyển qua AI Clipboard"
                    } else {
                        "Capture & Copy code diff to AI Clipboard"
                    },
                ),
                (
                    "g",
                    if is_vi {
                        "Đóng gói toàn bộ, tự động commit & push (Go)"
                    } else {
                        "Commit & Push changes auto (Go)"
                    },
                ),
            ],
        ),
        (
            "System Operations ",
            vec![
                (
                    "o",
                    if is_vi {
                        "Mở thư mục hiện tại bằng VS Code"
                    } else {
                        "Open workspace folder in VS Code"
                    },
                ),
                (
                    "w",
                    if is_vi {
                        "Đổi sang thư mục Project khác"
                    } else {
                        "Switch to another workspace project"
                    },
                ),
                (
                    "l",
                    if is_vi {
                        "Thay đổi ngôn ngữ TUI (Language Panel)"
                    } else {
                        "Open language configuration panel"
                    },
                ),
                (
                    "t",
                    if is_vi {
                        "Mở bảng cấu hình giao diện Sáng/Tối (Theme)"
                    } else {
                        "Open light/dark theme selection panel"
                    },
                ),
                (
                    "r",
                    if is_vi {
                        "Khôi phục cài đặt gốc của git-ai"
                    } else {
                        "Reset git-ai to default system settings"
                    },
                ),
                (
                    "q / Esc",
                    if is_vi {
                        "Đóng cửa sổ / Thoát chương trình"
                    } else {
                        "Close modal / Exit TUI dashboard"
                    },
                ),
            ],
        ),
    ];

    for (group_title, items) in shortcut_groups {
        content.push(Line::from(vec![Span::styled(
            format!("  ■ {}", group_title),
            Style::default()
                .fg(theme.purple)
                .add_modifier(Modifier::BOLD),
        )]));
        for (key, desc) in items {
            content.push(Line::from(vec![
                Span::styled("   ⚡ [", Style::default().fg(theme.border)),
                Span::styled(
                    key,
                    Style::default()
                        .fg(theme.green)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled("] : ", Style::default().fg(theme.border)),
                Span::styled(desc, Style::default().fg(theme.fg)),
            ]));
        }
        content.push(Line::from(""));
    }

    content.push(Line::from(vec![Span::styled(
        if is_vi {
            "Nhấn [Esc], [Space], hoặc [Enter] để ĐÓNG."
        } else {
            "Press [Esc], [Space], or [Enter] to CLOSE."
        },
        Style::default()
            .fg(theme.orange)
            .add_modifier(Modifier::BOLD),
    )]));

    let block = Block::default()
        .title(Span::styled(
            " SYSTEM MANUAL ",
            Style::default()
                .fg(theme.cyan)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.cyan))
        .border_type(BorderType::Double)
        .style(Style::default().bg(theme.bg));

    let paragraph = Paragraph::new(content)
        .alignment(ratatui::layout::Alignment::Center)
        .block(block);

    f.render_widget(paragraph, area);
}
