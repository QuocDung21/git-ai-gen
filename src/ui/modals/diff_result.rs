use crate::app::App;
use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};
use rust_i18n::t;

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
    content.push(Line::from(vec![Span::styled(
        t!("diff_result_preview_label").to_string(),
        Style::default()
            .fg(theme.border)
            .add_modifier(Modifier::ITALIC),
    )]));
    content.push(Line::from(""));

    let preview_limit = 22;

    let total_lines = app.diff_snapshot.lines().count();
    let max_scroll = total_lines.saturating_sub(preview_limit);
    let scroll = app.diff_snapshot_scroll.min(max_scroll);

    for line in app.diff_snapshot.lines().skip(scroll).take(preview_limit) {
        let expanded = line.replace('\t', "    ");
        let (styled_line, color) = if expanded.starts_with('+') && !expanded.starts_with("+++") {
            (expanded, theme.green)
        } else if expanded.starts_with('-') && !expanded.starts_with("---") {
            (expanded, theme.red)
        } else if expanded.starts_with("@@") {
            (expanded, theme.cyan)
        } else if expanded.starts_with("diff ") || expanded.starts_with("index ") {
            (expanded, theme.purple)
        } else {
            (expanded, theme.fg)
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

    content.push(Line::from(""));
    content.push(Line::from(vec![Span::styled(
        t!("diff_result_scroll_hint").to_string(),
        Style::default().fg(theme.border),
    )]));

    let block = Block::default()
        .title(Span::styled(
            " 🤖 AI DIFF SNAPSHOT ",
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
