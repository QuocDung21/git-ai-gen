use std::process::Command;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Clear},
    Frame,
};
use crate::app::App;
use crate::app::models::{GoStep, AmendStep, StashStep, StashAction};

pub fn render_language_select(f: &mut Frame, app: &App, area: Rect) {
    let is_vi = app.current_lang == "vi";
    f.render_widget(Clear, area);

    let mut content = vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            if is_vi {
                "Chọn ngôn ngữ của bạn / Select your language:"
            } else {
                "Select language / Chọn ngôn ngữ:"
            },
            Style::default()
                .fg(Color::Rgb(248, 248, 242))
                .add_modifier(Modifier::ITALIC),
        )]),
        Line::from(""),
    ];

    let items = vec![
        ("vi", "Tiếng Việt 🇻🇳", "[v]"),
        ("en", "English 🇺🇸", "[e]"),
        ("auto", "Tự động / Auto (System) ⚙️", "[a]"),
    ];

    // Resolve the current raw git config setting dynamically
    let raw_lang = if let Ok(output) = Command::new("git")
        .args(["config", "--global", "--get", "git-ai.lang"])
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout)
            .trim()
            .to_lowercase();
        if stdout == "vi" || stdout == "en" {
            stdout
        } else {
            "auto".to_string()
        }
    } else {
        "auto".to_string()
    };

    for (i, (lang_code, label, shortcut)) in items.into_iter().enumerate() {
        let is_hovered = i == app.selected_lang_index;
        let is_currently_active = raw_lang == lang_code;

        let cursor = if is_hovered {
            Span::styled(
                " ▶ ",
                Style::default()
                    .fg(Color::Rgb(189, 147, 249))
                    .add_modifier(Modifier::BOLD),
            )
        } else {
            Span::styled("   ", Style::default())
        };

        let active_badge = if is_currently_active {
            Span::styled(
                " (Active) ",
                Style::default()
                    .fg(Color::Rgb(80, 250, 123))
                    .add_modifier(Modifier::ITALIC),
            )
        } else {
            Span::styled("", Style::default())
        };

        let item_style = if is_hovered {
            Style::default()
                .fg(Color::Rgb(248, 248, 242))
                .bg(Color::Rgb(68, 71, 90))
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Rgb(248, 248, 242))
        };

        content.push(Line::from(vec![
            cursor,
            Span::styled(
                format!("{} ", shortcut),
                Style::default()
                    .fg(Color::Rgb(139, 233, 253))
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(label, item_style),
            active_badge,
        ]));
    }

    content.push(Line::from(""));
    content.push(Line::from(vec![Span::styled(
        if is_vi {
            "Dùng ↑/↓ hoặc j/k để di chuyển, Enter để chọn."
        } else {
            "Use ↑/↓ or j/k to navigate, Enter to select."
        },
        Style::default().fg(Color::Rgb(98, 114, 164)),
    )]));

    let block = Block::default()
        .title(Span::styled(
            if is_vi {
                " 🌎 THIẾT LẬP NGÔN NGỮ "
            } else {
                " 🌎 LANGUAGE CONFIGURATION "
            },
            Style::default()
                .fg(Color::Rgb(189, 147, 249))
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Rgb(189, 147, 249)))
        .border_type(BorderType::Double);

    let paragraph = Paragraph::new(content)
        .alignment(ratatui::layout::Alignment::Center)
        .block(block);

    f.render_widget(paragraph, area);
}

