use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use crate::app::App;

pub fn render_changes(f: &mut Frame, app: &App, area: Rect) {
    let is_vi = app.current_lang == "vi";
    let mut change_lines = vec![Line::from("")];
    if app.files.is_empty() {
        change_lines.push(Line::from(vec![
            Span::styled("   ✨ ", Style::default().fg(Color::Rgb(80, 250, 123))),
            Span::styled(
                if is_vi {
                    "Không có thay đổi!"
                } else {
                    "No changes detected!"
                },
                Style::default()
                    .fg(Color::Rgb(80, 250, 123))
                    .add_modifier(Modifier::BOLD),
            ),
        ]));
    } else {
        for (i, file) in app.files.iter().enumerate() {
            let is_selected = i == app.selected_index;

            // Check status to determine color badge
            let first_char = file.status.chars().next().unwrap_or(' ');
            let second_char = file.status.chars().nth(1).unwrap_or(' ');

            let is_staged = first_char != ' ' && first_char != '?';
            let is_untracked = first_char == '?' && second_char == '?';
            let is_deleted = first_char == 'D' || second_char == 'D';

            let (badge_text, badge_style) = if is_staged {
                (
                    " [S] ",
                    Style::default()
                        .fg(Color::Rgb(80, 250, 123))
                        .add_modifier(Modifier::BOLD),
                ) // Green Staged
            } else if is_untracked {
                (
                    " [?] ",
                    Style::default()
                        .fg(Color::Rgb(189, 147, 249))
                        .add_modifier(Modifier::BOLD),
                ) // Purple Untracked
            } else if is_deleted {
                (
                    " [D] ",
                    Style::default()
                        .fg(Color::Rgb(255, 85, 85))
                        .add_modifier(Modifier::BOLD),
                ) // Red Deleted
            } else {
                (
                    " [U] ",
                    Style::default()
                        .fg(Color::Rgb(241, 250, 140))
                        .add_modifier(Modifier::BOLD),
                ) // Yellow Unstaged
            };

            let cursor_span = if is_selected {
                Span::styled(
                    " ▶ ",
                    Style::default()
                        .fg(Color::Rgb(189, 147, 249))
                        .add_modifier(Modifier::BOLD),
                )
            } else {
                Span::styled("   ", Style::default().fg(Color::Rgb(98, 114, 164)))
            };

            let file_style = if is_selected {
                Style::default()
                    .fg(Color::Rgb(248, 248, 242))
                    .bg(Color::Rgb(68, 71, 90))
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Rgb(248, 248, 242))
            };

            change_lines.push(Line::from(vec![
                cursor_span,
                Span::styled(badge_text, badge_style),
                Span::styled(file.path.clone(), file_style),
            ]));
        }
    }

    let left_title = if is_vi {
        " 📂 THAY ĐỔI (CHANGES) "
    } else {
        " 📂 WORKSPACE CHANGES "
    };
    let changes_widget = Paragraph::new(change_lines).block(
        Block::default()
            .title(Span::styled(
                left_title,
                Style::default()
                    .fg(Color::Rgb(189, 147, 249))
                    .add_modifier(Modifier::BOLD),
            ))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Rgb(189, 147, 249)))
            .border_type(ratatui::widgets::BorderType::Rounded),
    );
    f.render_widget(changes_widget, area);
}
