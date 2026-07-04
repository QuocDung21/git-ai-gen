use crate::app::App;
use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};
use rust_i18n::t;

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
