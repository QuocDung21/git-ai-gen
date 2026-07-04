use crate::app::App;
use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};
use rust_i18n::t;

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
        let expanded = line.replace('\t', "    ");
        let color = if expanded.starts_with('+') && !expanded.starts_with("+++") {
            theme.green
        } else if expanded.starts_with('-') && !expanded.starts_with("---") {
            theme.red
        } else if expanded.starts_with("@@") {
            theme.cyan
        } else if expanded.starts_with("commit ")
            || expanded.starts_with("Author:")
            || expanded.starts_with("Date:")
        {
            theme.purple
        } else if expanded.starts_with("diff ")
            || expanded.starts_with("index ")
            || expanded.starts_with("---")
            || expanded.starts_with("+++")
        {
            theme.border
        } else {
            theme.fg
        };
        content.push(Line::from(vec![Span::styled(
            expanded,
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