pub fn render_revert_confirm(f: &mut Frame, app: &App, path: &str, area: Rect) {
    let is_vi = app.current_lang == "vi";
    f.render_widget(Clear, area);

    let content = if is_vi {
        vec![
            Line::from(""),
            Line::from(vec![Span::styled(
                "⚠️  CẢNH BÁO KHÔI PHỤC HỆ THỐNG  ⚠️",
                Style::default()
                    .fg(Color::Rgb(255, 85, 85))
                    .add_modifier(Modifier::BOLD),
            )]),
            Line::from(""),
            Line::from(vec![Span::styled(
                "Bạn có chắc chắn muốn khôi phục/xóa các thay đổi trong:",
                Style::default().fg(Color::Rgb(248, 248, 242)),
            )]),
            Line::from(vec![Span::styled(
                format!("👉 {} ", path),
                Style::default()
                    .fg(Color::Rgb(139, 233, 253))
                    .add_modifier(Modifier::BOLD),
            )]),
            Line::from(""),
            Line::from(vec![Span::styled(
                "⚠️ HÀNH ĐỘNG NÀY KHÔNG THỂ HOÀN TÁC!",
                Style::default()
                    .fg(Color::Rgb(255, 85, 85))
                    .add_modifier(Modifier::BOLD),
            )]),
            Line::from(""),
            Line::from(vec![
                Span::styled(
                    " [y] ĐỒNG Ý ",
                    Style::default()
                        .fg(Color::Rgb(248, 248, 242))
                        .bg(Color::Rgb(80, 250, 123))
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled("      ", Style::default()),
                Span::styled(
                    " [n] HỦY BỎ ",
                    Style::default()
                        .fg(Color::Rgb(248, 248, 242))
                        .bg(Color::Rgb(255, 85, 85))
                        .add_modifier(Modifier::BOLD),
                ),
            ]),
        ]
    } else {
        vec![
            Line::from(""),
            Line::from(vec![Span::styled(
                "⚠️  SYSTEM REVERT WARNING  ⚠️",
                Style::default()
                    .fg(Color::Rgb(255, 85, 85))
                    .add_modifier(Modifier::BOLD),
            )]),
            Line::from(""),
            Line::from(vec![Span::styled(
                "Are you sure you want to revert/delete changes in:",
                Style::default().fg(Color::Rgb(248, 248, 242)),
            )]),
            Line::from(vec![Span::styled(
                format!("👉 {} ", path),
                Style::default()
                    .fg(Color::Rgb(139, 233, 253))
                    .add_modifier(Modifier::BOLD),
            )]),
            Line::from(""),
            Line::from(vec![Span::styled(
                "⚠️ THIS ACTION CANNOT BE UNDONE!",
                Style::default()
                    .fg(Color::Rgb(255, 85, 85))
                    .add_modifier(Modifier::BOLD),
            )]),
            Line::from(""),
            Line::from(vec![
                Span::styled(
                    " [y] CONFIRM ",
                    Style::default()
                        .fg(Color::Rgb(248, 248, 242))
                        .bg(Color::Rgb(80, 250, 123))
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled("      ", Style::default()),
                Span::styled(
                    " [n] CANCEL ",
                    Style::default()
                        .fg(Color::Rgb(248, 248, 242))
                        .bg(Color::Rgb(255, 85, 85))
                        .add_modifier(Modifier::BOLD),
                ),
            ]),
        ]
    };

    let block = Block::default()
        .title(Span::styled(
            " WARNING CONFIRMATION ",
            Style::default()
                .fg(Color::Rgb(255, 85, 85))
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Rgb(255, 85, 85)))
        .border_type(BorderType::Double);

    let paragraph = Paragraph::new(content)
        .alignment(ratatui::layout::Alignment::Center)
        .block(block);

    f.render_widget(paragraph, area);
}

pub fn render_git_log(f: &mut Frame, app: &App, area: Rect) {
    let is_vi = app.current_lang == "vi";
    f.render_widget(Clear, area);

    let mut content = vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            if is_vi {
                "🌿 LỊCH SỬ COMMIT WORKSPACE 🌿"
            } else {
                "🌿 WORKSPACE COMMIT HISTORY 🌿"
            },
            Style::default()
                .fg(Color::Rgb(80, 250, 123))
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
    ];

    if app.commit_logs.is_empty() {
        content.push(Line::from(vec![Span::styled(
            if is_vi {
                "Không tìm thấy commit nào."
            } else {
                "No commits found."
            },
            Style::default()
                .fg(Color::Rgb(98, 114, 164))
                .add_modifier(Modifier::ITALIC),
        )]));
    } else {
        for (i, entry) in app.commit_logs.iter().enumerate() {
            let is_selected = i == app.selected_log_index;
            let bullet = if is_selected {
                Span::styled(
                    "  ▶ ● ",
                    Style::default()
                        .fg(Color::Rgb(189, 147, 249))
                        .add_modifier(Modifier::BOLD),
                )
            } else {
                Span::styled("    ● ", Style::default().fg(Color::Rgb(98, 114, 164)))
            };

            let hash_span = Span::styled(
                format!("[{}]", entry.hash),
                Style::default()
                    .fg(Color::Rgb(241, 250, 140))
                    .add_modifier(Modifier::BOLD),
            );

            let author_span = Span::styled(
                format!(" ({})", entry.author),
                Style::default().fg(Color::Rgb(255, 121, 198)),
            );

            let time_span = Span::styled(
                format!(" - {}", entry.time),
                Style::default()
                    .fg(Color::Rgb(139, 233, 253))
                    .add_modifier(Modifier::ITALIC),
            );

            let subject_style = if is_selected {
                Style::default()
                    .fg(Color::Rgb(248, 248, 242))
                    .bg(Color::Rgb(68, 71, 90))
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Rgb(248, 248, 242))
            };
            let subject_span = Span::styled(format!(" : {}", entry.subject), subject_style);

            content.push(Line::from(vec![
                bullet,
                hash_span,
                author_span,
                time_span,
                subject_span,
            ]));

            if i < app.commit_logs.len() - 1 {
                content.push(Line::from(vec![Span::styled(
                    "    │",
                    Style::default().fg(Color::Rgb(98, 114, 164)),
                )]));
            }
        }
    }

    content.push(Line::from(""));
    content.push(Line::from(vec![Span::styled(
        if is_vi {
            "   Dùng ↑/↓ hoặc j/k để di chuyển, nhấn [Esc] hoặc [q] để ĐÓNG."
        } else {
            "   Use ↑/↓ or j/k to navigate, press [Esc] or [q] to CLOSE."
        },
        Style::default()
            .fg(Color::Rgb(255, 184, 108))
            .add_modifier(Modifier::BOLD),
    )]));

    let block = Block::default()
        .title(Span::styled(
            if is_vi {
                " 🌿 LỊCH SỬ COMMIT "
            } else {
                " 🌿 COMMIT LOGS "
            },
            Style::default()
                .fg(Color::Rgb(80, 250, 123))
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Rgb(80, 250, 123)))
        .border_type(BorderType::Double);

    let paragraph = Paragraph::new(content)
        .alignment(ratatui::layout::Alignment::Left)
        .block(block);

    f.render_widget(paragraph, area);
}

