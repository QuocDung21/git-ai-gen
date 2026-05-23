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

    let display_msg = if app.github_download_url.len() > 65 {
        format!("{}...", &app.github_download_url[..62])
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
            Style::default().fg(theme.cyan).add_modifier(Modifier::BOLD),
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
                display_msg,
                Style::default()
                    .fg(theme.green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("_", Style::default().fg(theme.green)),
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

    if !app.github_history.is_empty() {
        content.push(Line::from(vec![Span::styled(
            if is_vi {
                "  📋 LỊCH SỬ REMOTE (Dùng ↑/↓ để chọn, Backspace để xóa):"
            } else {
                "  📋 REMOTE HISTORY (Use ↑/↓ to select, Backspace to delete):"
            },
            Style::default().fg(theme.cyan).add_modifier(Modifier::BOLD),
        )]));
        content.push(Line::from(""));

        for (i, url) in app.github_history.iter().enumerate() {
            let is_selected = app.selected_github_history_index == Some(i);
            let display_url = if url.len() > 65 {
                format!("{}...", &url[..62])
            } else {
                url.clone()
            };

            let prefix = if is_selected {
                Span::styled(
                    "   ➜ ",
                    Style::default()
                        .fg(theme.green)
                        .add_modifier(Modifier::BOLD),
                )
            } else {
                Span::styled("     ", Style::default().fg(theme.border))
            };

            let item_style = if is_selected {
                Style::default()
                    .fg(theme.select_fg)
                    .bg(theme.select_bg)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(theme.border)
            };

            content.push(Line::from(vec![
                prefix,
                Span::styled(display_url, item_style),
            ]));
        }
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

pub fn render_github_download_tree(f: &mut Frame, app: &App, area: Rect) {
    let theme = app.theme();
    let is_vi = app.current_lang == "vi";
    f.render_widget(Clear, area);

    let visible = app.get_visible_github_tree_entries();
    let total = visible.len();
    let selected = app.selected_github_tree_index;

    let mut content = vec![Line::from("")];

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
            Style::default()
                .fg(theme.border)
                .add_modifier(Modifier::ITALIC),
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
                Style::default()
                    .fg(theme.border)
                    .add_modifier(Modifier::ITALIC),
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
                Span::styled(
                    " ➜ ",
                    Style::default()
                        .fg(theme.green)
                        .add_modifier(Modifier::BOLD),
                )
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

            let is_checked = app.github_selected_paths.contains(&entry.path);
            let checkbox = if is_checked {
                Span::styled(
                    "[✓] ",
                    Style::default()
                        .fg(theme.green)
                        .add_modifier(Modifier::BOLD),
                )
            } else {
                Span::styled("[ ] ", Style::default().fg(theme.border))
            };
            line_spans.push(checkbox);

            let icon = if entry.is_dir {
                if app.github_expanded_dirs.contains(&entry.path) {
                    "▼ 📁 "
                } else {
                    "▶ 📁 "
                }
            } else {
                "  📄 "
            };
            line_spans.push(Span::styled(
                icon,
                if entry.is_dir {
                    Style::default().fg(theme.cyan)
                } else {
                    Style::default().fg(theme.border)
                },
            ));

            let name_style = if is_selected {
                Style::default()
                    .fg(theme.select_fg)
                    .bg(theme.select_bg)
                    .add_modifier(Modifier::BOLD)
            } else if entry.is_dir {
                Style::default().fg(theme.cyan).add_modifier(Modifier::BOLD)
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
                Style::default()
                    .fg(theme.border)
                    .add_modifier(Modifier::ITALIC),
            )]));
        }
    }

    content.push(Line::from(""));
    content.push(Line::from(vec![Span::styled(
        if is_vi {
            "  [B] Chọn branch  [V] Xem nhanh  [Space] Chọn/Bỏ chọn  [◀/▶] Đóng/Mở thư mục  [↑/↓] Di chuyển  [Enter] Xác nhận lưu & tải  [Esc] Quay lại"
        } else {
            "  [B] Branch  [V] Quick View  [Space] Select/Deselect  [◀/▶] Collapse/Expand Folder  [↑/↓] Navigate  [Enter] Confirm & Download  [Esc] Back"
        },
        Style::default().fg(theme.border),
    )]));

    let title_text = if is_vi {
        format!(
            " 📁 CHỌN MỤC TẢI VỀ (Nhánh: {}) ",
            app.current_github_branch
        )
    } else {
        format!(
            " 📁 SELECT ITEM TO DOWNLOAD (Branch: {}) ",
            app.current_github_branch
        )
    };

    let block = Block::default()
        .title(Span::styled(
            title_text,
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
            Span::styled(
                if is_vi {
                    "  Tập tin/Thư mục tải: "
                } else {
                    "  Downloading: "
                },
                Style::default().fg(theme.fg),
            ),
            Span::styled(
                selected_entry_name,
                Style::default().fg(theme.cyan).add_modifier(Modifier::BOLD),
            ),
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
                display_msg,
                Style::default()
                    .fg(theme.green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("_", Style::default().fg(theme.green)),
            Span::styled(" ]", Style::default().fg(theme.green)),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            if is_vi {
                "  [Enter] Đồng ý & Tải về  [Tab] Chọn thư mục (Finder)  [Esc] Quay lại"
            } else {
                "  [Enter] Confirm & Download  [Tab] Select Folder (Finder)  [Esc] Back"
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

fn highlight_word(word: &str, theme: &crate::theme::AppTheme) -> Span<'static> {
    if word.chars().all(|c| c.is_numeric()) {
        return Span::styled(word.to_string(), Style::default().fg(theme.orange));
    }
    let keywords = [
        "fn", "let", "pub", "struct", "enum", "impl", "if", "else", "return", "class", "import",
        "const", "var", "function", "for", "while", "match", "use", "mod", "true", "false", "null",
        "nil", "static", "mut", "ref", "self", "Self", "type", "as", "break", "continue", "in",
        "loop", "where", "trait", "crate", "async", "await", "dyn",
    ];
    if keywords.contains(&word) {
        return Span::styled(
            word.to_string(),
            Style::default()
                .fg(theme.purple)
                .add_modifier(Modifier::BOLD),
        );
    }
    let first_char = word.chars().next().unwrap_or(' ');
    let is_capitalized = first_char.is_uppercase();
    let is_numeric_type = [
        "i32", "u32", "i64", "u64", "f32", "f64", "usize", "bool", "char", "str",
    ]
    .contains(&word);
    if is_capitalized || is_numeric_type {
        return Span::styled(word.to_string(), Style::default().fg(theme.yellow));
    }
    Span::styled(word.to_string(), Style::default().fg(theme.fg))
}

fn highlight_line(line: &str, theme: &crate::theme::AppTheme) -> Line<'static> {
    let mut spans = Vec::new();
    let trimmed = line.trim_start();
    if trimmed.starts_with("//")
        || trimmed.starts_with("///")
        || trimmed.starts_with("#")
        || trimmed.starts_with("/*")
        || trimmed.starts_with("* ")
    {
        return Line::from(vec![Span::styled(
            line.to_string(),
            Style::default().fg(theme.border),
        )]);
    }
    let mut current_word = String::new();
    let mut in_string = false;
    let mut string_char = '"';
    let chars: Vec<char> = line.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        if in_string {
            current_word.push(c);
            if c == string_char && (i == 0 || chars[i - 1] != '\\') {
                in_string = false;
                spans.push(Span::styled(
                    current_word.clone(),
                    Style::default().fg(theme.green),
                ));
                current_word.clear();
            }
            i += 1;
            continue;
        }
        if c == '/' && i + 1 < chars.len() && chars[i + 1] == '/' {
            if !current_word.is_empty() {
                spans.push(Span::styled(
                    current_word.clone(),
                    Style::default().fg(theme.fg),
                ));
                current_word.clear();
            }
            let rest: String = chars[i..].iter().collect();
            spans.push(Span::styled(rest, Style::default().fg(theme.border)));
            break;
        }
        if c == '"' || c == '\'' {
            if !current_word.is_empty() {
                let word_span = highlight_word(&current_word, theme);
                spans.push(word_span);
                current_word.clear();
            }
            in_string = true;
            string_char = c;
            current_word.push(c);
            i += 1;
            continue;
        }
        if c.is_alphanumeric() || c == '_' {
            current_word.push(c);
        } else {
            if c == '(' {
                if !current_word.is_empty() {
                    spans.push(Span::styled(
                        current_word.clone(),
                        Style::default().fg(theme.cyan).add_modifier(Modifier::BOLD),
                    ));
                    current_word.clear();
                }
            } else if !current_word.is_empty() {
                let word_span = highlight_word(&current_word, theme);
                spans.push(word_span);
                current_word.clear();
            }
            let style = if c == '(' || c == ')' || c == '{' || c == '}' || c == '[' || c == ']' {
                Style::default().fg(theme.yellow)
            } else if c == '='
                || c == '+'
                || c == '-'
                || c == '*'
                || c == '/'
                || c == '&'
                || c == '|'
                || c == '!'
                || c == '<'
                || c == '>'
                || c == '?'
                || c == ':'
            {
                Style::default().fg(theme.purple)
            } else {
                Style::default().fg(theme.fg)
            };
            spans.push(Span::styled(c.to_string(), style));
        }
        i += 1;
    }
    if !current_word.is_empty() {
        let word_span = highlight_word(&current_word, theme);
        spans.push(word_span);
    }
    Line::from(spans)
}

fn apply_search_highlight(
    spans: Vec<Span<'static>>,
    query: &str,
    theme: &crate::theme::AppTheme,
) -> Vec<Span<'static>> {
    if query.is_empty() {
        return spans;
    }
    let mut new_spans = Vec::new();
    let query_lower = query.to_lowercase();
    for span in spans {
        let text = span.content.to_string();
        let text_lower = text.to_lowercase();
        if text_lower.contains(&query_lower) {
            let mut temp_text = text.clone();
            let mut temp_lower = text_lower.clone();
            while let Some(match_idx) = temp_lower.find(&query_lower) {
                if match_idx > 0 {
                    new_spans.push(Span::styled(temp_text[..match_idx].to_string(), span.style));
                }
                let match_end = match_idx + query.len();
                new_spans.push(Span::styled(
                    temp_text[match_idx..match_end].to_string(),
                    Style::default()
                        .bg(theme.yellow)
                        .fg(theme.select_fg)
                        .add_modifier(Modifier::BOLD),
                ));
                temp_text = temp_text[match_end..].to_string();
                temp_lower = temp_lower[match_end..].to_string();
            }
            if !temp_text.is_empty() {
                new_spans.push(Span::styled(temp_text, span.style));
            }
        } else {
            new_spans.push(span);
        }
    }
    new_spans
}

