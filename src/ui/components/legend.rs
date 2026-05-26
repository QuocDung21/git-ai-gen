use crate::app::App;
use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

pub fn render_legend(f: &mut Frame, app: &App, area: Rect) {
    let is_vi = app.current_lang == "vi";
    let theme = app.theme();
    let mut legend_lines = vec![Line::from("")];
    let nav_items = if app.focus_diff {
        vec![
            (
                "↑/↓ / j/k",
                if is_vi {
                    "Cuộn dòng diff"
                } else {
                    "Line scroll diff"
                },
                theme.yellow,
            ),
            (
                "d / u",
                if is_vi {
                    "Cuộn trang diff"
                } else {
                    "Page scroll diff"
                },
                theme.yellow,
            ),
            (
                "Tab / Esc",
                if is_vi {
                    "Quay lại thay đổi"
                } else {
                    "Return to changes"
                },
                theme.purple,
            ),
        ]
    } else {
        vec![
            (
                "↑/↓ / j/k",
                if is_vi {
                    "Chọn tập tin"
                } else {
                    "Select file"
                },
                theme.purple,
            ),
            (
                "Tab / l / →",
                if is_vi {
                    "Cuộn chi tiết diff"
                } else {
                    "Focus Diff scroll"
                },
                theme.yellow,
            ),
            (
                "[ / ]",
                if is_vi {
                    "Cuộn nhanh diff"
                } else {
                    "Quick scroll diff"
                },
                theme.cyan,
            ),
        ]
    };

    let groups = vec![
        ("Navigation", nav_items),
        (
            "Git Operations",
            vec![
                (
                    "Space",
                    "Stage/Unstage",
                    theme.green,
                ),
                (
                    "Backspace",
                    if is_vi {
                        "Revert / Xóa"
                    } else {
                        "Revert / Delete"
                    },
                    theme.red,
                ),
                (
                    "A",
                    if is_vi {
                        "Stage tất cả"
                    } else {
                        "Stage all"
                    },
                    theme.green,
                ),
                (
                    "U",
                    if is_vi {
                        "Unstage tất cả"
                    } else {
                        "Unstage all"
                    },
                    theme.red,
                ),
                (
                    "B",
                    if is_vi {
                        "Quản lý nhánh (Đổi/Trộn/Tạo)"
                    } else {
                        "Manage branch (Switch/Merge/New)"
                    },
                    theme.cyan,
                ),
                (
                    "V",
                    if is_vi {
                        "Xem Lịch sử"
                    } else {
                        "Commit history"
                    },
                    theme.yellow,
                ),
                (
                    "F",
                    if is_vi {
                        "Tìm nạp (Fetch)"
                    } else {
                        "Git Fetch"
                    },
                    theme.cyan,
                ),
                (
                    "P",
                    if is_vi {
                        "Cập nhật (Pull)"
                    } else {
                        "Git Pull"
                    },
                    theme.cyan,
                ),
                (
                    "I",
                    if is_vi {
                        "Thông tin & danh sách Remote"
                    } else {
                        "Remote info & list"
                    },
                    theme.cyan,
                ),
                (
                    "D",
                    "Copy diff -> AI",
                    theme.yellow,
                ),
                (
                    "X",
                    if is_vi {
                        "Xem prompt AI đã thiết lập"
                    } else {
                        "View configured AI prompt"
                    },
                    theme.yellow,
                ),
                (
                    "G",
                    "Git Menu (Add/Commit/Fetch/Pull/Remote...)",
                    theme.green,
                ),
                (
                    "N",
                    if is_vi {
                        "Tải tập tin từ GitHub"
                    } else {
                        "Download from GitHub"
                    },
                    theme.cyan,
                ),
            ],
        ),
        (
            "System",
            vec![
                (
                    "O",
                    if is_vi { "Mở VS Ide" } else { "Open VS Ide" },
                    theme.purple,
                ),
                (
                    "W",
                    if is_vi {
                        "Chọn Project"
                    } else {
                        "Select Project"
                    },
                    theme.cyan,
                ),
                (
                    "E",
                    if is_vi {
                        "Thống kê ngôn ngữ"
                    } else {
                        "Language stats"
                    },
                    theme.purple,
                ),
                (
                    "L",
                    if is_vi {
                        "Đổi ngôn ngữ"
                    } else {
                        "Toggle lang"
                    },
                    theme.purple,
                ),
                (
                    "T",
                    if is_vi {
                        "Chọn giao diện"
                    } else {
                        "Select theme"
                    },
                    theme.purple,
                ),
                (
                    ",",
                    if is_vi {
                        "Cài đặt hệ thống"
                    } else {
                        "System settings"
                    },
                    theme.purple,
                ),
                (
                    "R",
                    if is_vi {
                        "Reset cài đặt"
                    } else {
                        "Reset settings"
                    },
                    theme.red,
                ),
                (
                    "? / H",
                    if is_vi {
                        "Mở hướng dẫn"
                    } else {
                        "Open manual"
                    },
                    theme.cyan,
                ),
                (
                    "Q",
                    if is_vi {
                        "Thoát TUI panel"
                    } else {
                        "Exit TUI panel"
                    },
                    theme.border,
                ),
            ],
        ),
    ];

    for (group_title, items) in groups {
        legend_lines.push(Line::from(vec![Span::styled(
            format!("  ■ {}", group_title),
            Style::default().fg(theme.cyan).add_modifier(Modifier::BOLD),
        )]));
        for (key, desc, color) in items {
            legend_lines.push(Line::from(vec![
                Span::styled("   ⚡ [", Style::default().fg(theme.border)),
                Span::styled(key, Style::default().fg(color).add_modifier(Modifier::BOLD)),
                Span::styled("]  ", Style::default().fg(theme.border)),
                Span::styled(desc.to_string(), Style::default().fg(theme.fg)),
            ]));
        }
        legend_lines.push(Line::from(""));
    }

    let legend_widget = Paragraph::new(legend_lines).block(
        Block::default()
            .title(Span::styled(
                if is_vi {
                    " ⚡ BẢNG PHÍM TẮT "
                } else {
                    " ⚡ CONTROL LEGEND "
                },
                Style::default().fg(theme.cyan).add_modifier(Modifier::BOLD),
            ))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme.cyan))
            .border_type(BorderType::Rounded),
    );
    f.render_widget(legend_widget, area);
}