pub fn render_branch_select(f: &mut Frame, app: &App, area: Rect) {
    let is_vi = app.current_lang == "vi";
    f.render_widget(Clear, area);

    let mut content = vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            if is_vi {
                "🌿 DANH SÁCH CHI NHÁNH GIT (BRANCHES) 🌿"
            } else {
                "🌿 GIT BRANCH SELECTOR 🌿"
            },
            Style::default()
                .fg(Color::Rgb(139, 233, 253))
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
    ];

    if app.branches.is_empty() {
        content.push(Line::from(vec![Span::styled(
            if is_vi {
                "Không tìm thấy chi nhánh nào."
            } else {
                "No branches found."
            },
            Style::default()
                .fg(Color::Rgb(98, 114, 164))
                .add_modifier(Modifier::ITALIC),
        )]));
    } else {
        for (i, branch) in app.branches.iter().enumerate() {
            let is_selected = i == app.selected_branch_index;
            let is_active = branch == &app.current_branch;
            let cursor = if is_selected {
                Span::styled(
                    " ▶ ",
                    Style::default()
                        .fg(Color::Rgb(189, 147, 249))
                        .add_modifier(Modifier::BOLD),
                )
            } else {
                Span::styled("   ", Style::default())
            };

            let branch_style = if is_selected {
                Style::default()
                    .fg(Color::Rgb(248, 248, 242))
                    .bg(Color::Rgb(68, 71, 90))
                    .add_modifier(Modifier::BOLD)
            } else if is_active {
                Style::default()
                    .fg(Color::Rgb(80, 250, 123))
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Rgb(248, 248, 242))
            };

            let active_badge = if is_active {
                Span::styled(
                    if is_vi {
                        " (Đang hoạt động) "
                    } else {
                        " (Active) "
                    },
                    Style::default()
                        .fg(Color::Rgb(80, 250, 123))
                        .add_modifier(Modifier::ITALIC),
                )
            } else {
                Span::styled("", Style::default())
            };

            let prefix = if is_active { "★ " } else { "☆ " };
            let prefix_span = Span::styled(
                prefix,
                if is_active {
                    Style::default().fg(Color::Rgb(80, 250, 123))
                } else {
                    Style::default().fg(Color::Rgb(98, 114, 164))
                },
            );

            content.push(Line::from(vec![
                cursor,
                prefix_span,
                Span::styled(branch.clone(), branch_style),
                active_badge,
            ]));
        }
    }

    content.push(Line::from(""));
    content.push(Line::from(vec![Span::styled(
        if is_vi {
            "Dùng ↑/↓ hoặc j/k để di chuyển, [Enter] để chuyển nhánh."
        } else {
            "Use ↑/↓ or j/k to navigate, [Enter] to checkout branch."
        },
        Style::default()
            .fg(Color::Rgb(255, 184, 108))
            .add_modifier(Modifier::BOLD),
    )]));
    content.push(Line::from(vec![Span::styled(
        if is_vi {
            "Nhấn [m] để merge chi nhánh được chọn vào chi nhánh hiện tại."
        } else {
            "Press [m] to merge selected branch into current branch."
        },
        Style::default()
            .fg(Color::Rgb(80, 250, 123))
            .add_modifier(Modifier::BOLD),
    )]));
    content.push(Line::from(vec![Span::styled(
        if is_vi {
            "Nhấn [Esc] hoặc [q] để HỦY."
        } else {
            "Press [Esc] or [q] to CANCEL."
        },
        Style::default().fg(Color::Rgb(98, 114, 164)),
    )]));

    let block = Block::default()
        .title(Span::styled(
            if is_vi {
                " 🌿 CHỌN CHI NHÁNH "
            } else {
                " 🌿 SELECT BRANCH "
            },
            Style::default()
                .fg(Color::Rgb(139, 233, 253))
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Rgb(139, 233, 253)))
        .border_type(BorderType::Double);

    let paragraph = Paragraph::new(content)
        .alignment(ratatui::layout::Alignment::Center)
        .block(block);

    f.render_widget(paragraph, area);
}

pub fn render_diff_result(f: &mut Frame, app: &App, area: Rect) {
    let is_vi = app.current_lang == "vi";
    f.render_widget(Clear, area);

    let mut content = vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            if is_vi {
                "🤖 SNAPSHOT DIFF ĐÃ COPY VÀO CLIPBOARD 🤖"
            } else {
                "🤖 DIFF SNAPSHOT COPIED TO CLIPBOARD 🤖"
            },
            Style::default()
                .fg(Color::Rgb(139, 233, 253))
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "  ➕ ",
                Style::default()
                    .fg(Color::Rgb(80, 250, 123))
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!("{} lines added", app.diff_added_lines),
                Style::default()
                    .fg(Color::Rgb(80, 250, 123))
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "     ➖ ",
                Style::default()
                    .fg(Color::Rgb(255, 85, 85))
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!("{} lines removed", app.diff_removed_lines),
                Style::default()
                    .fg(Color::Rgb(255, 85, 85))
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            if is_vi {
                "  ── PREVIEW (40 dòng đầu) ──"
            } else {
                "  ── DIFF PREVIEW (first 40 lines) ──"
            },
            Style::default()
                .fg(Color::Rgb(98, 114, 164))
                .add_modifier(Modifier::ITALIC),
        )]),
        Line::from(""),
    ];

    for line in app.diff_snapshot.lines().take(30) {
        let (styled_line, color) = if line.starts_with('+') && !line.starts_with("+++") {
            (line, Color::Rgb(80, 250, 123))
        } else if line.starts_with('-') && !line.starts_with("---") {
            (line, Color::Rgb(255, 85, 85))
        } else if line.starts_with("@@") {
            (line, Color::Rgb(139, 233, 253))
        } else if line.starts_with("diff ") || line.starts_with("index ") {
            (line, Color::Rgb(189, 147, 249))
        } else {
            (line, Color::Rgb(98, 114, 164))
        };
        content.push(Line::from(vec![Span::styled(
            format!("  {}", styled_line),
            Style::default().fg(color),
        )]));
    }

    if app.diff_snapshot.lines().count() > 30 {
        content.push(Line::from(vec![Span::styled(
            if is_vi {
                "  ... (còn nhiều hơn, xem trong AI)"
            } else {
                "  ... (more in AI clipboard)"
            },
            Style::default()
                .fg(Color::Rgb(255, 184, 108))
                .add_modifier(Modifier::ITALIC),
        )]));
    }

    content.push(Line::from(""));
    content.push(Line::from(vec![
        Span::styled(
            "  ✅ ",
            Style::default()
                .fg(Color::Rgb(80, 250, 123))
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            if is_vi {
                "Prompt + Diff đã được copy! Dán vào AI ngay. 🚀"
            } else {
                "Prompt + Diff copied! Paste into your AI now. 🚀"
            },
            Style::default()
                .fg(Color::Rgb(241, 250, 140))
                .add_modifier(Modifier::BOLD),
        ),
    ]));
    content.push(Line::from(""));
    content.push(Line::from(vec![Span::styled(
        if is_vi {
            "  Nhấn [Enter] hoặc [Esc] để đóng."
        } else {
            "  Press [Enter] or [Esc] to close."
        },
        Style::default().fg(Color::Rgb(98, 114, 164)),
    )]));

    let block = Block::default()
        .title(Span::styled(
            if is_vi {
                " 🤖 AI DIFF SNAPSHOT "
            } else {
                " 🤖 AI DIFF SNAPSHOT "
            },
            Style::default()
                .fg(Color::Rgb(139, 233, 253))
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Rgb(139, 233, 253)))
        .border_type(BorderType::Double);

    let paragraph = Paragraph::new(content)
        .alignment(ratatui::layout::Alignment::Left)
        .block(block);

    f.render_widget(paragraph, area);
}

