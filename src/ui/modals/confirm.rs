use crate::app::models::{AmendStep, GoStep, StashAction, StashStep};
use crate::app::App;

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap},
    Frame,
};
use std::process::Command;

pub fn render_language_select(f: &mut Frame, app: &App, area: Rect) {
    let theme = app.theme();
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
            Style::default().fg(theme.fg).add_modifier(Modifier::ITALIC),
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
                    .fg(theme.purple)
                    .add_modifier(Modifier::BOLD),
            )
        } else {
            Span::styled("   ", Style::default())
        };

        let active_badge = if is_currently_active {
            Span::styled(
                " (Active) ",
                Style::default()
                    .fg(theme.green)
                    .add_modifier(Modifier::ITALIC),
            )
        } else {
            Span::styled("", Style::default())
        };

        let item_style = if is_hovered {
            Style::default()
                .fg(theme.fg)
                .bg(theme.select_bg)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(theme.fg)
        };

        content.push(Line::from(vec![
            cursor,
            Span::styled(
                format!("{} ", shortcut),
                Style::default().fg(theme.cyan).add_modifier(Modifier::BOLD),
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
        Style::default().fg(theme.border),
    )]));

    let block = Block::default()
        .title(Span::styled(
            if is_vi {
                " 🌎 THIẾT LẬP NGÔN NGỮ "
            } else {
                " 🌎 LANGUAGE CONFIGURATION "
            },
            Style::default()
                .fg(theme.purple)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.purple))
        .border_type(BorderType::Double)
        .style(Style::default().bg(theme.bg));

    let paragraph = Paragraph::new(content)
        .alignment(ratatui::layout::Alignment::Center)
        .block(block);

    f.render_widget(paragraph, area);
}

pub fn render_theme_select(f: &mut Frame, app: &App, area: Rect) {
    let theme = app.theme();
    let is_vi = app.current_lang == "vi";
    f.render_widget(Clear, area);

    let mut content = vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            if is_vi {
                "Select application theme:"
            } else {
                "Chọn giao diện:"
            },
            Style::default().fg(theme.fg).add_modifier(Modifier::ITALIC),
        )]),
        Line::from(""),
    ];

    let items = vec![
        ("dark", "Dracula (Tối / Dark) 🌌", "[d]"),
        ("light", "Premium Light (Sáng / Light) ☀️", "[l]"),
        ("auto", "Auto (Light/Dark) 🌍", "[a]"),
    ];

    let active_theme = if app.is_light_theme { "light" } else { "dark" };

    for (i, (theme_code, label, shortcut)) in items.into_iter().enumerate() {
        let is_hovered = i == app.selected_theme_index;
        let is_currently_active = active_theme == theme_code;

        let cursor = if is_hovered {
            Span::styled(
                " ▶ ",
                Style::default()
                    .fg(theme.purple)
                    .add_modifier(Modifier::BOLD),
            )
        } else {
            Span::styled("   ", Style::default())
        };

        let active_badge = if is_currently_active {
            Span::styled(
                " (Active) ",
                Style::default()
                    .fg(theme.green)
                    .add_modifier(Modifier::ITALIC),
            )
        } else {
            Span::styled("", Style::default())
        };

        let item_style = if is_hovered {
            Style::default()
                .fg(theme.fg)
                .bg(theme.select_bg)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(theme.fg)
        };

        content.push(Line::from(vec![
            cursor,
            Span::styled(
                format!("{} ", shortcut),
                Style::default().fg(theme.cyan).add_modifier(Modifier::BOLD),
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
        Style::default().fg(theme.border),
    )]));

    let block = Block::default()
        .title(Span::styled(
            if is_vi {
                " 🎨 THIẾT LẬP GIAO DIỆN "
            } else {
                " 🎨 THEME CONFIGURATION "
            },
            Style::default()
                .fg(theme.purple)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.purple))
        .border_type(BorderType::Double)
        .style(Style::default().bg(theme.bg));

    let paragraph = Paragraph::new(content)
        .alignment(ratatui::layout::Alignment::Center)
        .block(block);

    f.render_widget(paragraph, area);
}

pub fn render_revert_confirm(f: &mut Frame, app: &App, path: &str, area: Rect) {
    let theme = app.theme();
    let is_vi = app.current_lang == "vi";
    f.render_widget(Clear, area);

    let content = if is_vi {
        vec![
            Line::from(""),
            Line::from(vec![Span::styled(
                "⚠️  CẢNH BÁO KHÔI PHỤC HỆ THỐNG  ⚠️",
                Style::default().fg(theme.red).add_modifier(Modifier::BOLD),
            )]),
            Line::from(""),
            Line::from(vec![Span::styled(
                "Bạn có chắc chắn muốn khôi phục/xóa các thay đổi trong:",
                Style::default().fg(theme.fg),
            )]),
            Line::from(vec![Span::styled(
                format!("👉 {} ", path),
                Style::default().fg(theme.cyan).add_modifier(Modifier::BOLD),
            )]),
            Line::from(""),
            Line::from(vec![Span::styled(
                "⚠️ HÀNH ĐỘNG NÀY KHÔNG THỂ HOÀN TÁC!",
                Style::default().fg(theme.red).add_modifier(Modifier::BOLD),
            )]),
            Line::from(""),
            Line::from(vec![
                Span::styled(
                    " [y] ĐỒNG Ý ",
                    Style::default()
                        .fg(theme.fg)
                        .bg(theme.green)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled("      ", Style::default()),
                Span::styled(
                    " [n] HỦY BỎ ",
                    Style::default()
                        .fg(theme.fg)
                        .bg(theme.red)
                        .add_modifier(Modifier::BOLD),
                ),
            ]),
        ]
    } else {
        vec![
            Line::from(""),
            Line::from(vec![Span::styled(
                "⚠️  SYSTEM REVERT WARNING  ⚠️",
                Style::default().fg(theme.red).add_modifier(Modifier::BOLD),
            )]),
            Line::from(""),
            Line::from(vec![Span::styled(
                "Are you sure you want to revert/delete changes in:",
                Style::default().fg(theme.fg),
            )]),
            Line::from(vec![Span::styled(
                format!("👉 {} ", path),
                Style::default().fg(theme.cyan).add_modifier(Modifier::BOLD),
            )]),
            Line::from(""),
            Line::from(vec![Span::styled(
                "⚠️ THIS ACTION CANNOT BE UNDONE!",
                Style::default().fg(theme.red).add_modifier(Modifier::BOLD),
            )]),
            Line::from(""),
            Line::from(vec![
                Span::styled(
                    " [y] CONFIRM ",
                    Style::default()
                        .fg(theme.fg)
                        .bg(theme.green)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled("      ", Style::default()),
                Span::styled(
                    " [n] CANCEL ",
                    Style::default()
                        .fg(theme.fg)
                        .bg(theme.red)
                        .add_modifier(Modifier::BOLD),
                ),
            ]),
        ]
    };

    let block = Block::default()
        .title(Span::styled(
            " WARNING CONFIRMATION ",
            Style::default().fg(theme.red).add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.red))
        .border_type(BorderType::Double)
        .style(Style::default().bg(theme.bg));

    let paragraph = Paragraph::new(content)
        .alignment(ratatui::layout::Alignment::Center)
        .block(block);

    f.render_widget(paragraph, area);
}

pub fn render_git_log(f: &mut Frame, app: &App, area: Rect) {
    let theme = app.theme();
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
                .fg(theme.green)
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
                .fg(theme.border)
                .add_modifier(Modifier::ITALIC),
        )]));
    } else {
        for (i, entry) in app.commit_logs.iter().enumerate() {
            let is_selected = i == app.selected_log_index;
            let bullet = if is_selected {
                Span::styled(
                    "  ▶ ● ",
                    Style::default()
                        .fg(theme.purple)
                        .add_modifier(Modifier::BOLD),
                )
            } else {
                Span::styled("    ● ", Style::default().fg(theme.border))
            };

            let hash_span = Span::styled(
                format!("[{}]", entry.hash),
                Style::default()
                    .fg(theme.yellow)
                    .add_modifier(Modifier::BOLD),
            );

            let author_span = Span::styled(
                format!(" ({})", entry.author),
                Style::default().fg(theme.purple),
            );

            let time_span = Span::styled(
                format!(" - {}", entry.time),
                Style::default()
                    .fg(theme.cyan)
                    .add_modifier(Modifier::ITALIC),
            );

            let subject_style = if is_selected {
                Style::default()
                    .fg(theme.fg)
                    .bg(theme.select_bg)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(theme.fg)
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
                    Style::default().fg(theme.border),
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
            .fg(theme.orange)
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
                .fg(theme.green)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.green))
        .border_type(BorderType::Double)
        .style(Style::default().bg(theme.bg));

    let paragraph = Paragraph::new(content)
        .alignment(ratatui::layout::Alignment::Left)
        .block(block);

    f.render_widget(paragraph, area);
}

