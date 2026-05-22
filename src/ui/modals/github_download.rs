use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};

use crate::app::App;

pub fn render_github_download_url_input(f: &mut Frame, app: &App, area: Rect) {
    let theme = app.theme();
    let is_vi = app.current_lang == "vi";
    f.render_widget(Clear, area);

    let display_msg = if app.github_download_url.len() > 70 {
        format!("{}...", &app.github_download_url[..67])
    } else {
        app.github_download_url.clone()
    };

    let mut content = vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            if is_vi {
                "📥 TẢI TẬP TIN TỪ GITHUB"
            } else {
                "📥 DOWNLOAD FILES FROM GITHUB"
            },
            Style::default()
                .fg(theme.cyan)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(vec![Span::styled(
            if is_vi {
                "  Nhập URL repository (HTTPS hoặc SSH) bên dưới:"
            } else {
                "  Enter repository URL (HTTPS or SSH) below:"
            },
            Style::default().fg(theme.border),
        )]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  🔍 [ ", Style::default().fg(theme.cyan)),
            Span::styled(
                format!("{}_", display_msg),
                Style::default()
                    .fg(theme.green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" ]", Style::default().fg(theme.cyan)),
        ]),
        Line::from(""),
    ];

    if let Some(err) = &app.github_cloning_error {
        content.push(Line::from(vec![Span::styled(
            format!("  ❌ Lỗi: {}", err),
            Style::default().fg(theme.red).add_modifier(Modifier::BOLD),
        )]));
        content.push(Line::from(""));
    }

    content.push(Line::from(vec![Span::styled(
        if is_vi {
            "  Nhập URL, [Enter] để tìm nạp repo, [Esc] để hủy"
        } else {
            "  Type URL, [Enter] to fetch repo, [Esc] to cancel"
        },
        Style::default().fg(theme.border),
    )]));

    let block = Block::default()
        .title(Span::styled(
            if is_vi {
                " 📥 TẢI TỪ GITHUB "
            } else {
                " 📥 GITHUB DOWNLOAD "
            },
            Style::default()
                .fg(theme.cyan)
                .add_modifier(Modifier::BOLD),
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

pub fn render_github_download_tree(f: &mut Frame, app: &App, area: Rect) {
    let theme = app.theme();
    let is_vi = app.current_lang == "vi";
    f.render_widget(Clear, area);

    let visible = app.get_visible_github_tree_entries();
    let total = visible.len();
    let selected = app.selected_github_tree_index;

    let mut content = vec![
        Line::from(""),
    ];

    if visible.is_empty() {
        content.push(Line::from(vec![Span::styled(
            if is_vi {
                "  📁 CẤU TRÚC THƯ MỤC REPOSITORY"
            } else {
                "  📁 REPOSITORY DIRECTORY TREE"
            },
            Style::default().fg(theme.cyan).add_modifier(Modifier::BOLD),
        )]));
        content.push(Line::from(""));
        content.push(Line::from(vec![Span::styled(
            if is_vi {
                "  (Không tìm thấy tập tin nào)"
            } else {
                "  (No files found)"
            },
            Style::default().fg(theme.border).add_modifier(Modifier::ITALIC),
        )]));
    } else {
        content.push(Line::from(vec![
            Span::styled(
                if is_vi {
                    "  📁 CẤU TRÚC THƯ MỤC REPOSITORY"
                } else {
                    "  📁 REPOSITORY DIRECTORY TREE"
                },
                Style::default().fg(theme.cyan).add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!("  (Mục {} / {})", selected + 1, total),
                Style::default().fg(theme.border).add_modifier(Modifier::ITALIC),
            ),
        ]));
        content.push(Line::from(""));

        let page_size = (area.height as usize).saturating_sub(7).max(5);
        let start = if selected > page_size / 2 {
            (selected - page_size / 2).min(total.saturating_sub(page_size))
        } else {
            0
        };
        let visible_page = visible.iter().skip(start).take(page_size);

        for (i, entry) in visible_page.enumerate() {
            let real_idx = start + i;
            let is_selected = real_idx == selected;

            let mut line_spans = Vec::new();

            let cursor = if is_selected {
                Span::styled(" ➜ ", Style::default().fg(theme.green).add_modifier(Modifier::BOLD))
            } else {
                Span::styled("   ", Style::default())
            };
            line_spans.push(cursor);

            if entry.depth > 0 {
                for _ in 0..(entry.depth - 1) {
                    line_spans.push(Span::styled("│  ", Style::default().fg(theme.border)));
                }
                let connector = if real_idx + 1 < total {
                    if visible[real_idx + 1].depth < entry.depth {
                        "└── "
                    } else {
                        "├── "
                    }
                } else {
                    "└── "
                };
                line_spans.push(Span::styled(connector, Style::default().fg(theme.border)));
            }

            let icon = if entry.is_dir {
                if app.github_expanded_dirs.contains(&entry.path) {
                    "▼ 📁 "
                } else {
                    "▶ 📁 "
                }
            } else {
                "  📄 "
            };
            line_spans.push(Span::styled(icon, if entry.is_dir { Style::default().fg(theme.cyan) } else { Style::default().fg(theme.border) }));

            let name_style = if is_selected {
                Style::default()
                    .fg(theme.select_fg)
                    .bg(theme.select_bg)
                    .add_modifier(Modifier::BOLD)
            } else if entry.is_dir {
                Style::default()
                    .fg(theme.cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(theme.fg)
            };

            line_spans.push(Span::styled(&entry.name, name_style));

            content.push(Line::from(line_spans));
        }

        if total > start + page_size {
            content.push(Line::from(vec![Span::styled(
                if is_vi {
                    format!("   ... và {} mục khác", total - (start + page_size))
                } else {
                    format!("   ... and {} more items", total - (start + page_size))
                },
                Style::default().fg(theme.border).add_modifier(Modifier::ITALIC),
            )]));
        }
    }

    content.push(Line::from(""));
    content.push(Line::from(vec![Span::styled(
        if is_vi {
            "  [Khoảng trắng] Mở/Đóng  [↑/↓] Di chuyển  [Enter] Chọn để tải về  [Esc] Quay lại"
        } else {
            "  [Space] Expand/Collapse  [↑/↓] Navigate  [Enter] Select to download  [Esc] Back"
        },
        Style::default().fg(theme.border),
    )]));

    let block = Block::default()
        .title(Span::styled(
            if is_vi {
                " 📁 CHỌN MỤC TẢI VỀ "
            } else {
                " 📁 SELECT ITEM TO DOWNLOAD "
            },
            Style::default()
                .fg(theme.cyan)
                .add_modifier(Modifier::BOLD),
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

pub fn render_github_download_target_input(f: &mut Frame, app: &App, area: Rect) {
    let theme = app.theme();
    let is_vi = app.current_lang == "vi";
    f.render_widget(Clear, area);

    let display_msg = if app.github_download_target_path.len() > 70 {
        format!("{}...", &app.github_download_target_path[..67])
    } else {
        app.github_download_target_path.clone()
    };

    let visible = app.get_visible_github_tree_entries();
    let selected_entry_name = if let Some(entry) = visible.get(app.selected_github_tree_index) {
        entry.name.clone()
    } else {
        String::new()
    };

    let content = vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            if is_vi {
                "💾 CHỌN NƠI LƯU TẬP TIN"
            } else {
                "💾 SELECT SAVE DESTINATION"
            },
            Style::default()
                .fg(theme.green)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(vec![
            Span::styled(if is_vi { "  Tập tin/Thư mục tải: " } else { "  Downloading: " }, Style::default().fg(theme.fg)),
            Span::styled(selected_entry_name, Style::default().fg(theme.cyan).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            if is_vi {
                "  Nhập đường dẫn thư mục lưu bên dưới:"
            } else {
                "  Enter destination folder path below:"
            },
            Style::default().fg(theme.border),
        )]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  📥 [ ", Style::default().fg(theme.green)),
            Span::styled(
                format!("{}_", display_msg),
                Style::default()
                    .fg(theme.fg)
                    .bg(theme.bg)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" ]", Style::default().fg(theme.green)),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            if is_vi {
                "  [Enter] Đồng ý & Tải về  [Esc] Quay lại"
            } else {
                "  [Enter] Confirm & Download  [Esc] Back"
            },
            Style::default().fg(theme.border),
        )]),
    ];

    let block = Block::default()
        .title(Span::styled(
            if is_vi {
                " 💾 NƠI LƯU "
            } else {
                " 💾 SAVE TO "
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