pub fn render_go_confirm(f: &mut Frame, app: &App, area: Rect) {
    let is_vi = app.current_lang == "vi";
    f.render_widget(Clear, area);

    let content = match &app.go_step {
        GoStep::Confirm => {
            let msg_lines: Vec<&str> = app.commit_message_preview.lines().take(3).collect();
            let msg_preview = msg_lines.join(" | ");
            let msg_truncated = if msg_preview.len() > 80 {
                format!("{}...", &msg_preview[..77])
            } else {
                msg_preview
            };

            let mut lines = vec![
                Line::from(""),
                Line::from(vec![Span::styled(
                    if is_vi {
                        "🚀 XÁC NHẬN ĐÓNG GÓI COMMIT & PUSH 🚀"
                    } else {
                        "🚀 CONFIRM COMMIT & PUSH 🚀"
                    },
                    Style::default()
                        .fg(Color::Rgb(80, 250, 123))
                        .add_modifier(Modifier::BOLD),
                )]),
                Line::from(""),
            ];

            if app.staged_count > 0 {
                lines.push(Line::from(vec![Span::styled(
                    if is_vi {
                        "📂 Các file bạn đã chọn để commit:"
                    } else {
                        "📂 Selected files to commit:"
                    },
                    Style::default()
                        .fg(Color::Rgb(139, 233, 253))
                        .add_modifier(Modifier::BOLD),
                )]));
                for file in &app.files {
                    let first_char = file.status.chars().next().unwrap_or(' ');
                    if first_char != ' ' && first_char != '?' {
                        lines.push(Line::from(vec![
                            Span::styled(
                                "   🟢 ",
                                Style::default().fg(Color::Rgb(80, 250, 123)),
                            ),
                            Span::styled(
                                file.path.clone(),
                                Style::default().fg(Color::Rgb(248, 248, 242)),
                            ),
                        ]));
                    }
                }
            } else {
                lines.push(Line::from(vec![
                    Span::styled(
                        if is_vi { "⚠️ CẢNH BÁO: Chưa chọn file nào! Vui lòng thoát ra nhấn phím [Space] để chọn." }
                        else { "⚠️ WARNING: No files selected! Please exit and press [Space] to select." },
                        Style::default().fg(Color::Rgb(255, 85, 85)).add_modifier(Modifier::BOLD)
                    )
                ]));
            }

            lines.extend(vec![
                Line::from(""),
                Line::from(vec![Span::styled(
                    if is_vi {
                        "📋 Commit message từ Clipboard:"
                    } else {
                        "📋 Commit message from Clipboard:"
                    },
                    Style::default()
                        .fg(Color::Rgb(98, 114, 164))
                        .add_modifier(Modifier::ITALIC),
                )]),
                Line::from(vec![Span::styled(
                    format!("  💬 {}", msg_truncated),
                    Style::default()
                        .fg(Color::Rgb(248, 248, 242))
                        .bg(Color::Rgb(40, 42, 54))
                        .add_modifier(Modifier::BOLD),
                )]),
                Line::from(""),
                Line::from(vec![Span::styled(
                    if is_vi {
                        "  ⚡ Tiến trình: git commit -> git push"
                    } else {
                        "  ⚡ Execution: git commit -> git push"
                    },
                    Style::default().fg(Color::Rgb(255, 184, 108)),
                )]),
                Line::from(""),
            ]);

            if app.staged_count > 0 {
                lines.push(Line::from(vec![
                    Span::styled(
                        " [y] / Enter ",
                        Style::default()
                            .fg(Color::Rgb(40, 42, 54))
                            .bg(Color::Rgb(80, 250, 123))
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        " TIẾN HÀNH          ",
                        Style::default()
                            .fg(Color::Rgb(80, 250, 123))
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        " [n] / Esc ",
                        Style::default()
                            .fg(Color::Rgb(248, 248, 242))
                            .bg(Color::Rgb(255, 85, 85))
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        " HỦY ",
                        Style::default()
                            .fg(Color::Rgb(255, 85, 85))
                            .add_modifier(Modifier::BOLD),
                    ),
                ]));
            } else {
                lines.push(Line::from(vec![
                    Span::styled(
                        " [Esc] ",
                        Style::default()
                            .fg(Color::Rgb(248, 248, 242))
                            .bg(Color::Rgb(255, 85, 85))
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        " QUAY LẠI CHỌN FILE ",
                        Style::default()
                            .fg(Color::Rgb(255, 85, 85))
                            .add_modifier(Modifier::BOLD),
                    ),
                ]));
            }
            lines.push(Line::from(""));
            lines
        }
        GoStep::Pushing => {
            vec![
                Line::from(""),
                Line::from(vec![Span::styled(
                    if is_vi {
                        "⚡ ĐANG XỬ LÝ... ⚡"
                    } else {
                        "⚡ PROCESSING... ⚡"
                    },
                    Style::default()
                        .fg(Color::Rgb(241, 250, 140))
                        .add_modifier(Modifier::BOLD | Modifier::SLOW_BLINK),
                )]),
                Line::from(""),
                Line::from(vec![Span::styled(
                    if is_vi {
                        "  🔄 Đang chạy: git commit → git push"
                    } else {
                        "  🔄 Running: git commit → git push"
                    },
                    Style::default().fg(Color::Rgb(139, 233, 253)),
                )]),
                Line::from(""),
                Line::from(vec![Span::styled(
                    if is_vi {
                        "  Vui lòng chờ..."
                    } else {
                        "  Please wait..."
                    },
                    Style::default()
                        .fg(Color::Rgb(98, 114, 164))
                        .add_modifier(Modifier::ITALIC),
                )]),
                Line::from(""),
            ]
        }
        GoStep::Done(result) => {
            let result_color = if result.starts_with("✅") {
                Color::Rgb(80, 250, 123)
            } else {
                Color::Rgb(255, 85, 85)
            };
            let mut lines = vec![
                Line::from(""),
                Line::from(vec![Span::styled(
                    if is_vi {
                        "📋 KẾT QUẢ"
                    } else {
                        "📋 RESULT"
                    },
                    Style::default()
                        .fg(Color::Rgb(189, 147, 249))
                        .add_modifier(Modifier::BOLD),
                )]),
                Line::from(""),
            ];
            for l in result.lines() {
                lines.push(Line::from(vec![Span::styled(
                    format!("  {}", l),
                    Style::default()
                        .fg(result_color)
                        .add_modifier(Modifier::BOLD),
                )]));
            }
            lines.push(Line::from(""));
            lines.push(Line::from(vec![Span::styled(
                if is_vi {
                    "  Nhấn [Enter] hoặc [Esc] để đóng và làm mới."
                } else {
                    "  Press [Enter] or [Esc] to close and refresh."
                },
                Style::default().fg(Color::Rgb(98, 114, 164)),
            )]));
            lines
        }
    };

    let (title, border_color) = match &app.go_step {
        GoStep::Confirm => (
            if is_vi {
                " 🚀 COMMIT & PUSH "
            } else {
                " 🚀 COMMIT & PUSH "
            },
            Color::Rgb(80, 250, 123),
        ),
        GoStep::Pushing => (
            if is_vi {
                " ⚡ ĐANG TIẾN HÀNH "
            } else {
                " ⚡ PROCESSING "
            },
            Color::Rgb(241, 250, 140),
        ),
        GoStep::Done(r) => (
            if r.starts_with("✅") {
                if is_vi {
                    " ✅ THÀNH CÔNG "
                } else {
                    " ✅ SUCCESS "
                }
            } else {
                if is_vi {
                    " ❌ THẤT BẠI "
                } else {
                    " ❌ FAILED "
                }
            },
            if r.starts_with("✅") {
                Color::Rgb(80, 250, 123)
            } else {
                Color::Rgb(255, 85, 85)
            },
        ),
    };

    let block = Block::default()
        .title(Span::styled(
            title,
            Style::default()
                .fg(border_color)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color))
        .border_type(BorderType::Double);

    let paragraph = Paragraph::new(content)
        .alignment(ratatui::layout::Alignment::Left)
        .block(block);

    f.render_widget(paragraph, area);
}