pub fn render_branch_select(f: &mut Frame, app: &App, area: Rect) {
    let theme = app.theme();
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
            Style::default().fg(theme.cyan).add_modifier(Modifier::BOLD),
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
                .fg(theme.border)
                .add_modifier(Modifier::ITALIC),
        )]));
    } else {
        for (i, branch) in app.branches.iter().enumerate() {
            let is_selected = i == app.selected_branch_index;
            let is_active = !branch.is_remote && branch.name == app.current_branch;
            let cursor = if is_selected {
                Span::styled(
                    " ▶ ",
                    Style::default()
                        .fg(theme.purple)
                        .add_modifier(Modifier::BOLD),
                )
            } else {
                Span::styled("   ", Style::default())
            };

            let branch_style = if is_selected {
                Style::default()
                    .fg(theme.fg)
                    .bg(theme.select_bg)
                    .add_modifier(Modifier::BOLD)
            } else if is_active {
                Style::default()
                    .fg(theme.green)
                    .add_modifier(Modifier::BOLD)
            } else if branch.is_remote {
                Style::default().fg(theme.orange)
            } else {
                Style::default().fg(theme.fg)
            };

            let active_badge = if is_active {
                Span::styled(
                    if is_vi {
                        " (Đang hoạt động) "
                    } else {
                        " (Active) "
                    },
                    Style::default()
                        .fg(theme.green)
                        .add_modifier(Modifier::ITALIC),
                )
            } else if branch.is_remote {
                Span::styled(
                    " (Remote) ",
                    Style::default()
                        .fg(theme.orange)
                        .add_modifier(Modifier::ITALIC),
                )
            } else {
                Span::styled("", Style::default())
            };

            let prefix = if branch.is_remote {
                "🌍 "
            } else if is_active {
                "★ "
            } else {
                "☆ "
            };

            let prefix_span = Span::styled(
                prefix,
                if branch.is_remote {
                    Style::default().fg(theme.orange)
                } else if is_active {
                    Style::default().fg(theme.green)
                } else {
                    Style::default().fg(theme.border)
                },
            );

            content.push(Line::from(vec![
                cursor,
                prefix_span,
                Span::styled(branch.name.clone(), branch_style),
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
            .fg(theme.orange)
            .add_modifier(Modifier::BOLD),
    )]));
    content.push(Line::from(vec![Span::styled(
        if is_vi {
            "Nhấn [m] để merge chi nhánh được chọn vào chi nhánh hiện tại."
        } else {
            "Press [m] to merge selected branch into current branch."
        },
        Style::default()
            .fg(theme.green)
            .add_modifier(Modifier::BOLD),
    )]));
    content.push(Line::from(vec![Span::styled(
        if is_vi {
            "Nhấn [c] để tạo và chuyển sang chi nhánh mới (checkout -b)."
        } else {
            "Press [c] to create and checkout a new branch (checkout -b)."
        },
        Style::default()
            .fg(theme.purple)
            .add_modifier(Modifier::BOLD),
    )]));
    content.push(Line::from(vec![Span::styled(
        if is_vi {
            "Nhấn [Esc] hoặc [q] để HỦY."
        } else {
            "Press [Esc] or [q] to CANCEL."
        },
        Style::default().fg(theme.border),
    )]));

    let block = Block::default()
        .title(Span::styled(
            if is_vi {
                " 🌿 CHỌN CHI NHÁNH "
            } else {
                " 🌿 SELECT BRANCH "
            },
            Style::default().fg(theme.cyan).add_modifier(Modifier::BOLD),
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

pub fn render_diff_result(f: &mut Frame, app: &App, area: Rect) {
    let theme = app.theme();
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
            Style::default().fg(theme.cyan).add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "  ➕ ",
                Style::default()
                    .fg(theme.green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!("{} lines added", app.diff_added_lines),
                Style::default()
                    .fg(theme.green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "     ➖ ",
                Style::default().fg(theme.red).add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!("{} lines removed", app.diff_removed_lines),
                Style::default().fg(theme.red).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  🤖 Model: ", Style::default().fg(theme.border)),
            Span::styled(
                if app.current_kilo_model.is_empty() {
                    if is_vi {
                        "Mặc định (Kilo config)"
                    } else {
                        "Default (Kilo config)"
                    }
                    .to_string()
                } else {
                    app.current_kilo_model.clone()
                },
                Style::default()
                    .fg(theme.green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("   [M] Đổi", Style::default().fg(theme.cyan)),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            if is_vi {
                "  ── PREVIEW (40 dòng đầu) ──"
            } else {
                "  ── DIFF PREVIEW (first 40 lines) ──"
            },
            Style::default()
                .fg(theme.border)
                .add_modifier(Modifier::ITALIC),
        )]),
        Line::from(""),
    ];

    let preview_limit = if !app.diff_kilo_generated.is_empty() {
        6
    } else {
        22
    };

    for line in app.diff_snapshot.lines().take(preview_limit) {
        let (styled_line, color) = if line.starts_with('+') && !line.starts_with("+++") {
            (line, theme.green)
        } else if line.starts_with('-') && !line.starts_with("---") {
            (line, theme.red)
        } else if line.starts_with("@@") {
            (line, theme.cyan)
        } else if line.starts_with("diff ") || line.starts_with("index ") {
            (line, theme.purple)
        } else {
            (line, theme.fg)
        };
        content.push(Line::from(vec![Span::styled(
            format!("  {}", styled_line),
            Style::default().fg(color),
        )]));
    }

    if app.diff_snapshot.lines().count() > preview_limit {
        content.push(Line::from(vec![Span::styled(
            if is_vi {
                "  ... (diff dài, xem trong KILO)"
            } else {
                "  ... (long diff, see in KILO)"
            },
            Style::default()
                .fg(theme.orange)
                .add_modifier(Modifier::ITALIC),
        )]));
    }

    content.push(Line::from(""));
    content.push(Line::from(vec![
        Span::styled(
            "  ✅ ",
            Style::default()
                .fg(theme.green)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            if is_vi {
                "Prompt + Diff đã được copy! Dán vào AI ngay. 🚀"
            } else {
                "Prompt + Diff copied! Paste into your AI now. 🚀"
            },
            Style::default()
                .fg(theme.yellow)
                .add_modifier(Modifier::BOLD),
        ),
    ]));

    if !app.diff_kilo_generated.is_empty() {
        content.push(Line::from(""));
        content.push(Line::from(vec![Span::styled(
            if is_vi {
                "  🤖 KILO ĐÃ SINH COMMIT MESSAGE:"
            } else {
                "  🤖 KILO GENERATED COMMIT MESSAGE:"
            },
            Style::default()
                .fg(theme.green)
                .add_modifier(Modifier::BOLD),
        )]));
        for line in app.diff_kilo_generated.lines().take(11) {
            content.push(Line::from(vec![Span::styled(
                format!("    {}", line),
                Style::default().fg(theme.fg),
            )]));
        }
        if app.diff_kilo_generated.lines().count() > 11 {
            content.push(Line::from(vec![Span::styled(
                "    ...",
                Style::default()
                    .fg(theme.border)
                    .add_modifier(Modifier::ITALIC),
            )]));
        }
        content.push(Line::from(""));
        content.push(Line::from(vec![Span::styled(
            if is_vi {
                "  [C] Copy  [Enter/G] Dùng message này  [Esc] Đóng"
            } else {
                "  [C] Copy  [Enter/G] Use this message  [Esc] Close"
            },
            Style::default().fg(theme.cyan).add_modifier(Modifier::BOLD),
        )]));
    } else {
        content.push(Line::from(""));
        if app.kilo_generating {
            content.push(Line::from(vec![Span::styled(
                if is_vi {
                    "  ⏳ ĐANG HỎI KILO..."
                } else {
                    "  ⏳ ASKING KILO..."
                },
                Style::default()
                    .fg(theme.yellow)
                    .add_modifier(Modifier::BOLD),
            )]));
            if !app.kilo_generation_status.is_empty() {
                content.push(Line::from(vec![Span::styled(
                    format!("  {}", app.kilo_generation_status),
                    Style::default().fg(theme.fg),
                )]));
            }
        } else if !app.diff_kilo_generated.is_empty() {
            // already handled above
        } else {
            content.push(Line::from(vec![Span::styled(
                if is_vi {
                    "  Nhấn [K] để KILO sinh commit message trực tiếp!"
                } else {
                    "  Press [K] to let KILO generate the commit message!"
                },
                Style::default()
                    .fg(theme.purple)
                    .add_modifier(Modifier::BOLD),
            )]));
        }
    }

    content.push(Line::from(""));
    content.push(Line::from(vec![Span::styled(
        if is_vi {
            "  Nhấn [Enter] hoặc [Esc] để đóng."
        } else {
            "  Press [Enter] or [Esc] to close."
        },
        Style::default().fg(theme.border),
    )]));

    let block = Block::default()
        .title(Span::styled(
            if !app.diff_kilo_generated.is_empty() {
                if is_vi {
                    " 🤖 KILO AI COMMIT "
                } else {
                    " 🤖 KILO AI COMMIT "
                }
            } else if is_vi {
                " 🤖 AI DIFF SNAPSHOT "
            } else {
                " 🤖 AI DIFF SNAPSHOT "
            },
            Style::default().fg(theme.cyan).add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.cyan))
        .border_type(BorderType::Double)
        .style(Style::default().bg(theme.bg));

    let paragraph = Paragraph::new(content)
        .alignment(ratatui::layout::Alignment::Left)
        .block(block);

    f.render_widget(paragraph, area);
}

pub fn render_go_confirm(f: &mut Frame, app: &App, area: Rect) {
    let theme = app.theme();
    let is_vi = app.current_lang == "vi";
    f.render_widget(Clear, area);

    let content = match &app.go_step {
        GoStep::Confirm => {
            let display_msg = if app.commit_input_mode {
                &app.commit_input_text
            } else {
                &app.commit_message_preview
            };
            let msg_lines: Vec<&str> = display_msg.lines().take(10).collect();

            let mut lines = vec![
                Line::from(""),
                Line::from(vec![Span::styled(
                    if is_vi {
                        "🚀 XÁC NHẬN ĐÓNG GÓI COMMIT & PUSH 🚀"
                    } else {
                        "🚀 CONFIRM COMMIT & PUSH 🚀"
                    },
                    Style::default()
                        .fg(theme.green)
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
                    Style::default().fg(theme.cyan).add_modifier(Modifier::BOLD),
                )]));
                for file in &app.files {
                    let first_char = file.status.chars().next().unwrap_or(' ');
                    if first_char != ' ' && first_char != '?' {
                        lines.push(Line::from(vec![
                            Span::styled("   🟢 ", Style::default().fg(theme.green)),
                            Span::styled(file.path.clone(), Style::default().fg(theme.fg)),
                        ]));
                    }
                }
            } else {
                lines.push(Line::from(vec![
                    Span::styled(
                        if is_vi { "⚠️ CẢNH BÁO: Chưa chọn file nào! Vui lòng thoát ra nhấn phím [Space] để chọn." }
                        else { "⚠️ WARNING: No files selected! Please exit and press [Space] to select." },
                        Style::default().fg(theme.red).add_modifier(Modifier::BOLD)
                    )
                ]));
            }

            lines.push(Line::from(""));
            lines.push(Line::from(vec![Span::styled(
                if is_vi {
                    "📋 Commit message từ Clipboard:"
                } else {
                    "📋 Commit message from Clipboard:"
                },
                Style::default()
                    .fg(theme.border)
                    .add_modifier(Modifier::ITALIC),
            )]));

            for l in &msg_lines {
                lines.push(Line::from(vec![Span::styled(
                    format!("  {}", l),
                    Style::default().fg(theme.fg).add_modifier(Modifier::BOLD),
                )]));
            }

            lines.push(Line::from(""));
            lines.push(Line::from(vec![Span::styled(
                if is_vi {
                    "  ⚡ Tiến trình: git commit -> git push"
                } else {
                    "  ⚡ Execution: git commit -> git push"
                },
                Style::default().fg(theme.orange),
            )]));
            lines.push(Line::from(""));

            if app.staged_count > 0 {
                lines.push(Line::from(vec![
                    Span::styled(
                        " [y] / Enter ",
                        Style::default()
                            .fg(theme.bg)
                            .bg(theme.green)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        " TIẾN HÀNH          ",
                        Style::default()
                            .fg(theme.green)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        " [n] / Esc ",
                        Style::default()
                            .fg(theme.fg)
                            .bg(theme.red)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        " HỦY ",
                        Style::default().fg(theme.red).add_modifier(Modifier::BOLD),
                    ),
                ]));
            } else {
                lines.push(Line::from(vec![
                    Span::styled(
                        " [Esc] ",
                        Style::default()
                            .fg(theme.fg)
                            .bg(theme.red)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        " QUAY LẠI CHỌN FILE ",
                        Style::default().fg(theme.red).add_modifier(Modifier::BOLD),
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
                        .fg(theme.yellow)
                        .add_modifier(Modifier::BOLD | Modifier::SLOW_BLINK),
                )]),
                Line::from(""),
                Line::from(vec![Span::styled(
                    if is_vi {
                        "  🔄 Đang chạy: git commit → git push"
                    } else {
                        "  🔄 Running: git commit → git push"
                    },
                    Style::default().fg(theme.cyan),
                )]),
                Line::from(""),
                Line::from(vec![Span::styled(
                    if is_vi {
                        "  Vui lòng chờ..."
                    } else {
                        "  Please wait..."
                    },
                    Style::default()
                        .fg(theme.border)
                        .add_modifier(Modifier::ITALIC),
                )]),
                Line::from(""),
            ]
        }
        GoStep::Done(result) => {
            let result_color = if result.starts_with("✅") {
                theme.green
            } else {
                theme.red
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
                        .fg(theme.purple)
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
                Style::default().fg(theme.border),
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
            theme.green,
        ),
        GoStep::Pushing => (
            if is_vi {
                " ⚡ ĐANG TIẾN HÀNH "
            } else {
                " ⚡ PROCESSING "
            },
            theme.yellow,
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
                theme.green
            } else {
                theme.red
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
        .border_type(BorderType::Double)
        .style(Style::default().bg(theme.bg));

    let paragraph = Paragraph::new(content)
        .alignment(ratatui::layout::Alignment::Left)
        .block(block);

    f.render_widget(paragraph, area);
}

pub fn render_stash_list(f: &mut Frame, app: &App, area: Rect) {
    let theme = app.theme();
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
                .fg(theme.orange)
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
                        .fg(theme.border)
                        .add_modifier(Modifier::ITALIC),
                )]));
                content.push(Line::from(""));
                content.push(Line::from(vec![Span::styled(
                    if is_vi {
                        "  Nhấn [n] để stash thay đổi hiện tại"
                    } else {
                        "  Press [n] to stash current changes"
                    },
                    Style::default().fg(theme.cyan),
                )]));
            } else {
                for (i, entry) in app.stash_entries.iter().enumerate() {
                    let is_sel = i == app.selected_stash_index;
                    let cursor = if is_sel { " ▶ " } else { "   " };
                    let row_style = if is_sel {
                        Style::default()
                            .fg(theme.fg)
                            .bg(theme.select_bg)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(theme.fg)
                    };
                    content.push(Line::from(vec![
                        Span::styled(
                            cursor,
                            Style::default()
                                .fg(theme.orange)
                                .add_modifier(Modifier::BOLD),
                        ),
                        Span::styled(
                            format!("[{}] ", entry.index),
                            Style::default()
                                .fg(theme.purple)
                                .add_modifier(Modifier::BOLD),
                        ),
                        Span::styled(
                            format!("({}) ", entry.branch),
                            Style::default().fg(theme.cyan),
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
                        .fg(theme.orange)
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
                StashAction::Drop => theme.red,
                _ => theme.green,
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
                        .fg(theme.bg)
                        .bg(action_color)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled("    ", Style::default()),
                Span::styled(
                    " [n/Esc] HỦY ",
                    Style::default()
                        .fg(theme.fg)
                        .bg(theme.select_bg)
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
                .fg(theme.orange)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.orange))
        .border_type(BorderType::Double)
        .style(Style::default().bg(theme.bg));

    let paragraph = Paragraph::new(content)
        .alignment(ratatui::layout::Alignment::Left)
        .block(block);
    f.render_widget(paragraph, area);
}

pub fn render_remote_info(f: &mut Frame, app: &App, area: Rect) {
    let theme = app.theme();
    let is_vi = app.current_lang == "vi";
    f.render_widget(Clear, area);

    let ahead_color = if app.ahead_count > 0 {
        theme.green
    } else {
        theme.border
    };
    let behind_color = if app.behind_count > 0 {
        theme.red
    } else {
        theme.border
    };

    let mut content = vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            if is_vi {
                "🌐 THÔNG TIN REMOTE & TRACKING"
            } else {
                "🌐 REMOTE & UPSTREAM INFO"
            },
            Style::default().fg(theme.cyan).add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  🌿 Branch:   ", Style::default().fg(theme.border)),
            Span::styled(
                app.current_branch.clone(),
                Style::default()
                    .fg(theme.green)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("  🔗 Tracking: ", Style::default().fg(theme.border)),
            Span::styled(
                app.remote_tracking.clone(),
                Style::default()
                    .fg(theme.orange)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("  📡 Remote:   ", Style::default().fg(theme.border)),
            Span::styled(app.remote_url.clone(), Style::default().fg(theme.purple)),
        ]),
    ];

    content.push(Line::from(""));
    content.push(Line::from(vec![Span::styled(
        if is_vi {
            "  📋 Danh sách Remotes:"
        } else {
            "  📋 Remotes List:"
        },
        Style::default().fg(theme.cyan).add_modifier(Modifier::BOLD),
    )]));
    if app.remotes.is_empty() {
        content.push(Line::from(vec![Span::styled(
            if is_vi {
                "    (không có remote nào)"
            } else {
                "    (no remotes configured)"
            },
            Style::default()
                .fg(theme.border)
                .add_modifier(Modifier::ITALIC),
        )]));
    } else {
        for remote in &app.remotes {
            content.push(Line::from(vec![
                Span::styled("    • ", Style::default().fg(theme.border)),
                Span::styled(
                    remote.name.clone(),
                    Style::default()
                        .fg(theme.green)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(" → ", Style::default().fg(theme.yellow)),
                Span::styled(remote.url.clone(), Style::default().fg(theme.purple)),
            ]));
        }
    }

    content.push(Line::from(""));
    content.push(Line::from(vec![
        Span::styled("  ↑ Ahead:  ", Style::default().fg(theme.border)),
        Span::styled(
            format!("{} commit(s) ahead of remote", app.ahead_count),
            Style::default()
                .fg(ahead_color)
                .add_modifier(Modifier::BOLD),
        ),
    ]));
    content.push(Line::from(vec![
        Span::styled("  ↓ Behind: ", Style::default().fg(theme.border)),
        Span::styled(
            format!("{} commit(s) behind remote", app.behind_count),
            Style::default()
                .fg(behind_color)
                .add_modifier(Modifier::BOLD),
        ),
    ]));
    content.push(Line::from(""));
    content.push(Line::from(vec![Span::styled(
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
            .fg(theme.yellow)
            .add_modifier(Modifier::ITALIC),
    )]));
    content.push(Line::from(""));
    content.push(Line::from(vec![Span::styled(
        if is_vi {
            "  [Esc] hoặc [Enter] để đóng"
        } else {
            "  [Esc] or [Enter] to close"
        },
        Style::default().fg(theme.border),
    )]));

    let block = Block::default()
        .title(Span::styled(
            " REMOTE INFO ",
            Style::default().fg(theme.cyan).add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.cyan))
        .border_type(BorderType::Double)
        .style(Style::default().bg(theme.bg));

    let paragraph = Paragraph::new(content)
        .alignment(ratatui::layout::Alignment::Left)
        .block(block);
    f.render_widget(paragraph, area);
}

pub fn render_amend_commit(f: &mut Frame, app: &App, area: Rect) {
    let theme = app.theme();
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
                        .fg(theme.orange)
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
                        .fg(theme.red)
                        .add_modifier(Modifier::ITALIC),
                )]),
                Line::from(""),
                Line::from(vec![Span::styled(
                    if is_vi {
                        "  Commit message mới (chỉnh sửa bên dưới):"
                    } else {
                        "  New commit message (edit below):"
                    },
                    Style::default().fg(theme.border),
                )]),
                Line::from(vec![
                    Span::styled("  ┌─── ", Style::default().fg(theme.orange)),
                    Span::styled(
                        format!("{}_", display_msg),
                        Style::default()
                            .fg(theme.fg)
                            .bg(theme.bg)
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
                    Style::default().fg(theme.border),
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
                        .fg(theme.yellow)
                        .add_modifier(Modifier::BOLD | Modifier::SLOW_BLINK),
                )]),
                Line::from(""),
            ]
        }
        AmendStep::Done(result) => {
            let color = if result.starts_with("✅") {
                theme.green
            } else {
                theme.red
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
                    Style::default().fg(theme.border),
                )]),
            ]
        }
    };

    let block = Block::default()
        .title(Span::styled(
            " ✏️  AMEND COMMIT ",
            Style::default()
                .fg(theme.orange)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.orange))
        .border_type(BorderType::Double)
        .style(Style::default().bg(theme.bg));

    let paragraph = Paragraph::new(content)
        .alignment(ratatui::layout::Alignment::Left)
        .block(block);
    f.render_widget(paragraph, area);
}

