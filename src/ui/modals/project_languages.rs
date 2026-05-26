use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};

use crate::app::App;

pub fn render_project_languages(f: &mut Frame, app: &App, area: Rect) {
    let theme = app.theme();
    f.render_widget(Clear, area);

    let mut content = vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            app.tr("  📊 TỶ LỆ NGÔN NGỮ DỰ ÁN 📊", "  📊 PROJECT LANGUAGE COMPOSITION 📊"),
            Style::default()
                .fg(theme.green)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
    ];

    if app.language_stats.is_empty() {
        content.push(Line::from(vec![Span::styled(
            app.tr("  (Chưa tải dữ liệu ngôn ngữ)", "  (Language data not loaded yet)"),
            Style::default()
                .fg(theme.border)
                .add_modifier(Modifier::ITALIC),
        )]));
    } else {
        let bar_width = (area.width as usize).saturating_sub(8);
        if bar_width > 0 {
            let mut bar_spans = Vec::new();
            let mut remaining_chars = bar_width;

            for (i, stat) in app.language_stats.iter().enumerate() {
                let count = if i == app.language_stats.len() - 1 {
                    remaining_chars
                } else {
                    let calc = ((stat.percentage / 100.0) * bar_width as f64).round() as usize;
                    calc.min(remaining_chars)
                };

                if count > 0 {
                    let color = Color::Rgb(stat.color_code.0, stat.color_code.1, stat.color_code.2);
                    let chars = "█".repeat(count);
                    bar_spans.push(Span::styled(chars, Style::default().fg(color)));
                    remaining_chars = remaining_chars.saturating_sub(count);
                }
            }

            if !bar_spans.is_empty() {
                let mut line_spans = vec![Span::styled("    ", Style::default())];
                line_spans.extend(bar_spans);
                content.push(Line::from(line_spans));
                content.push(Line::from(""));
            }
        }

        fn format_bytes(bytes: u64) -> String {
            if bytes >= 1_073_741_824 {
                format!("{:.2} GB", bytes as f64 / 1_073_741_824.0)
            } else if bytes >= 1_048_576 {
                format!("{:.2} MB", bytes as f64 / 1_048_576.0)
            } else if bytes >= 1024 {
                format!("{:.2} KB", bytes as f64 / 1024.0)
            } else {
                format!("{} B", bytes)
            }
        }

        for stat in &app.language_stats {
            let bullet_color = Color::Rgb(stat.color_code.0, stat.color_code.1, stat.color_code.2);
            let bullet = Span::styled("  ● ", Style::default().fg(bullet_color));

            let name_span = Span::styled(
                format!("{:<12}", stat.name),
                Style::default().fg(theme.fg).add_modifier(Modifier::BOLD),
            );

            let pct_span = Span::styled(
                format!(" {:>5.1}%", stat.percentage),
                Style::default().fg(theme.cyan),
            );

            let size_span = Span::styled(
                format!(" ({})", format_bytes(stat.bytes)),
                Style::default()
                    .fg(theme.border)
                    .add_modifier(Modifier::ITALIC),
            );

            content.push(Line::from(vec![bullet, name_span, pct_span, size_span]));
        }
    }

    content.push(Line::from(""));
    content.push(Line::from(vec![Span::styled(
        app.tr("  Nhấn [Esc] hoặc [q] để ĐÓNG màn hình thống kê.", "  Press [Esc] or [q] to CLOSE statistics screen."),
        Style::default()
            .fg(theme.orange)
            .add_modifier(Modifier::BOLD),
    )]));

    let block = Block::default()
        .title(Span::styled(
            app.tr(" 📊 THỐNG KÊ NGÔN NGỮ ", " 📊 LANGUAGE STATS "),
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