pub fn render_stash_list(f: &mut Frame, app: &App, area: Rect) {
    let is_vi = app.current_lang == "vi";
    f.render_widget(Clear, area);

    let mut content = vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            if is_vi {
                "📦 QUẢN LÝ STASH — GIT STASH MANAGER"
            } else {
                "📦 GIT STASH MANAGER"
            },
            Style::default()
                .fg(Color::Rgb(255, 184, 108))
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
    ];

    match &app.stash_step {
        StashStep::List => {
            if app.stash_entries.is_empty() {
                content.push(Line::from(vec![Span::styled(
                    if is_vi {
                        "  (Không có stash nào)"
                    } else {
                        "  (No stashes found)"
                    },
                    Style::default()
                        .fg(Color::Rgb(98, 114, 164))
                        .add_modifier(Modifier::ITALIC),
                )]));
                content.push(Line::from(""));
                content.push(Line::from(vec![Span::styled(
                    if is_vi {
                        "  Nhấn [n] để stash thay đổi hiện tại"
                    } else {
                        "  Press [n] to stash current changes"
                    },
                    Style::default().fg(Color::Rgb(139, 233, 253)),
                )]));
            } else {
                for (i, entry) in app.stash_entries.iter().enumerate() {
                    let is_sel = i == app.selected_stash_index;
                    let cursor = if is_sel { " ▶ " } else { "   " };
                    let row_style = if is_sel {
                        Style::default()
                            .fg(Color::Rgb(248, 248, 242))
                            .bg(Color::Rgb(68, 71, 90))
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::Rgb(248, 248, 242))
                    };
                    content.push(Line::from(vec![
                        Span::styled(
                            cursor,
                            Style::default()
                                .fg(Color::Rgb(255, 184, 108))
                                .add_modifier(Modifier::BOLD),
                        ),
                        Span::styled(
                            format!("[{}] ", entry.index),
                            Style::default()
                                .fg(Color::Rgb(189, 147, 249))
                                .add_modifier(Modifier::BOLD),
                        ),
                        Span::styled(
                            format!("({}) ", entry.branch),
                            Style::default().fg(Color::Rgb(139, 233, 253)),
                        ),
                        Span::styled(entry.message.clone(), row_style),
                    ]));
                }
                content.push(Line::from(""));
                content.push(Line::from(vec![Span::styled(
                    if is_vi {
                        "  [n] Stash mới  [Enter/p] Pop  [a] Apply  [x] Xóa  [Esc] Đóng"
                    } else {
                        "  [n] New Stash  [Enter/p] Pop  [a] Apply  [x] Drop  [Esc] Close"
                    },
                    Style::default()
                        .fg(Color::Rgb(255, 184, 108))
                        .add_modifier(Modifier::BOLD),
                )]));
            }
        }
        StashStep::Confirm(idx, action) => {
            let action_str = match action {
                StashAction::Pop => {
                    if is_vi {
                        "POP (apply + xóa)"
                    } else {
                        "POP (apply + drop)"
                    }
                }
                StashAction::Apply => {
                    if is_vi {
                        "APPLY (giữ lại stash)"
                    } else {
                        "APPLY (keep stash)"
                    }
                }
                StashAction::Drop => {
                    if is_vi {
                        "XÓA stash"
                    } else {
                        "DROP stash"
                    }
                }
            };
            let action_color = match action {
                StashAction::Drop => Color::Rgb(255, 85, 85),
                _ => Color::Rgb(80, 250, 123),
            };
            content.push(Line::from(vec![Span::styled(
                format!("  ⚠️  Xác nhận {} stash@{{{}}}?", action_str, idx),
                Style::default()
                    .fg(action_color)
                    .add_modifier(Modifier::BOLD),
            )]));
            content.push(Line::from(""));
            content.push(Line::from(vec![
                Span::styled(
                    " [y] XÁC NHẬN ",
                    Style::default()
                        .fg(Color::Rgb(40, 42, 54))
                        .bg(action_color)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled("    ", Style::default()),
                Span::styled(
                    " [n/Esc] HỦY ",
                    Style::default()
                        .fg(Color::Rgb(248, 248, 242))
                        .bg(Color::Rgb(68, 71, 90))
                        .add_modifier(Modifier::BOLD),
                ),
            ]));
        }
    }

    let block = Block::default()
        .title(Span::styled(
            if is_vi {
                " 📦 STASH MANAGER "
            } else {
                " 📦 STASH MANAGER "
            },
            Style::default()
                .fg(Color::Rgb(255, 184, 108))
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Rgb(255, 184, 108)))
        .border_type(BorderType::Double);

    let paragraph = Paragraph::new(content)
        .alignment(ratatui::layout::Alignment::Left)
        .block(block);
    f.render_widget(paragraph, area);
}