pub fn render_commit_diff(f: &mut Frame, app: &App, hash: &str, area: Rect) {
    let theme = app.theme();
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
                    .fg(theme.yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!("  [{}/{}]", scroll + 1, lines.len().max(1)),
                Style::default().fg(theme.border),
            ),
        ]),
        Line::from(""),
    ];

    for line in visible_lines {
        let color = if line.starts_with('+') && !line.starts_with("+++") {
            theme.green
        } else if line.starts_with('-') && !line.starts_with("---") {
            theme.red
        } else if line.starts_with("@@") {
            theme.cyan
        } else if line.starts_with("commit ")
            || line.starts_with("Author:")
            || line.starts_with("Date:")
        {
            theme.purple
        } else if line.starts_with("diff ")
            || line.starts_with("index ")
            || line.starts_with("---")
            || line.starts_with("+++")
        {
            theme.border
        } else {
            theme.fg
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
            .fg(theme.orange)
            .add_modifier(Modifier::BOLD),
    )]));

    let block = Block::default()
        .title(Span::styled(
            format!(" 🔍 COMMIT DIFF — {} ", hash),
            Style::default()
                .fg(theme.yellow)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.yellow))
        .border_type(BorderType::Double)
        .style(Style::default().bg(theme.bg));

    let paragraph = Paragraph::new(content)
        .alignment(ratatui::layout::Alignment::Left)
        .block(block);
    f.render_widget(paragraph, area);
}

