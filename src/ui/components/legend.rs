use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};
use crate::app::App;

pub fn render_legend(f: &mut Frame, app: &App, area: Rect) {
    let is_vi = app.current_lang == "vi";
    let mut legend_lines = vec![Line::from("")];
    let groups = vec![
        (
            "Navigation",
            vec![
                (
                    "↑/↓ / j/k",
                    if is_vi {
                        "Chọn tập tin"
                    } else {
                        "Select file"
                    },
                    Color::Rgb(189, 147, 249),
                ),
                (
                    "PgUp/Dn",
                    if is_vi { "Cuộn diff" } else { "Scroll diff" },
                    Color::Rgb(189, 147, 249),
                ),
            ],
        ),
        (
            "Git Operations",
            vec![
                (
                    "Space",
                    if is_vi {
                        "Stage/Unstage"
                    } else {
                        "Stage/Unstage"
                    },
                    Color::Rgb(80, 250, 123),
                ),
                (
                    "Backspace",
                    if is_vi {
                        "Revert / Xóa"
                    } else {
                        "Revert / Delete"
                    },
                    Color::Rgb(255, 85, 85),
                ),
                (
                    "A",
                    if is_vi {
                        "Stage tất cả"
                    } else {
                        "Stage all"
                    },
                    Color::Rgb(80, 250, 123),
                ),
                (
                    "U",
                    if is_vi {
                        "Unstage tất cả"
                    } else {
                        "Unstage all"
                    },
                    Color::Rgb(255, 85, 85),
                ),
                (
                    "B",
                    if is_vi {
                        "Đổi / Trộn nhánh (Merge)"
                    } else {
                        "Switch / Merge branch"
                    },
                    Color::Rgb(139, 233, 253),
                ),
                (
                    "V",
                    if is_vi {
                        "Xem Lịch sử"
                    } else {
                        "Commit history"
                    },
                    Color::Rgb(241, 250, 140),
                ),
                (
                    "F",
                    if is_vi {
                        "Tìm nạp (Fetch)"
                    } else {
                        "Git Fetch"
                    },
                    Color::Rgb(139, 233, 253),
                ),
                (
                    "P",
                    if is_vi {
                        "Cập nhật (Pull)"
                    } else {
                        "Git Pull"
                    },
                    Color::Rgb(139, 233, 253),
                ),
                (
                    "D",
                    if is_vi {
                        "Copy diff -> AI"
                    } else {
                        "Copy diff -> AI"
                    },
                    Color::Rgb(241, 250, 140),
                ),
                (
                    "G",
                    if is_vi {
                        "Đóng gói (Go)"
                    } else {
                        "Commit & Push (Go)"
                    },
                    Color::Rgb(80, 250, 123),
                ),
            ],
        ),
        (
            "System",
            vec![
                (
                    "O",
                    if is_vi {
                        "Mở VS Code"
                    } else {
                        "Open VS Code"
                    },
                    Color::Rgb(255, 121, 198),
                ),
                (
                    "W",
                    if is_vi {
                        "Chọn Project"
                    } else {
                        "Select Project"
                    },
                    Color::Rgb(139, 233, 253),
                ),
                (
                    "L",
                    if is_vi {
                        "Đổi ngôn ngữ"
                    } else {
                        "Toggle lang"
                    },
                    Color::Rgb(189, 147, 249),
                ),
                (
                    "R",
                    if is_vi {
                        "Reset cài đặt"
                    } else {
                        "Reset settings"
                    },
                    Color::Rgb(255, 85, 85),
                ),
                (
                    "? / H",
                    if is_vi {
                        "Mở hướng dẫn"
                    } else {
                        "Open manual"
                    },
                    Color::Rgb(139, 233, 253),
                ),
                (
                    "Q",
                    if is_vi {
                        "Thoát TUI panel"
                    } else {
                        "Exit TUI panel"
                    },
                    Color::Rgb(98, 114, 164),
                ),
            ],
        ),
    ];

    for (group_title, items) in groups {
        legend_lines.push(Line::from(vec![Span::styled(
            format!("  ■ {}", group_title),
            Style::default()
                .fg(Color::Rgb(139, 233, 253))
                .add_modifier(Modifier::BOLD),
        )]));
        for (key, desc, color) in items {
            legend_lines.push(Line::from(vec![
                Span::styled("   ⚡ [", Style::default().fg(Color::Rgb(98, 114, 164))),
                Span::styled(key, Style::default().fg(color).add_modifier(Modifier::BOLD)),
                Span::styled("]  ", Style::default().fg(Color::Rgb(98, 114, 164))),
                Span::styled(
                    desc.to_string(),
                    Style::default().fg(Color::Rgb(248, 248, 242)),
                ),
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
                Style::default()
                    .fg(Color::Rgb(139, 233, 253))
                    .add_modifier(Modifier::BOLD),
            ))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Rgb(139, 233, 253)))
            .border_type(BorderType::Rounded),
    );
    f.render_widget(legend_widget, area);
}