pub fn render_remote_info(f: &mut Frame, app: &App, area: Rect) {
    let is_vi = app.current_lang == "vi";
    f.render_widget(Clear, area);

    let ahead_color = if app.ahead_count > 0 {
        Color::Rgb(80, 250, 123)
    } else {
        Color::Rgb(98, 114, 164)
    };
    let behind_color = if app.behind_count > 0 {
        Color::Rgb(255, 85, 85)
    } else {
        Color::Rgb(98, 114, 164)
    };

    let content = vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            if is_vi {
                "🌐 THÔNG TIN REMOTE & TRACKING"
            } else {
                "🌐 REMOTE & UPSTREAM INFO"
            },
            Style::default()
                .fg(Color::Rgb(139, 233, 253))
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "  🌿 Branch:   ",
                Style::default().fg(Color::Rgb(98, 114, 164)),
            ),
            Span::styled(
                app.current_branch.clone(),
                Style::default()
                    .fg(Color::Rgb(80, 250, 123))
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "  🔗 Tracking: ",
                Style::default().fg(Color::Rgb(98, 114, 164)),
            ),
            Span::styled(
                app.remote_tracking.clone(),
                Style::default()
                    .fg(Color::Rgb(255, 184, 108))
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "  📡 Remote:   ",
                Style::default().fg(Color::Rgb(98, 114, 164)),
            ),
            Span::styled(
                app.remote_url.clone(),
                Style::default().fg(Color::Rgb(189, 147, 249)),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "  ↑ Ahead:  ",
                Style::default().fg(Color::Rgb(98, 114, 164)),
            ),
            Span::styled(
                format!("{} commit(s) ahead of remote", app.ahead_count),
                Style::default()
                    .fg(ahead_color)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "  ↓ Behind: ",
                Style::default().fg(Color::Rgb(98, 114, 164)),
            ),
            Span::styled(
                format!("{} commit(s) behind remote", app.behind_count),
                Style::default()
                    .fg(behind_color)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            if app.ahead_count > 0 && app.behind_count == 0 {
                if is_vi {
                    "  💡 Bạn có thể push lên remote"
                } else {
                    "  💡 You can push to remote"
                }
            } else if app.behind_count > 0 {
                if is_vi {
                    "  ⚠️  Hãy git pull trước khi push"
                } else {
                    "  ⚠️  Run git pull before pushing"
                }
            } else {
                if is_vi {
                    "  ✅ Đồng bộ với remote"
                } else {
                    "  ✅ In sync with remote"
                }
            },
            Style::default()
                .fg(Color::Rgb(241, 250, 140))
                .add_modifier(Modifier::ITALIC),
        )]),
        Line::from(""),
        Line::from(vec![Span::styled(
            if is_vi {
                "  [Esc] hoặc [Enter] để đóng"
            } else {
                "  [Esc] or [Enter] to close"
            },
            Style::default().fg(Color::Rgb(98, 114, 164)),
        )]),
    ];

    let block = Block::default()
        .title(Span::styled(
            " REMOTE INFO ",
            Style::default()
                .fg(Color::Rgb(139, 233, 253))
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Rgb(139, 233, 253)))
        .border_type(BorderType::Double);

    let paragraph = Paragraph::new(content)
        .alignment(ratatui::layout::Alignment::Left)
        .block(block);
    f.render_widget(paragraph, area);
}

