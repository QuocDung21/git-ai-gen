use crate::app::App;
use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap},
    Frame,
};
use rust_i18n::t;

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
        let expanded = line_str.replace('\t', "    ");
        content.push(Line::from(Span::styled(
            expanded,
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
