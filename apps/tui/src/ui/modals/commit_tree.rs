use crate::app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};
use rust_i18n::t;

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
            let expanded = line.replace('\t', "    ");
            let color = if expanded.starts_with('+') && !expanded.starts_with("+++") {
                theme.green
            } else if expanded.starts_with('-') && !expanded.starts_with("---") {
                theme.red
            } else if expanded.starts_with("@@") {
                theme.cyan
            } else {
                theme.fg
            };
            right_content.push(Line::from(vec![Span::styled(
                format!(" {}", expanded),
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
