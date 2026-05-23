use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};

use crate::app::App;

pub fn render_branch_delete_confirm(f: &mut Frame, app: &App, branch_name: &str, area: Rect) {
    let theme = app.theme();
    let is_vi = app.current_lang == "vi";
    f.render_widget(Clear, area);

    let content = if is_vi {
        vec![
            Line::from(""),
            Line::from(vec![Span::styled(
                "⚠️  CẢNH BÁO XÓA CHI NHÁNH  ⚠️",
                Style::default().fg(theme.red).add_modifier(Modifier::BOLD),
            )]),
            Line::from(""),
            Line::from(vec![Span::styled(
                "Bạn có chắc chắn muốn xóa chi nhánh:",
                Style::default().fg(theme.fg),
            )]),
            Line::from(vec![Span::styled(
                format!("👉 {} ", branch_name),
                Style::default().fg(theme.cyan).add_modifier(Modifier::BOLD),
            )]),
            Line::from(""),
            Line::from(vec![Span::styled(
                "⚠️ HÀNH ĐỘNG NÀY CÓ THỂ KHÔNG THỂ HOÀN TÁC!",
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
                "⚠️  BRANCH DELETION WARNING  ⚠️",
                Style::default().fg(theme.red).add_modifier(Modifier::BOLD),
            )]),
            Line::from(""),
            Line::from(vec![Span::styled(
                "Are you sure you want to delete the branch:",
                Style::default().fg(theme.fg),
            )]),
            Line::from(vec![Span::styled(
                format!("👉 {} ", branch_name),
                Style::default().fg(theme.cyan).add_modifier(Modifier::BOLD),
            )]),
            Line::from(""),
            Line::from(vec![Span::styled(
                "⚠️ THIS ACTION MIGHT BE IRREVERSIBLE!",
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
            " BRANCH DELETION ",
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