pub fn render_amend_commit(f: &mut Frame, app: &App, area: Rect) {
    let is_vi = app.current_lang == "vi";
    f.render_widget(Clear, area);

    let content = match &app.amend_step {
        AmendStep::Edit => {
            let display_msg = if app.amend_message.len() > 70 {
                format!("{}...", &app.amend_message[..67])
            } else {
                app.amend_message.clone()
            };
            vec![
                Line::from(""),
                Line::from(vec![Span::styled(
                    if is_vi {
                        "✏️  SỬA COMMIT CUỐI (AMEND)"
                    } else {
                        "✏️  AMEND LAST COMMIT"
                    },
                    Style::default()
                        .fg(Color::Rgb(255, 184, 108))
                        .add_modifier(Modifier::BOLD),
                )]),
                Line::from(""),
                Line::from(vec![Span::styled(
                    if is_vi {
                        "  ⚠️  Lưu ý: Nếu đã push, cần force push sau khi amend!"
                    } else {
                        "  ⚠️  Note: If already pushed, you'll need to force push after amend!"
                    },
                    Style::default()
                        .fg(Color::Rgb(255, 85, 85))
                        .add_modifier(Modifier::ITALIC),
                )]),
                Line::from(""),
                Line::from(vec![Span::styled(
                    if is_vi {
                        "  Commit message mới (chỉnh sửa bên dưới):"
                    } else {
                        "  New commit message (edit below):"
                    },
                    Style::default().fg(Color::Rgb(98, 114, 164)),
                )]),
                Line::from(vec![
                    Span::styled("  ┌─── ", Style::default().fg(Color::Rgb(255, 184, 108))),
                    Span::styled(
                        format!("{}_", display_msg),
                        Style::default()
                            .fg(Color::Rgb(248, 248, 242))
                            .bg(Color::Rgb(40, 42, 54))
                            .add_modifier(Modifier::BOLD),
                    ),
                ]),
                Line::from(""),
                Line::from(vec![Span::styled(
                    if is_vi {
                        "  Nhập để chỉnh sửa, [Enter] để xác nhận, [Esc] để hủy"
                    } else {
                        "  Type to edit, [Enter] to confirm, [Esc] to cancel"
                    },
                    Style::default().fg(Color::Rgb(98, 114, 164)),
                )]),
            ]
        }
        AmendStep::Pushing => {
            vec![
                Line::from(""),
                Line::from(vec![Span::styled(
                    if is_vi {
                        "⚡ ĐANG AMEND..."
                    } else {
                        "⚡ AMENDING..."
                    },
                    Style::default()
                        .fg(Color::Rgb(241, 250, 140))
                        .add_modifier(Modifier::BOLD | Modifier::SLOW_BLINK),
                )]),
                Line::from(""),
            ]
        }
        AmendStep::Done(result) => {
            let color = if result.starts_with("✅") {
                Color::Rgb(80, 250, 123)
            } else {
                Color::Rgb(255, 85, 85)
            };
            vec![
                Line::from(""),
                Line::from(vec![Span::styled(
                    result.clone(),
                    Style::default().fg(color).add_modifier(Modifier::BOLD),
                )]),
                Line::from(""),
                Line::from(vec![Span::styled(
                    if is_vi {
                        "  [Enter/Esc] để đóng"
                    } else {
                        "  [Enter/Esc] to close"
                    },
                    Style::default().fg(Color::Rgb(98, 114, 164)),
                )]),
            ]
        }
    };

    let block = Block::default()
        .title(Span::styled(
            " ✏️  AMEND COMMIT ",
            Style::default()
                .fg(Color::Rgb(255, 184, 108))
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Rgb(255, 184, 108)))
        .border_type(BorderType::Double);

    let paragraph = Paragraph::new(content)
        .alignment(ratatui::layout::Alignment::Left)
        .block(block);
    f.render_widget(paragraph, area);
}