pub fn render_github_quick_view(f: &mut Frame, app: &App, area: Rect, path: &str, name: &str) {
    let theme = app.theme();
    let is_vi = app.current_lang == "vi";
    f.render_widget(Clear, area);

    let temp_dir = if let Some(ref dir) = app.github_temp_dir {
        dir.path()
    } else {
        return;
    };
    let file_path = temp_dir.join(path);
    let content_text = if file_path.exists() {
        std::fs::read_to_string(&file_path).unwrap_or_else(|_| {
            if is_vi {
                "❌ Không thể đọc file".to_string()
            } else {
                "❌ Cannot read file".to_string()
            }
        })
    } else {
        if is_vi {
            "❌ File không tồn tại".to_string()
        } else {
            "❌ File not found".to_string()
        }
    };

    let lines_total: Vec<&str> = content_text.lines().collect();
    let total_count = lines_total.len();
    let page_size = (area.height as usize).saturating_sub(6).max(5);
    let start_idx = app
        .github_quickview_scroll
        .min(total_count.saturating_sub(page_size));
    let visible_lines = lines_total.iter().skip(start_idx).take(page_size);

    let mut content = vec![Line::from("")];

    for (idx, line) in visible_lines.enumerate() {
        let real_line_num = start_idx + idx + 1;
        let line_num_str = format!("{:>4} │ ", real_line_num);
        let highlighted = highlight_line(line, &theme);
        let filtered_spans =
            apply_search_highlight(highlighted.spans, &app.github_quickview_search, &theme);

        let mut spans = vec![Span::styled(
            line_num_str,
            Style::default().fg(theme.border),
        )];
        spans.extend(filtered_spans);
        content.push(Line::from(spans));
    }

    content.push(Line::from(""));
    if app.github_quickview_searching {
        content.push(Line::from(vec![
            Span::styled(
                " 🔍 SEARCH: ",
                Style::default()
                    .fg(theme.green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                &app.github_quickview_search,
                Style::default().fg(theme.fg).add_modifier(Modifier::BOLD),
            ),
            Span::styled("_", Style::default().fg(theme.green)),
        ]));
    } else {
        let mut spans = vec![Span::styled(
            if is_vi {
                "[Esc] Đóng  [↑/↓] Cuộn  [/] Tìm kiếm"
            } else {
                "[Esc] Close  [↑/↓] Scroll  [/] Search"
            },
            Style::default().fg(theme.border),
        )];
        if !app.github_quickview_search.is_empty() {
            spans.push(Span::styled(
                if is_vi {
                    "  [c] Xóa tìm kiếm"
                } else {
                    "  [c] Clear search"
                },
                Style::default().fg(theme.purple),
            ));
            spans.push(Span::styled(
                format!("  (Active: {})", app.github_quickview_search),
                Style::default()
                    .fg(theme.green)
                    .add_modifier(Modifier::ITALIC),
            ));
        }
        content.push(Line::from(spans));
    }

    let title_text = format!(
        " 👁 {} (Lines: {} - {} / {}) ",
        name,
        start_idx + 1,
        (start_idx + page_size).min(total_count),
        total_count
    );

    let block = Block::default()
        .title(Span::styled(
            title_text,
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

pub fn render_github_branch_select(f: &mut Frame, app: &App, area: Rect) {
    let theme = app.theme();
    let is_vi = app.current_lang == "vi";
    f.render_widget(Clear, area);

    let mut content = vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            if is_vi {
                "🌿 DANH SÁCH CHI NHÁNH GITHUB (BRANCHES) 🌿"
            } else {
                "🌿 GITHUB BRANCH SELECTOR 🌿"
            },
            Style::default().fg(theme.cyan).add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
    ];

    if app.github_branches.is_empty() {
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
        for (i, branch) in app.github_branches.iter().enumerate() {
            let is_selected = i == app.selected_github_branch_index;
            let is_active = branch == &app.current_github_branch;
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
            } else {
                Style::default().fg(theme.fg)
            };

            let active_badge = if is_active {
                Span::styled(
                    if is_vi {
                        " (Đang xem) "
                    } else {
                        " (Viewing) "
                    },
                    Style::default()
                        .fg(theme.green)
                        .add_modifier(Modifier::ITALIC),
                )
            } else {
                Span::styled("", Style::default())
            };

            let prefix = if is_active { "★ " } else { "☆ " };

            let prefix_span = Span::styled(
                prefix,
                if is_active {
                    Style::default().fg(theme.green)
                } else {
                    Style::default().fg(theme.border)
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
            "Dùng ↑/↓ hoặc j/k để di chuyển, [Enter] để chuyển branch."
        } else {
            "Use ↑/↓ or j/k to navigate, [Enter] to switch branch."
        },
        Style::default()
            .fg(theme.orange)
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
                " 🌿 CHỌN CHI NHÁNH GITHUB "
            } else {
                " 🌿 SELECT GITHUB BRANCH "
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
