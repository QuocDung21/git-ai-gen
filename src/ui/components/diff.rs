use crate::app::App;
use crate::models::DiffLineKind;
use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{
        Block, BorderType, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState,
    },
    Frame,
};
use rust_i18n::t;

pub fn render_diff(f: &mut Frame, app: &App, area: Rect) {
    let theme = app.theme();
    let diff_box_height = if area.height > 2 {
        (area.height - 2) as usize
    } else {
        0
    };
    let max_scroll = app
        .selected_file_diff_lines
        .len()
        .saturating_sub(diff_box_height);
    let scroll_offset = app.diff_scroll_offset.min(max_scroll);
    let mut diff_lines = Vec::new();

    if app.selected_file_diff_lines.is_empty() {
        diff_lines.push(Line::from(""));
        diff_lines.push(Line::from(vec![Span::styled(
            t!("diff_empty").to_string(),
            Style::default()
                .fg(theme.border)
                .add_modifier(Modifier::ITALIC),
        )]));
    } else {
        let max_len = (area.width as usize).saturating_sub(4).max(15);
        for line in app
            .selected_file_diff_lines
            .iter()
            .skip(scroll_offset)
            .take(diff_box_height)
        {
            let display_line = if line.text.chars().count() > max_len {
                line.text.chars().take(max_len).collect::<String>()
            } else {
                line.text.clone()
            };

            let style = match line.kind {
                DiffLineKind::Added => Style::default().fg(theme.green),
                DiffLineKind::Removed => Style::default().fg(theme.red),
                DiffLineKind::Hunk => Style::default()
                    .fg(theme.purple)
                    .add_modifier(Modifier::BOLD),
                DiffLineKind::Header => Style::default()
                    .fg(theme.border)
                    .add_modifier(Modifier::BOLD),
                DiffLineKind::Normal => Style::default().fg(theme.fg),
            };
            diff_lines.push(Line::from(vec![Span::styled(display_line, style)]));
        }
    }

    let scroll_info = if max_scroll > 0 {
        format!(
            " [{}/{}]",
            scroll_offset + 1,
            app.selected_file_diff_lines.len()
        )
    } else {
        "".to_string()
    };
    let diff_border_color = if app.focus_diff {
        theme.yellow
    } else {
        theme.border
    };

    let diff_base_title = if app.focus_diff {
        t!("diff_title_scroll")
    } else {
        t!("diff_title_normal")
    };
    let diff_title = format!("{}{} ", diff_base_title, scroll_info);

    let diff_widget = Paragraph::new(diff_lines)
        .style(Style::default().bg(theme.bg).fg(theme.fg))
        .block(
            Block::default()
                .title(Span::styled(
                    diff_title,
                    Style::default()
                        .fg(diff_border_color)
                        .add_modifier(Modifier::BOLD),
                ))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(diff_border_color))
                .border_type(BorderType::Rounded),
        );
    f.render_widget(diff_widget, area);

    if max_scroll > 0 {
        let scrollbar = Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("▲"))
            .end_symbol(Some("▼"))
            .track_symbol(Some("░"))
            .thumb_symbol("█");

        let mut scrollbar_state = ScrollbarState::new(max_scroll).position(scroll_offset);
        f.render_stateful_widget(
            scrollbar,
            area.inner(&ratatui::layout::Margin {
                vertical: 1,
                horizontal: 1,
            }),
            &mut scrollbar_state,
        );
    }
}