pub fn render_commit_diff(f: &mut Frame, app: &App, hash: &str, area: Rect) {
    let is_vi = app.current_lang == "vi";
    f.render_widget(Clear, area);

    let lines: Vec<&str> = app.commit_diff_content.lines().collect();
    let max_scroll = lines.len().saturating_sub(5);
    let scroll = app.commit_diff_scroll.min(max_scroll);
    let visible_lines: Vec<&str> = lines.iter().skip(scroll).take(60).cloned().collect();

    let mut content = vec![
        Line::from(vec![
            Span::styled(
                format!("  🔍 Commit: {}", hash),
                Style::default()
                    .fg(Color::Rgb(241, 250, 140))
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!("  [{}/{}]", scroll + 1, lines.len().max(1)),
                Style::default().fg(Color::Rgb(98, 114, 164)),
            ),
        ]),
        Line::from(""),
    ];

    for line in visible_lines {
        let color = if line.starts_with('+') && !line.starts_with("+++") {
            Color::Rgb(80, 250, 123)
        } else if line.starts_with('-') && !line.starts_with("---") {
            Color::Rgb(255, 85, 85)
        } else if line.starts_with("@@") {
            Color::Rgb(139, 233, 253)
        } else if line.starts_with("commit ")
            || line.starts_with("Author:")
            || line.starts_with("Date:")
        {
            Color::Rgb(189, 147, 249)
        } else if line.starts_with("diff ")
            || line.starts_with("index ")
            || line.starts_with("---")
            || line.starts_with("+++")
        {
            Color::Rgb(98, 114, 164)
        } else {
            Color::Rgb(248, 248, 242)
        };
        content.push(Line::from(vec![Span::styled(
            line.to_string(),
            Style::default().fg(color),
        )]));
    }

    content.push(Line::from(""));
    content.push(Line::from(vec![Span::styled(
        if is_vi {
            "  ↑/↓ j/k cuộn  PgUp/PgDn  [Esc/q] Quay lại lịch sử"
        } else {
            "  ↑/↓ j/k scroll  PgUp/PgDn  [Esc/q] Back to history"
        },
        Style::default()
            .fg(Color::Rgb(255, 184, 108))
            .add_modifier(Modifier::BOLD),
    )]));

    let block = Block::default()
        .title(Span::styled(
            format!(" 🔍 COMMIT DIFF — {} ", hash),
            Style::default()
                .fg(Color::Rgb(241, 250, 140))
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Rgb(241, 250, 140)))
        .border_type(BorderType::Double);

    let paragraph = Paragraph::new(content)
        .alignment(ratatui::layout::Alignment::Left)
        .block(block);
    f.render_widget(paragraph, area);
}

pub fn render_merge_confirm(f: &mut Frame, app: &App, branch_to_merge: &str, area: Rect) {
    let is_vi = app.current_lang == "vi";
    f.render_widget(Clear, area);

    let content = if is_vi {
        vec![
            Line::from(""),
            Line::from(vec![Span::styled(
                "🔀  XÁC NHẬN MERGE CHI NHÁNH  🔀",
                Style::default()
                    .fg(Color::Rgb(255, 184, 108))
                    .add_modifier(Modifier::BOLD),
            )]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Trộn chi nhánh ", Style::default().fg(Color::Rgb(248, 248, 242))),
                Span::styled(format!("\"{}\"", branch_to_merge), Style::default().fg(Color::Rgb(139, 233, 253)).add_modifier(Modifier::BOLD)),
                Span::styled(" vào ", Style::default().fg(Color::Rgb(248, 248, 242))),
                Span::styled(format!("\"{}\"", app.current_branch), Style::default().fg(Color::Rgb(80, 250, 123)).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(""),
            Line::from(vec![Span::styled(
                "⚠️ Lưu ý: Nếu xảy ra xung đột (conflict), git-ai sẽ báo lỗi",
                Style::default().fg(Color::Rgb(255, 85, 85)),
            )]),
            Line::from(vec![Span::styled(
                "và hiển thị danh sách file xung đột ngoài màn hình Workspace.",
                Style::default().fg(Color::Rgb(255, 85, 85)),
            )]),
            Line::from(""),
            Line::from(vec![
                Span::styled(
                    " [y] / Enter ĐỒNG Ý ",
                    Style::default()
                        .fg(Color::Rgb(248, 248, 242))
                        .bg(Color::Rgb(80, 250, 123))
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled("      ", Style::default()),
                Span::styled(
                    " [n] / Esc HỦY BỎ ",
                    Style::default()
                        .fg(Color::Rgb(248, 248, 242))
                        .bg(Color::Rgb(255, 85, 85))
                        .add_modifier(Modifier::BOLD),
                ),
            ]),
        ]
    } else {
        vec![
            Line::from(""),
            Line::from(vec![Span::styled(
                "🔀  CONFIRM MERGE BRANCH  🔀",
                Style::default()
                    .fg(Color::Rgb(255, 184, 108))
                    .add_modifier(Modifier::BOLD),
            )]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Merge branch ", Style::default().fg(Color::Rgb(248, 248, 242))),
                Span::styled(format!("\"{}\"", branch_to_merge), Style::default().fg(Color::Rgb(139, 233, 253)).add_modifier(Modifier::BOLD)),
                Span::styled(" into ", Style::default().fg(Color::Rgb(248, 248, 242))),
                Span::styled(format!("\"{}\"", app.current_branch), Style::default().fg(Color::Rgb(80, 250, 123)).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(""),
            Line::from(vec![Span::styled(
                "⚠️ Note: If conflicts occur, git-ai will report error",
                Style::default().fg(Color::Rgb(255, 85, 85)),
            )]),
            Line::from(vec![Span::styled(
                "and conflict files will be listed on Workspace changes panel.",
                Style::default().fg(Color::Rgb(255, 85, 85)),
            )]),
            Line::from(""),
            Line::from(vec![
                Span::styled(
                    " [y] / Enter CONFIRM ",
                    Style::default()
                        .fg(Color::Rgb(248, 248, 242))
                        .bg(Color::Rgb(80, 250, 123))
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled("      ", Style::default()),
                Span::styled(
                    " [n] / Esc CANCEL ",
                    Style::default()
                        .fg(Color::Rgb(248, 248, 242))
                        .bg(Color::Rgb(255, 85, 85))
                        .add_modifier(Modifier::BOLD),
                ),
            ]),
        ]
    };

    let block = Block::default()
        .title(Span::styled(
            if is_vi { " 🔀 XÁC NHẬN MERGE " } else { " 🔀 CONFIRM MERGE " },
            Style::default()
                .fg(Color::Rgb(255, 184, 108))
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Rgb(255, 184, 108)))
        .border_type(BorderType::Double);

    let paragraph = Paragraph::new(content)
        .alignment(ratatui::layout::Alignment::Center)
        .block(block);

    f.render_widget(paragraph, area);
}

