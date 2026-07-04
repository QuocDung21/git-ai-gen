use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};
use rust_i18n::t;

use crate::app::App;

pub fn render_github_download_url_input(f: &mut Frame, app: &App, area: Rect) {
    let theme = app.theme();
    f.render_widget(Clear, area);

    let display_msg = if app.github_download_url.len() > 65 {
        format!("{}...", &app.github_download_url[..62])
    } else {
        app.github_download_url.clone()
    };

    let mut content = vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            t!("github_download_url_header").to_string(),
            Style::default().fg(theme.cyan).add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(vec![Span::styled(
            t!("github_download_url_prompt").to_string(),
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
            t!("github_download_error_label", err = err).to_string(),
            Style::default().fg(theme.red).add_modifier(Modifier::BOLD),
        )]));
        content.push(Line::from(""));
    }

    if !app.github_history.is_empty() {
        content.push(Line::from(vec![Span::styled(
            t!("github_download_url_history").to_string(),
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
        t!("github_download_url_help").to_string(),
        Style::default().fg(theme.border),
    )]));

    let block = Block::default()
        .title(Span::styled(
            t!("github_download_url_title").to_string(),
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
    f.render_widget(Clear, area);

    let visible = app.get_visible_github_tree_entries();
    let total = visible.len();
    let selected = app.selected_github_tree_index;

    let mut content = vec![Line::from("")];

    if visible.is_empty() {
        content.push(Line::from(vec![Span::styled(
            t!("github_repo_tree_title").to_string(),
            Style::default().fg(theme.cyan).add_modifier(Modifier::BOLD),
        )]));
        content.push(Line::from(""));
        content.push(Line::from(vec![Span::styled(
            t!("github_no_files_found").to_string(),
            Style::default()
                .fg(theme.border)
                .add_modifier(Modifier::ITALIC),
        )]));
    } else {
        content.push(Line::from(vec![
            Span::styled(
                t!("github_repo_tree_title").to_string(),
                Style::default().fg(theme.cyan).add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                t!(
                    "github_download_tree_counter",
                    current = selected + 1,
                    total = total
                )
                .to_string(),
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
                t!("github_more_items", count = total - (start + page_size)).to_string(),
                Style::default()
                    .fg(theme.border)
                    .add_modifier(Modifier::ITALIC),
            )]));
        }
    }

    content.push(Line::from(""));
    content.push(Line::from(vec![Span::styled(
        t!("github_download_instructions").to_string(),
        Style::default().fg(theme.border),
    )]));

    let title_text = t!("github_download_title", branch = app.current_github_branch).to_string();

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
            t!("github_download_target_header").to_string(),
            Style::default()
                .fg(theme.green)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                t!("github_download_target_label").to_string(),
                Style::default().fg(theme.fg),
            ),
            Span::styled(
                selected_entry_name,
                Style::default().fg(theme.cyan).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            t!("github_download_target_prompt").to_string(),
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
            t!("github_download_target_help").to_string(),
            Style::default().fg(theme.border),
        )]),
    ];

    let block = Block::default()
        .title(Span::styled(
            t!("github_download_target_title").to_string(),
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

pub fn render_github_branch_select(f: &mut Frame, app: &App, area: Rect) {
    let theme = app.theme();
    f.render_widget(Clear, area);

    let mut content = vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            t!("github_branch_select_header").to_string(),
            Style::default().fg(theme.cyan).add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
    ];

    if app.github_branches.is_empty() {
        content.push(Line::from(vec![Span::styled(
            t!("github_branch_select_empty").to_string(),
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
                    t!("github_branch_select_active_badge").to_string(),
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
        t!("github_branch_select_navigate").to_string(),
        Style::default()
            .fg(theme.orange)
            .add_modifier(Modifier::BOLD),
    )]));
    content.push(Line::from(vec![Span::styled(
        t!("github_branch_select_cancel").to_string(),
        Style::default().fg(theme.border),
    )]));

    let block = Block::default()
        .title(Span::styled(
            t!("github_branch_select_title").to_string(),
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