pub fn render_merge_confirm(f: &mut Frame, app: &App, branch_to_merge: &str, area: Rect) {
    let theme = app.theme();
    let is_vi = app.current_lang == "vi";
    f.render_widget(Clear, area);

    let content = if is_vi {
        vec![
            Line::from(""),
            Line::from(vec![Span::styled(
                "🔀  XÁC NHẬN MERGE CHI NHÁNH  🔀",
                Style::default()
                    .fg(theme.orange)
                    .add_modifier(Modifier::BOLD),
            )]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Trộn chi nhánh ", Style::default().fg(theme.fg)),
                Span::styled(
                    format!("\"{}\"", branch_to_merge),
                    Style::default().fg(theme.cyan).add_modifier(Modifier::BOLD),
                ),
                Span::styled(" vào ", Style::default().fg(theme.fg)),
                Span::styled(
                    format!("\"{}\"", app.current_branch),
                    Style::default()
                        .fg(theme.green)
                        .add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(""),
            Line::from(vec![Span::styled(
                "⚠️ Lưu ý: Nếu xảy ra xung đột (conflict), git-ai sẽ báo lỗi",
                Style::default().fg(theme.red),
            )]),
            Line::from(vec![Span::styled(
                "và hiển thị danh sách file xung đột ngoài màn hình Workspace.",
                Style::default().fg(theme.red),
            )]),
            Line::from(""),
            Line::from(vec![
                Span::styled(
                    " [y] / Enter ĐỒNG Ý ",
                    Style::default()
                        .fg(theme.fg)
                        .bg(theme.green)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled("      ", Style::default()),
                Span::styled(
                    " [n] / Esc HỦY BỎ ",
                    Style::default()
                        .fg(theme.fg)
                        .bg(theme.red)
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
                    .fg(theme.orange)
                    .add_modifier(Modifier::BOLD),
            )]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Merge branch ", Style::default().fg(theme.fg)),
                Span::styled(
                    format!("\"{}\"", branch_to_merge),
                    Style::default().fg(theme.cyan).add_modifier(Modifier::BOLD),
                ),
                Span::styled(" into ", Style::default().fg(theme.fg)),
                Span::styled(
                    format!("\"{}\"", app.current_branch),
                    Style::default()
                        .fg(theme.green)
                        .add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(""),
            Line::from(vec![Span::styled(
                "⚠️ Note: If conflicts occur, git-ai will report error",
                Style::default().fg(theme.red),
            )]),
            Line::from(vec![Span::styled(
                "and conflict files will be listed on Workspace changes panel.",
                Style::default().fg(theme.red),
            )]),
            Line::from(""),
            Line::from(vec![
                Span::styled(
                    " [y] / Enter CONFIRM ",
                    Style::default()
                        .fg(theme.fg)
                        .bg(theme.green)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled("      ", Style::default()),
                Span::styled(
                    " [n] / Esc CANCEL ",
                    Style::default()
                        .fg(theme.fg)
                        .bg(theme.red)
                        .add_modifier(Modifier::BOLD),
                ),
            ]),
        ]
    };

    let block = Block::default()
        .title(Span::styled(
            if is_vi {
                " 🔀 XÁC NHẬN MERGE "
            } else {
                " 🔀 CONFIRM MERGE "
            },
            Style::default()
                .fg(theme.orange)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.orange))
        .border_type(BorderType::Double)
        .style(Style::default().bg(theme.bg));

    let paragraph = Paragraph::new(content)
        .alignment(ratatui::layout::Alignment::Center)
        .block(block);

    f.render_widget(paragraph, area);
}

pub fn render_new_branch_input(f: &mut Frame, app: &App, area: Rect) {
    let theme = app.theme();
    let is_vi = app.current_lang == "vi";
    f.render_widget(Clear, area);

    let display_msg = if app.new_branch_name.len() > 70 {
        format!("{}...", &app.new_branch_name[..67])
    } else {
        app.new_branch_name.clone()
    };

    let content = vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            if is_vi {
                "🌿 TẠO CHI NHÁNH MỚI (CHECKOUT -B)"
            } else {
                "🌿 CREATE & CHECKOUT NEW BRANCH"
            },
            Style::default()
                .fg(theme.purple)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(vec![Span::styled(
            if is_vi {
                "  Nhập tên chi nhánh mới bên dưới:"
            } else {
                "  Enter new branch name below:"
            },
            Style::default().fg(theme.border),
        )]),
        Line::from(vec![
            Span::styled("  ┌─── ", Style::default().fg(theme.purple)),
            Span::styled(
                format!("{}_", display_msg),
                Style::default()
                    .fg(theme.fg)
                    .bg(theme.bg)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            if is_vi {
                "  Nhập tên, [Enter] để xác nhận, [Esc] để hủy"
            } else {
                "  Type name, [Enter] to confirm, [Esc] to cancel"
            },
            Style::default().fg(theme.border),
        )]),
    ];

    let block = Block::default()
        .title(Span::styled(
            if is_vi {
                " 🌿 CHI NHÁNH MỚI "
            } else {
                " 🌿 NEW BRANCH "
            },
            Style::default()
                .fg(theme.purple)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.purple))
        .border_type(BorderType::Double)
        .style(Style::default().bg(theme.bg));

    let paragraph = Paragraph::new(content)
        .alignment(ratatui::layout::Alignment::Left)
        .block(block);
    f.render_widget(paragraph, area);
}

pub fn render_workspace_history(f: &mut Frame, app: &App, area: Rect) {
    let theme = app.theme();
    let is_vi = app.current_lang == "vi";
    f.render_widget(Clear, area);

    let mut content = vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            if is_vi {
                "📂 LỊCH SỬ WORKSPACE — CHỌN NHANH PROJECT"
            } else {
                "📂 WORKSPACE HISTORY — QUICK PROJECT SWITCH"
            },
            Style::default().fg(theme.cyan).add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
    ];

    if app.workspace_history.is_empty() {
        content.push(Line::from(vec![Span::styled(
            if is_vi {
                "  (Chưa có project nào trong lịch sử)"
            } else {
                "  (No projects in history yet)"
            },
            Style::default()
                .fg(theme.border)
                .add_modifier(Modifier::ITALIC),
        )]));
    } else {
        for (i, path) in app.workspace_history.iter().enumerate() {
            let is_selected = i == app.selected_workspace_index;
            let is_active = *path == app.current_dir;

            let cursor = if is_selected {
                Span::styled(
                    " ▶ ",
                    Style::default()
                        .fg(theme.purple)
                        .add_modifier(Modifier::BOLD),
                )
            } else {
                Span::styled("   ", Style::default())
            };

            let icon = if is_active { "★ " } else { "☆ " };
            let icon_span = Span::styled(
                icon,
                if is_active {
                    Style::default().fg(theme.green)
                } else {
                    Style::default().fg(theme.border)
                },
            );

            // Display shortened path: show last 2 components for readability
            let display_path = {
                let parts: Vec<&str> = path.rsplitn(3, '/').collect();
                if parts.len() >= 2 {
                    format!(
                        ".../{}",
                        parts
                            .iter()
                            .rev()
                            .skip(1)
                            .cloned()
                            .collect::<Vec<&str>>()
                            .join("/")
                    )
                } else {
                    path.clone()
                }
            };

            let path_style = if is_selected {
                Style::default()
                    .fg(theme.fg)
                    .bg(theme.select_bg)
                    .add_modifier(Modifier::BOLD)
            } else if is_active {
                Style::default()
                    .fg(theme.green)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(theme.fg)
            };

            let active_badge = if is_active {
                Span::styled(
                    if is_vi {
                        " (Đang mở) "
                    } else {
                        " (Active) "
                    },
                    Style::default()
                        .fg(theme.green)
                        .add_modifier(Modifier::ITALIC),
                )
            } else {
                Span::styled("", Style::default())
            };

            content.push(Line::from(vec![
                cursor,
                icon_span,
                Span::styled(display_path, path_style),
                active_badge,
            ]));
        }
    }

    content.push(Line::from(""));
    content.push(Line::from(vec![Span::styled(
        if is_vi {
            "  [Enter] Mở  [n] Folder mới  [x] Xóa  [Esc] Đóng"
        } else {
            "  [Enter] Open  [n] New Folder  [x] Remove  [Esc] Close"
        },
        Style::default()
            .fg(theme.orange)
            .add_modifier(Modifier::BOLD),
    )]));

    let block = Block::default()
        .title(Span::styled(
            if is_vi {
                " 📂 LỊCH SỬ WORKSPACE "
            } else {
                " 📂 WORKSPACE HISTORY "
            },
            Style::default().fg(theme.cyan).add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.cyan))
        .border_type(BorderType::Double)
        .style(Style::default().bg(theme.bg));

    let paragraph = Paragraph::new(content)
        .alignment(ratatui::layout::Alignment::Left)
        .block(block);
    f.render_widget(paragraph, area);
}

pub fn render_view_prompt(f: &mut Frame, app: &App, area: Rect) {
    let theme = app.theme();
    let is_vi = app.current_lang == "vi";
    f.render_widget(Clear, area);

    let mut content = vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            if is_vi {
                "🤖 PROMPT AI ĐÃ THIẾT LẬP"
            } else {
                "🤖 CONFIGURED AI PROMPT"
            },
            Style::default().fg(theme.cyan).add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
    ];

    for line_str in app.prompt_text.lines() {
        content.push(Line::from(Span::styled(
            line_str.to_string(),
            Style::default().fg(theme.fg),
        )));
    }

    content.push(Line::from(""));
    content.push(Line::from(vec![Span::styled(
        if is_vi {
            "  [Esc] [q] [x] [Enter] để đóng"
        } else {
            "  [Esc] [q] [x] [Enter] to close"
        },
        Style::default().fg(theme.border),
    )]));

    let block = Block::default()
        .title(Span::styled(
            " AI PROMPT ",
            Style::default().fg(theme.cyan).add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.cyan))
        .border_type(BorderType::Double)
        .style(Style::default().bg(theme.bg));

    let paragraph = Paragraph::new(content)
        .alignment(ratatui::layout::Alignment::Left)
        .wrap(Wrap { trim: true })
        .block(block);
    f.render_widget(paragraph, area);
}

pub fn render_kilo_model_select(f: &mut Frame, app: &App, area: Rect) {
    let theme = app.theme();
    let is_vi = app.current_lang == "vi";
    f.render_widget(Clear, area);

    let filtered: Vec<&String> = if app.kilo_model_filter.is_empty() {
        app.kilo_models.iter().collect()
    } else {
        let f = app.kilo_model_filter.to_lowercase();
        app.kilo_models
            .iter()
            .filter(|m| m.to_lowercase().contains(&f))
            .collect()
    };

    let mut content = vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            if is_vi {
                "🤖 CHỌN MODEL KILO"
            } else {
                "🤖 SELECT KILO MODEL"
            },
            Style::default().fg(theme.cyan).add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
    ];

    // Search bar
    if app.kilo_model_search_mode || !app.kilo_model_filter.is_empty() {
        let search_display = if app.kilo_model_filter.is_empty() {
            if is_vi {
                "  Tìm: _".to_string()
            } else {
                "  Search: _".to_string()
            }
        } else {
            if is_vi {
                format!("  Tìm: {}", app.kilo_model_filter)
            } else {
                format!("  Search: {}", app.kilo_model_filter)
            }
        };
        content.push(Line::from(vec![Span::styled(
            search_display,
            Style::default()
                .fg(theme.yellow)
                .add_modifier(Modifier::BOLD),
        )]));
        content.push(Line::from(""));
    }

    if filtered.is_empty() {
        content.push(Line::from(vec![Span::styled(
            if is_vi {
                "  Không tìm thấy model nào khớp."
            } else {
                "  No matching models found."
            },
            Style::default().fg(theme.red),
        )]));
    } else {
        let start = if app.selected_kilo_model_index > 10 {
            app.selected_kilo_model_index - 10
        } else {
            0
        };
        let visible: Vec<_> = filtered.iter().skip(start).take(16).collect();

        for (i, model) in visible.iter().enumerate() {
            let real_idx = start + i;
            let is_selected = real_idx == app.selected_kilo_model_index;

            let prefix = if is_selected {
                Span::styled(
                    " ▶ ",
                    Style::default()
                        .fg(theme.green)
                        .add_modifier(Modifier::BOLD),
                )
            } else {
                Span::styled("   ", Style::default())
            };

            let style = if is_selected {
                Style::default()
                    .fg(theme.fg)
                    .bg(theme.select_bg)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(theme.fg)
            };

            content.push(Line::from(vec![
                prefix,
                Span::styled((*model).to_string(), style),
            ]));
        }

        if filtered.len() > 16 {
            content.push(Line::from(vec![Span::styled(
                "  ...",
                Style::default().fg(theme.border),
            )]));
        }
    }

    content.push(Line::from(""));
    content.push(Line::from(vec![Span::styled(
        if is_vi {
            "  [/] Tìm  [↑/↓] Di chuyển  [Enter] Chọn  [Esc] Hủy"
        } else {
            "  [/] Search  [↑/↓] Move  [Enter] Select  [Esc] Cancel"
        },
        Style::default().fg(theme.border),
    )]));

    let block = Block::default()
        .title(Span::styled(
            " KILO MODEL ",
            Style::default().fg(theme.cyan).add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.cyan))
        .border_type(BorderType::Double)
        .style(Style::default().bg(theme.bg));

    let paragraph = Paragraph::new(content)
        .alignment(ratatui::layout::Alignment::Left)
        .block(block);
    f.render_widget(paragraph, area);
}

pub fn render_git_menu(f: &mut Frame, app: &App, area: Rect) {
    let theme = app.theme();
    let is_vi = app.current_lang == "vi";
    f.render_widget(Clear, area);

    let actions = if is_vi {
        vec![
            (
                "Commit",
                vec![
                    ("🤖 AI Commit & Push", 'g'),
                    ("✍️  Commit thủ công", 'c'),
                    ("📝 Amend commit cuối", 'm'),
                ],
            ),
            (
                "Remote",
                vec![
                    ("📥 Fetch", 'f'),
                    ("⬇️  Pull", 'p'),
                    ("⬆️  Push", 'u'),
                    ("🌐 Remote Info", 'i'),
                ],
            ),
            (
                "Khác",
                vec![
                    ("🌿 Quản lý Branch", 'b'),
                    ("📦 Stash", 's'),
                    ("🌳 Cây Commit (graph)", 't'),
                    ("📜 Lịch sử Commit", 'v'),
                    ("🧩 Commit theo Feature", 'e'),
                    ("💻 Viết lệnh Terminal", 'z'),
                ],
            ),
        ]
    } else {
        vec![
            (
                "Commit",
                vec![
                    ("🤖 AI Commit & Push", 'g'),
                    ("✍️  Manual Commit", 'c'),
                    ("📝 Amend Last Commit", 'm'),
                ],
            ),
            (
                "Remote",
                vec![
                    ("📥 Fetch", 'f'),
                    ("⬇️  Pull", 'p'),
                    ("⬆️  Push", 'u'),
                    ("🌐 Remote Info", 'i'),
                ],
            ),
            (
                "Other",
                vec![
                    ("🌿 Branch Management", 'b'),
                    ("📦 Stash", 's'),
                    ("🌳 Commit Tree (graph)", 't'),
                    ("📜 Commit History", 'v'),
                    ("🧩 Feature Commit", 'e'),
                    ("💻 Terminal Command", 'z'),
                ],
            ),
        ]
    };

    let mut content = vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            if is_vi {
                "🛠️  MENU GIT OPERATIONS"
            } else {
                "🛠️  GIT OPERATIONS MENU"
            },
            Style::default().fg(theme.cyan).add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
    ];

    let mut idx = 0;
    for (group, items) in &actions {
        content.push(Line::from(vec![Span::styled(
            format!("  ■ {}", group),
            Style::default()
                .fg(theme.purple)
                .add_modifier(Modifier::BOLD),
        )]));

        for (name, key) in items {
            let is_selected = idx == app.selected_git_action;

            let prefix = if is_selected {
                Span::styled(
                    " ▶ ",
                    Style::default()
                        .fg(theme.green)
                        .add_modifier(Modifier::BOLD),
                )
            } else {
                Span::styled("   ", Style::default())
            };

            let style = if is_selected {
                Style::default()
                    .fg(theme.fg)
                    .bg(theme.select_bg)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(theme.fg)
            };

            content.push(Line::from(vec![
                prefix,
                Span::styled(format!("[{}] {}", key, name), style),
            ]));
            idx += 1;
        }
        content.push(Line::from(""));
    }

    content.push(Line::from(vec![Span::styled(
        if is_vi {
            "  [↑/↓] Di chuyển  [Enter] Chọn  [Esc] Đóng"
        } else {
            "  [↑/↓] Navigate  [Enter] Select  [Esc] Close"
        },
        Style::default().fg(theme.border),
    )]));

    let block = Block::default()
        .title(Span::styled(
            if is_vi {
                " 🛠️ GIT MENU "
            } else {
                " 🛠️ GIT MENU "
            },
            Style::default().fg(theme.cyan).add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.cyan))
        .border_type(BorderType::Double)
        .style(Style::default().bg(theme.bg));

    let paragraph = Paragraph::new(content)
        .alignment(ratatui::layout::Alignment::Left)
        .block(block);
    f.render_widget(paragraph, area);
}

// render_manual_commit has been moved to ui/modals/manual_commit.rs
// (see docs/adding-a-modal.md for the new recommended pattern)

pub fn render_commit_tree(f: &mut Frame, app: &App, area: Rect) {
    let theme = app.theme();
    let is_vi = app.current_lang == "vi";
    f.render_widget(Clear, area);

    // Split thành 2 cột - cho Graph nhiều không gian hơn
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(62), Constraint::Percentage(38)])
        .split(area);

    // === LEFT: Commit Graph (cải tiến) ===
    // Màu graph đồng bộ theme + ưu tiên màu nổi bật
    let graph_colors = vec![
        theme.green,
        theme.cyan,
        theme.purple,
        theme.yellow,
        theme.orange,
        theme.green, // lặp lại để đủ lane
    ];

    let mut left_content = vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            if is_vi {
                "🌳 COMMIT GRAPH"
            } else {
                "🌳 COMMIT GRAPH"
            },
            Style::default()
                .fg(theme.green)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
    ];

    if app.commit_logs.is_empty() {
        left_content.push(Line::from(vec![Span::styled(
            if is_vi {
                "Không có commit."
            } else {
                "No commits."
            },
            Style::default()
                .fg(theme.border)
                .add_modifier(Modifier::ITALIC),
        )]));
    } else {
        for (i, entry) in app.commit_logs.iter().enumerate() {
            let is_selected = i == app.selected_log_index;

            // === Graph nâng cao - nhiều lane + ký tự kết nối ===
            let lane = if entry.parents.len() > 1 {
                0
            } else {
                (i % 4) as usize
            };

            // Xây dựng graph column
            let graph = match (entry.parents.len() > 1, lane, i) {
                (true, _, _) => " ├─◉".to_string(), // Merge
                (_, 0, 0) => " ●  ".to_string(),    // Root main
                (_, 0, _) => " │  ".to_string(),    // Main line
                (_, 1, _) => " ├─●".to_string(),    // Branch 1
                (_, 2, _) => " ├─●".to_string(),    // Branch 2
                (_, 3, _) => " └─●".to_string(),    // Branch 3
                _ => " │  ".to_string(),
            };

            let branch_color = graph_colors[lane % graph_colors.len()];

            // Avatar: đồng bộ theme sáng/tối để không bị chìm
            let initial = entry.author.chars().next().unwrap_or('?');
            let avatar_fg = if app.is_light_theme {
                theme.fg
            } else {
                theme.bg
            };
            let avatar = Span::styled(
                format!(" {} ", initial),
                Style::default()
                    .fg(avatar_fg)
                    .bg(branch_color)
                    .add_modifier(Modifier::BOLD),
            );

            let author = Span::styled(
                format!("{:<12}", entry.author),
                Style::default().fg(theme.fg),
            );
            let hash = Span::styled(
                format!("[{}]", entry.short_hash),
                Style::default()
                    .fg(theme.yellow)
                    .add_modifier(Modifier::BOLD),
            );

            let subject_style = if is_selected {
                Style::default()
                    .fg(theme.fg)
                    .bg(theme.select_bg)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(theme.fg)
            };
            let subject = Span::styled(format!(" {}", entry.subject), subject_style);

            left_content.push(Line::from(vec![
                Span::styled(
                    graph,
                    Style::default()
                        .fg(branch_color)
                        .add_modifier(Modifier::BOLD),
                ),
                avatar,
                author,
                hash,
                subject,
            ]));
        }
    }

    left_content.push(Line::from(""));
    left_content.push(Line::from(vec![Span::styled(
        if is_vi {
            "↑/↓  [Esc]  t = Tree"
        } else {
            "↑/↓  [Esc]  t = Tree"
        },
        Style::default().fg(theme.border),
    )]));

    let left_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.green))
        .border_type(BorderType::Rounded)
        .style(Style::default().bg(theme.bg))
        .title(Span::styled(
            " Graph ",
            Style::default()
                .fg(theme.green)
                .add_modifier(Modifier::BOLD),
        ));

    let left_paragraph = Paragraph::new(left_content)
        .alignment(ratatui::layout::Alignment::Left)
        .block(left_block);
    f.render_widget(left_paragraph, chunks[0]);

    // === RIGHT: Diff Preview ===
    let mut right_content = vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            if is_vi {
                "DIFF CỦA COMMIT"
            } else {
                "DIFF OF COMMIT"
            },
            Style::default().fg(theme.cyan).add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
    ];

    if !app.commit_diff_content.is_empty() {
        let diff_lines: Vec<&str> = app.commit_diff_content.lines().take(22).collect();
        for line in diff_lines {
            let color = if line.starts_with('+') && !line.starts_with("+++") {
                theme.green
            } else if line.starts_with('-') && !line.starts_with("---") {
                theme.red
            } else if line.starts_with("@@") {
                theme.cyan
            } else {
                theme.fg
            };
            right_content.push(Line::from(vec![Span::styled(
                format!(" {}", line),
                Style::default().fg(color),
            )]));
        }
    } else {
        right_content.push(Line::from(vec![Span::styled(
            if is_vi {
                "(Chọn commit để xem diff)"
            } else {
                "(Select commit to view diff)"
            },
            Style::default()
                .fg(theme.border)
                .add_modifier(Modifier::ITALIC),
        )]));
    }

    let right_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.cyan))
        .border_type(BorderType::Rounded)
        .style(Style::default().bg(theme.bg))
        .title(Span::styled(
            " Diff ",
            Style::default().fg(theme.cyan).add_modifier(Modifier::BOLD),
        ));

    let right_paragraph = Paragraph::new(right_content)
        .alignment(ratatui::layout::Alignment::Left)
        .block(right_block);
    f.render_widget(right_paragraph, chunks[1]);
}
