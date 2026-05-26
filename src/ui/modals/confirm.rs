use crate::app::App;
use crate::models::{AmendStep, GoStep, StashAction, StashStep};
use rust_i18n::t;

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
    f.render_widget(Clear, area);

    let mut content = vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            t!("lang_select_prompt").to_string(),
            Style::default().fg(theme.fg).add_modifier(Modifier::ITALIC),
        )]),
        Line::from(""),
    ];

    let items = vec![
        ("vi", "Tiếng Việt 🇻🇳", "[v]"),
        ("en", "English 🇺🇸", "[e]"),
        ("auto", "Tự động / Auto (System) ⚙️", "[a]"),
    ];

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
        t!("lang_select_navigate").to_string(),
        Style::default().fg(theme.border),
    )]));

    let block = Block::default()
        .title(Span::styled(
            t!("lang_select_title").to_string(),
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
    f.render_widget(Clear, area);

    let mut content = vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            t!("theme_select_prompt").to_string(),
            Style::default().fg(theme.fg).add_modifier(Modifier::ITALIC),
        )]),
        Line::from(""),
    ];

    let items = crate::theme::get_all_themes();
    let active_theme = &app.theme_id;

    for (i, t_info) in items.iter().enumerate() {
        let is_hovered = i == app.selected_theme_index;
        let is_currently_active = active_theme == t_info.id;

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

        let label = if app.current_lang == "vi" {
            t_info.name_vi
        } else {
            t_info.name_en
        };

        content.push(Line::from(vec![
            cursor,
            Span::styled(
                format!("{} ", t_info.shortcut),
                Style::default().fg(theme.cyan).add_modifier(Modifier::BOLD),
            ),
            Span::styled(label, item_style),
            active_badge,
        ]));
    }

    content.push(Line::from(""));
    content.push(Line::from(vec![Span::styled(
        t!("theme_select_navigate").to_string(),
        Style::default().fg(theme.border),
    )]));

    let block = Block::default()
        .title(Span::styled(
            t!("theme_select_title").to_string(),
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
    f.render_widget(Clear, area);

    let content = vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            t!("confirm_revert_warning").to_string(),
            Style::default().fg(theme.red).add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(vec![Span::styled(
            t!("confirm_revert_question").to_string(),
            Style::default().fg(theme.fg),
        )]),
        Line::from(vec![Span::styled(
            format!("👉 {} ", path),
            Style::default().fg(theme.cyan).add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(vec![Span::styled(
            t!("confirm_revert_irreversible").to_string(),
            Style::default().fg(theme.red).add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                t!("confirm_revert_confirm_btn").to_string(),
                Style::default()
                    .fg(theme.fg)
                    .bg(theme.green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("      ", Style::default()),
            Span::styled(
                t!("confirm_revert_cancel_btn").to_string(),
                Style::default()
                    .fg(theme.fg)
                    .bg(theme.red)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
    ];

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
    f.render_widget(Clear, area);

    let mut content = vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            t!("git_log_heading").to_string(),
            Style::default()
                .fg(theme.green)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
    ];

    if app.commit_logs.is_empty() {
        content.push(Line::from(vec![Span::styled(
            t!("git_log_empty").to_string(),
            Style::default()
                .fg(theme.border)
                .add_modifier(Modifier::ITALIC),
        )]));
    } else {
        content.push(Line::from(vec![
            Span::styled("    ", Style::default()),
            Span::styled(
                "HASH     ",
                Style::default()
                    .fg(theme.purple)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                t!("git_log_col_date").to_string(),
                Style::default()
                    .fg(theme.purple)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                t!("git_log_col_author").to_string(),
                Style::default()
                    .fg(theme.purple)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                t!("git_log_col_subject").to_string(),
                Style::default()
                    .fg(theme.purple)
                    .add_modifier(Modifier::BOLD),
            ),
        ]));

        let sep_width = (area.width as usize).saturating_sub(8).min(120);
        content.push(Line::from(vec![Span::styled(
            format!("    {}", "─".repeat(sep_width)),
            Style::default().fg(theme.border),
        )]));

        for (i, entry) in app.commit_logs.iter().enumerate() {
            let is_selected = i == app.selected_log_index;
            let bg_style = if is_selected {
                theme.select_bg
            } else {
                theme.bg
            };
            let style_base = if is_selected {
                Style::default()
                    .fg(theme.select_fg)
                    .bg(theme.select_bg)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(theme.fg).bg(theme.bg)
            };

            let pointer = if is_selected { "  ➜ " } else { "    " };
            let pointer_span = Span::styled(
                pointer,
                Style::default()
                    .fg(theme.cyan)
                    .bg(bg_style)
                    .add_modifier(Modifier::BOLD),
            );

            let short_hash = &entry.short_hash;
            let hash_str = format!("{:<9}", short_hash);
            let hash_span = Span::styled(
                hash_str,
                Style::default()
                    .fg(theme.yellow)
                    .bg(bg_style)
                    .add_modifier(Modifier::BOLD),
            );

            let time_truncated: String = entry.time.chars().take(17).collect();
            let time_str = format!("{:<17}", time_truncated);
            let time_span = Span::styled(
                time_str,
                Style::default()
                    .fg(theme.cyan)
                    .bg(bg_style)
                    .add_modifier(Modifier::ITALIC),
            );

            let author_truncated: String = entry.author.chars().take(15).collect();
            let author_str = format!("{:<16}", author_truncated);
            let author_span =
                Span::styled(author_str, Style::default().fg(theme.purple).bg(bg_style));

            let max_sub_width = (area.width as usize).saturating_sub(52).min(65);
            let subject_text = &entry.subject;
            let subject_str = if subject_text.chars().count() > max_sub_width {
                let truncated: String = subject_text
                    .chars()
                    .take(max_sub_width.saturating_sub(3))
                    .collect();
                format!("{}...", truncated)
            } else {
                subject_text.clone()
            };
            let subject_span = Span::styled(subject_str, style_base);

            content.push(Line::from(vec![
                pointer_span,
                hash_span,
                time_span,
                author_span,
                subject_span,
            ]));
        }
    }

    content.push(Line::from(""));
    content.push(Line::from(vec![Span::styled(
        t!("git_log_footer").to_string(),
        Style::default()
            .fg(theme.orange)
            .add_modifier(Modifier::BOLD),
    )]));

    let block = Block::default()
        .title(Span::styled(
            t!("git_log_title", count = app.commit_logs.len()).to_string(),
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
    f.render_widget(Clear, area);

    let mut content = vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            t!("branch_select_heading").to_string(),
            Style::default().fg(theme.cyan).add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
    ];

    if app.branches.is_empty() {
        content.push(Line::from(vec![Span::styled(
            t!("branch_select_empty").to_string(),
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
                    t!("branch_select_active_badge").to_string(),
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
        t!("branch_select_navigate").to_string(),
        Style::default()
            .fg(theme.orange)
            .add_modifier(Modifier::BOLD),
    )]));
    content.push(Line::from(vec![Span::styled(
        t!("branch_select_merge_hint").to_string(),
        Style::default()
            .fg(theme.green)
            .add_modifier(Modifier::BOLD),
    )]));
    content.push(Line::from(vec![Span::styled(
        t!("branch_select_create_hint").to_string(),
        Style::default()
            .fg(theme.purple)
            .add_modifier(Modifier::BOLD),
    )]));
    content.push(Line::from(vec![Span::styled(
        t!("branch_select_delete_hint").to_string(),
        Style::default().fg(theme.red).add_modifier(Modifier::BOLD),
    )]));
    content.push(Line::from(vec![Span::styled(
        t!("branch_select_cancel_hint").to_string(),
        Style::default().fg(theme.border),
    )]));

    let block = Block::default()
        .title(Span::styled(
            t!("branch_select_title").to_string(),
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
    f.render_widget(Clear, area);

    let header_color = if app.diff_copy_failed {
        theme.yellow
    } else {
        theme.cyan
    };
    let header_text = if app.diff_copy_failed {
        t!("diff_result_clipboard_failed_header").to_string()
    } else {
        t!("diff_result_clipboard_ok_header").to_string()
    };

    let mut content = vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            header_text,
            Style::default()
                .fg(header_color)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
    ];

    if app.diff_captured_unstaged {
        content.push(Line::from(vec![Span::styled(
            t!("diff_result_unstaged_notice").to_string(),
            Style::default()
                .fg(theme.orange)
                .add_modifier(Modifier::ITALIC),
        )]));
        content.push(Line::from(""));
    }

    content.push(Line::from(vec![
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
    ]));
    content.push(Line::from(""));
    content.push(Line::from(vec![
        Span::styled("  🤖 Model: ", Style::default().fg(theme.border)),
        Span::styled(
            if app.current_kilo_model.is_empty() {
                t!("diff_result_model_default").to_string()
            } else {
                app.current_kilo_model.clone()
            },
            Style::default()
                .fg(theme.green)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled("   [M] Đổi", Style::default().fg(theme.cyan)),
    ]));
    content.push(Line::from(""));
    content.push(Line::from(vec![Span::styled(
        t!("diff_result_preview_label").to_string(),
        Style::default()
            .fg(theme.border)
            .add_modifier(Modifier::ITALIC),
    )]));
    content.push(Line::from(""));

    let preview_limit = if !app.diff_kilo_generated.is_empty() {
        6
    } else {
        22
    };

    let total_lines = app.diff_snapshot.lines().count();
    let max_scroll = total_lines.saturating_sub(preview_limit);
    let scroll = app.diff_snapshot_scroll.min(max_scroll);

    for line in app.diff_snapshot.lines().skip(scroll).take(preview_limit) {
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
            t!("diff_result_long_diff").to_string(),
            Style::default()
                .fg(theme.orange)
                .add_modifier(Modifier::ITALIC),
        )]));
    }

    content.push(Line::from(""));
    let status_icon = if app.diff_copy_failed {
        "  ⚠️ "
    } else {
        "  ✅ "
    };
    let status_color = if app.diff_copy_failed {
        theme.yellow
    } else {
        theme.green
    };
    let status_text = if app.diff_copy_failed {
        t!("diff_result_clipboard_fail_status").to_string()
    } else {
        t!("diff_result_clipboard_ok_status").to_string()
    };

    content.push(Line::from(vec![
        Span::styled(
            status_icon,
            Style::default()
                .fg(status_color)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            status_text,
            Style::default()
                .fg(theme.yellow)
                .add_modifier(Modifier::BOLD),
        ),
    ]));

    if !app.diff_kilo_generated.is_empty() {
        content.push(Line::from(""));
        content.push(Line::from(vec![Span::styled(
            t!("diff_result_kilo_header").to_string(),
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
            t!("diff_result_kilo_actions").to_string(),
            Style::default().fg(theme.cyan).add_modifier(Modifier::BOLD),
        )]));
    } else {
        content.push(Line::from(""));
        if app.kilo_generating {
            content.push(Line::from(vec![Span::styled(
                t!("diff_result_kilo_asking").to_string(),
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
        } else {
            content.push(Line::from(vec![Span::styled(
                t!("diff_result_kilo_prompt").to_string(),
                Style::default()
                    .fg(theme.purple)
                    .add_modifier(Modifier::BOLD),
            )]));
        }
    }

    content.push(Line::from(""));
    content.push(Line::from(vec![Span::styled(
        t!("diff_result_scroll_hint").to_string(),
        Style::default().fg(theme.border),
    )]));

    let block = Block::default()
        .title(Span::styled(
            if !app.diff_kilo_generated.is_empty() {
                " 🤖 KILO AI COMMIT "
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
                    t!("go_confirm_heading").to_string(),
                    Style::default()
                        .fg(theme.green)
                        .add_modifier(Modifier::BOLD),
                )]),
                Line::from(""),
            ];

            if app.staged_count > 0 {
                lines.push(Line::from(vec![Span::styled(
                    t!("go_confirm_files_label").to_string(),
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
                lines.push(Line::from(vec![Span::styled(
                    t!("go_confirm_no_files_warning").to_string(),
                    Style::default().fg(theme.red).add_modifier(Modifier::BOLD),
                )]));
            }

            lines.extend(vec![
                Line::from(""),
                Line::from(vec![Span::styled(
                    t!("go_confirm_commit_from_clipboard").to_string(),
                    Style::default()
                        .fg(theme.border)
                        .add_modifier(Modifier::ITALIC),
                )]),
                Line::from(vec![Span::styled(
                    format!("  💬 {}", msg_truncated),
                    Style::default()
                        .fg(theme.fg)
                        .bg(theme.bg)
                        .add_modifier(Modifier::BOLD),
                )]),
                Line::from(""),
                Line::from(vec![Span::styled(
                    t!("go_confirm_execution").to_string(),
                    Style::default().fg(theme.orange),
                )]),
                Line::from(""),
            ]);

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
                        t!("go_confirm_proceed_label").to_string(),
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
                        t!("go_confirm_cancel_label").to_string(),
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
                        t!("go_confirm_back_label").to_string(),
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
                    t!("go_confirm_processing").to_string(),
                    Style::default()
                        .fg(theme.yellow)
                        .add_modifier(Modifier::BOLD | Modifier::SLOW_BLINK),
                )]),
                Line::from(""),
                Line::from(vec![Span::styled(
                    t!("go_confirm_running").to_string(),
                    Style::default().fg(theme.cyan),
                )]),
                Line::from(""),
                Line::from(vec![Span::styled(
                    t!("go_confirm_please_wait").to_string(),
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
                    t!("go_confirm_result_label").to_string(),
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
                t!("go_confirm_close_hint").to_string(),
                Style::default().fg(theme.border),
            )]));
            lines
        }
    };

    let (title, border_color) = match &app.go_step {
        GoStep::Confirm => (t!("go_confirm_title").to_string(), theme.green),
        GoStep::Pushing => (t!("go_confirm_processing_title").to_string(), theme.yellow),
        GoStep::Done(r) => (
            if r.starts_with("✅") {
                t!("go_confirm_success_title").to_string()
            } else {
                t!("go_confirm_failed_title").to_string()
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
    f.render_widget(Clear, area);

    let mut content = vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            t!("stash_list_heading").to_string(),
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
                    t!("stash_list_empty").to_string(),
                    Style::default()
                        .fg(theme.border)
                        .add_modifier(Modifier::ITALIC),
                )]));
                content.push(Line::from(""));
                content.push(Line::from(vec![Span::styled(
                    t!("stash_list_new_stash_hint").to_string(),
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
                    t!("stash_list_actions_hint").to_string(),
                    Style::default()
                        .fg(theme.orange)
                        .add_modifier(Modifier::BOLD),
                )]));
            }
        }
        StashStep::Confirm(idx, action) => {
            let action_str = match action {
                StashAction::Pop => t!("stash_confirm_pop_action").to_string(),
                StashAction::Apply => t!("stash_confirm_apply_action").to_string(),
                StashAction::Drop => t!("stash_confirm_drop_action").to_string(),
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
                    t!("stash_confirm_confirm_btn").to_string(),
                    Style::default()
                        .fg(theme.bg)
                        .bg(action_color)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled("    ", Style::default()),
                Span::styled(
                    t!("stash_confirm_cancel_btn").to_string(),
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
            " 📦 STASH MANAGER ",
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
            t!("remote_info_heading").to_string(),
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
        t!("remote_info_remotes_label").to_string(),
        Style::default().fg(theme.cyan).add_modifier(Modifier::BOLD),
    )]));
    if app.remotes.is_empty() {
        content.push(Line::from(vec![Span::styled(
            t!("remote_info_no_remotes").to_string(),
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
            t!("remote_info_can_push").to_string()
        } else if app.behind_count > 0 {
            t!("remote_info_pull_first").to_string()
        } else {
            t!("remote_info_in_sync").to_string()
        },
        Style::default()
            .fg(theme.yellow)
            .add_modifier(Modifier::ITALIC),
    )]));
    content.push(Line::from(""));
    content.push(Line::from(vec![Span::styled(
        t!("remote_info_close_hint").to_string(),
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
                    t!("amend_edit_heading").to_string(),
                    Style::default()
                        .fg(theme.orange)
                        .add_modifier(Modifier::BOLD),
                )]),
                Line::from(""),
                Line::from(vec![Span::styled(
                    t!("amend_edit_warning").to_string(),
                    Style::default()
                        .fg(theme.red)
                        .add_modifier(Modifier::ITALIC),
                )]),
                Line::from(""),
                Line::from(vec![Span::styled(
                    t!("amend_edit_msg_label").to_string(),
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
                    t!("amend_edit_type_hint").to_string(),
                    Style::default().fg(theme.border),
                )]),
            ]
        }
        AmendStep::Pushing => {
            vec![
                Line::from(""),
                Line::from(vec![Span::styled(
                    t!("amend_pushing_label").to_string(),
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
                    t!("amend_done_close_hint").to_string(),
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
        t!("commit_diff_scroll_hint").to_string(),
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
    f.render_widget(Clear, area);

    let content = vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            t!("merge_confirm_heading").to_string(),
            Style::default()
                .fg(theme.orange)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                t!("merge_confirm_merge_verb").to_string(),
                Style::default().fg(theme.fg),
            ),
            Span::styled(
                format!("\"{}\"", branch_to_merge),
                Style::default().fg(theme.cyan).add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                t!("merge_confirm_into").to_string(),
                Style::default().fg(theme.fg),
            ),
            Span::styled(
                format!("\"{}\"", app.current_branch),
                Style::default()
                    .fg(theme.green)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            t!("merge_confirm_conflict_note1").to_string(),
            Style::default().fg(theme.red),
        )]),
        Line::from(vec![Span::styled(
            t!("merge_confirm_conflict_note2").to_string(),
            Style::default().fg(theme.red),
        )]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                t!("merge_confirm_confirm_btn").to_string(),
                Style::default()
                    .fg(theme.fg)
                    .bg(theme.green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("      ", Style::default()),
            Span::styled(
                t!("merge_confirm_cancel_btn").to_string(),
                Style::default()
                    .fg(theme.fg)
                    .bg(theme.red)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
    ];

    let block = Block::default()
        .title(Span::styled(
            t!("merge_confirm_title").to_string(),
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
    f.render_widget(Clear, area);

    let display_msg = if app.new_branch_name.len() > 70 {
        format!("{}...", &app.new_branch_name[..67])
    } else {
        app.new_branch_name.clone()
    };

    let content = vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            t!("new_branch_heading").to_string(),
            Style::default()
                .fg(theme.purple)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(vec![Span::styled(
            t!("new_branch_enter_label").to_string(),
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
            t!("new_branch_type_hint").to_string(),
            Style::default().fg(theme.border),
        )]),
    ];

    let block = Block::default()
        .title(Span::styled(
            t!("new_branch_title").to_string(),
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

pub fn render_workspace_path_input(f: &mut Frame, app: &App, area: Rect) {
    let theme = app.theme();
    f.render_widget(Clear, area);

    let display_msg = if app.workspace_path_input.len() > 70 {
        format!("{}...", &app.workspace_path_input[..67])
    } else {
        app.workspace_path_input.clone()
    };

    let content = vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            t!("workspace_path_heading").to_string(),
            Style::default()
                .fg(theme.purple)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(vec![Span::styled(
            t!("workspace_path_enter_label").to_string(),
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
            t!("workspace_path_type_hint").to_string(),
            Style::default().fg(theme.border),
        )]),
    ];

    let block = Block::default()
        .title(Span::styled(
            t!("workspace_path_title").to_string(),
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
    f.render_widget(Clear, area);

    let mut content = vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            t!("workspace_history_heading").to_string(),
            Style::default().fg(theme.cyan).add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
    ];

    if app.workspace_history.is_empty() {
        content.push(Line::from(vec![Span::styled(
            t!("workspace_history_empty").to_string(),
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
                    t!("workspace_history_active_badge").to_string(),
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
        t!("workspace_history_actions_hint").to_string(),
        Style::default()
            .fg(theme.orange)
            .add_modifier(Modifier::BOLD),
    )]));

    let block = Block::default()
        .title(Span::styled(
            t!("workspace_history_title").to_string(),
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
    f.render_widget(Clear, area);

    let mut content = vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            t!("view_prompt_heading").to_string(),
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
        t!("view_prompt_close_hint").to_string(),
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
            t!("kilo_model_heading").to_string(),
            Style::default().fg(theme.cyan).add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
    ];

    if app.kilo_model_search_mode || !app.kilo_model_filter.is_empty() {
        let search_display = if app.kilo_model_filter.is_empty() {
            t!("kilo_model_search_empty_label").to_string()
        } else {
            t!(
                "kilo_model_search_label",
                filter = app.kilo_model_filter.clone()
            )
            .to_string()
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
            t!("kilo_model_no_match").to_string(),
            Style::default().fg(theme.red),
        )]));
    } else {
        let start = app.selected_kilo_model_index.saturating_sub(10);
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
        t!("kilo_model_actions_hint").to_string(),
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
    f.render_widget(Clear, area);

    let ai_commit_label = if app.kilo_ai_enabled {
        t!("git_menu_ai_commit_enabled").to_string()
    } else {
        t!("git_menu_ai_commit_disabled").to_string()
    };

    let actions: Vec<(String, Vec<(String, char)>)> = vec![
        (
            t!("git_menu_group_commit").to_string(),
            vec![
                (ai_commit_label, 'g'),
                (t!("git_menu_manual_commit").to_string(), 'c'),
                (t!("git_menu_amend").to_string(), 'm'),
            ],
        ),
        (
            t!("git_menu_group_remote").to_string(),
            vec![
                (t!("git_menu_fetch").to_string(), 'f'),
                (t!("git_menu_pull").to_string(), 'p'),
                (t!("git_menu_push").to_string(), 'u'),
                (t!("git_menu_remote_info").to_string(), 'i'),
            ],
        ),
        (
            t!("git_menu_group_other").to_string(),
            vec![
                (t!("git_menu_branch").to_string(), 'b'),
                (t!("git_menu_stash").to_string(), 's'),
                (t!("git_menu_tree").to_string(), 't'),
                (t!("git_menu_history").to_string(), 'v'),
                (t!("git_menu_feature").to_string(), 'e'),
                (t!("git_menu_download").to_string(), 'n'),
                (t!("git_menu_settings").to_string(), 'y'),
            ],
        ),
    ];

    let mut content = vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            t!("git_menu_heading").to_string(),
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
        t!("git_menu_navigate_hint").to_string(),
        Style::default().fg(theme.border),
    )]));

    let block = Block::default()
        .title(Span::styled(
            " 🛠️ GIT MENU ",
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

pub fn render_commit_tree(f: &mut Frame, app: &App, area: Rect) {
    let theme = app.theme();
    f.render_widget(Clear, area);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(62), Constraint::Percentage(38)])
        .split(area);

    let graph_colors = [
        theme.green,
        theme.cyan,
        theme.purple,
        theme.yellow,
        theme.orange,
        theme.green,
    ];

    let mut left_content = vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            "🌳 COMMIT GRAPH",
            Style::default()
                .fg(theme.green)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
    ];

    if app.commit_logs.is_empty() {
        left_content.push(Line::from(vec![Span::styled(
            t!("commit_tree_no_commits").to_string(),
            Style::default()
                .fg(theme.border)
                .add_modifier(Modifier::ITALIC),
        )]));
    } else {
        for (i, entry) in app.commit_logs.iter().enumerate() {
            let is_selected = i == app.selected_log_index;

            let lane = if entry.parents.len() > 1 { 0 } else { i % 4 };

            let graph = match (entry.parents.len() > 1, lane, i) {
                (true, _, _) => " ├─◉".to_string(),
                (_, 0, 0) => " ●  ".to_string(),
                (_, 0, _) => " │  ".to_string(),
                (_, 1, _) => " ├─●".to_string(),
                (_, 2, _) => " ├─●".to_string(),
                (_, 3, _) => " └─●".to_string(),
                _ => " │  ".to_string(),
            };

            let branch_color = graph_colors[lane % graph_colors.len()];

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
        "↑/↓  [Esc]  t = Tree",
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

    let mut right_content = vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            t!("commit_tree_diff_heading").to_string(),
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
            t!("commit_tree_diff_empty").to_string(),
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
