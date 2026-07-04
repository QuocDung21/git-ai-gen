use crate::app::App;
use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use rust_i18n::t;

pub fn render_changes(f: &mut Frame, app: &App, area: Rect) {
    let theme = app.theme();
    let mut change_lines = vec![Line::from("")];
    if app.files.is_empty() {
        change_lines.push(Line::from(vec![
            Span::styled("   ✨ ", Style::default().fg(theme.green)),
            Span::styled(
                t!("changes_empty").to_string(),
                Style::default()
                    .fg(theme.green)
                    .add_modifier(Modifier::BOLD),
            ),
        ]));
    } else {
        let max_path_len = (area.width as usize).saturating_sub(12).max(10);
        for (i, file) in app.files.iter().enumerate() {
            let is_selected = i == app.selected_index;

            let first_char = file.status.chars().next().unwrap_or(' ');
            let second_char = file.status.chars().nth(1).unwrap_or(' ');

            let is_staged = first_char != ' ' && first_char != '?';
            let is_untracked = first_char == '?' && second_char == '?';
            let is_deleted = first_char == 'D' || second_char == 'D';

            let (badge_text, badge_style) = if is_staged {
                (
                    " [S] ",
                    Style::default()
                        .fg(theme.green)
                        .add_modifier(Modifier::BOLD),
                )
            } else if is_untracked {
                (
                    " [?] ",
                    Style::default()
                        .fg(theme.purple)
                        .add_modifier(Modifier::BOLD),
                )
            } else if is_deleted {
                (
                    " [D] ",
                    Style::default().fg(theme.red).add_modifier(Modifier::BOLD),
                )
            } else {
                (
                    " [U] ",
                    Style::default()
                        .fg(theme.yellow)
                        .add_modifier(Modifier::BOLD),
                )
            };
            let cursor_span = if is_selected {
                Span::styled(
                    " ▶ ",
                    Style::default()
                        .fg(theme.purple)
                        .add_modifier(Modifier::BOLD),
                )
            } else {
                Span::styled("   ", Style::default().fg(theme.border))
            };

            let file_style = if is_selected {
                Style::default()
                    .fg(theme.select_fg)
                    .bg(theme.select_bg)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(theme.fg)
            };

            let display_path = if file.path.chars().count() > max_path_len {
                let count = file.path.chars().count();
                let skip_count = count - max_path_len;
                format!(
                    "...{}",
                    file.path.chars().skip(skip_count).collect::<String>()
                )
            } else {
                file.path.clone()
            };

            change_lines.push(Line::from(vec![
                cursor_span,
                Span::styled(badge_text, badge_style),
                Span::styled(display_path, file_style),
            ]));
        }
    }

    let left_title = t!("changes_title");
    let changes_border_color = if app.focus_diff {
        theme.border
    } else {
        theme.purple
    };

    let changes_widget = Paragraph::new(change_lines)
        .style(Style::default().bg(theme.bg).fg(theme.fg))
        .block(
            Block::default()
                .title(Span::styled(
                    left_title.as_ref(),
                    Style::default()
                        .fg(changes_border_color)
                        .add_modifier(Modifier::BOLD),
                ))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(changes_border_color))
                .border_type(ratatui::widgets::BorderType::Rounded),
        );
    f.render_widget(changes_widget, area);
}
