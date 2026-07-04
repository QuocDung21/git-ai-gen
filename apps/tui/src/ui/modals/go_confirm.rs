use crate::app::App;
use crate::models::GoStep;
use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};
use rust_i18n::t;

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
