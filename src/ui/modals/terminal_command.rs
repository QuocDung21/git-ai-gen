use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};
use crate::app::App;
use crate::ui::modals::centered_rect;

pub fn render_terminal_command(f: &mut Frame, app: &App, area: Rect) {
    let theme = app.theme();
    let is_vi = app.current_lang == "vi";
    
    let popup_area = centered_rect(60, 30, area);
    f.render_widget(Clear, popup_area);

    let title = if is_vi { "💻 Chạy lệnh Terminal" } else { "💻 Run Terminal Command" };
    let prompt_label = if is_vi { "Nhập lệnh của bạn" } else { "Enter your command" };
    let footer = if is_vi { 
        "Enter: Chạy lệnh | Esc: Đóng" 
    } else { 
        "Enter: Run command | Esc: Close" 
    };

    let block = Block::default()
        .title(Span::styled(title, Style::default().fg(theme.purple).add_modifier(Modifier::BOLD)))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.purple));

    let mut text = vec![
        Line::from(vec![
            Span::styled(format!("{}: ", prompt_label), Style::default().fg(theme.cyan)),
            Span::styled(&app.prompt_text, Style::default().fg(theme.fg)),
            Span::styled("_", Style::default().fg(theme.purple).add_modifier(Modifier::SLOW_BLINK)),
        ]),
        Line::from(""),
    ];

    if is_vi {
         text.push(Line::from(vec![
            Span::styled("ℹ️ Tip: ", Style::default().fg(theme.yellow)),
            Span::styled("Nhập lệnh terminal bạn muốn thực thi trực tiếp trên hệ thống.", Style::default().fg(theme.fg).add_modifier(Modifier::ITALIC)),
        ]));
    } else {
        text.push(Line::from(vec![
            Span::styled("ℹ️ Tip: ", Style::default().fg(theme.yellow)),
            Span::styled("Enter the terminal command you want to execute directly on your system.", Style::default().fg(theme.fg).add_modifier(Modifier::ITALIC)),
        ]));
    }

    let paragraph = Paragraph::new(text)
        .block(block)
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, popup_area);

    // Footer info
    let footer_area = Rect {
        x: popup_area.x + 2,
        y: popup_area.y + popup_area.height - 2,
        width: popup_area.width - 4,
        height: 1,
    };
    let footer_widget = Paragraph::new(footer)
        .style(Style::default().fg(theme.border).add_modifier(Modifier::DIM));
    f.render_widget(footer_widget, footer_area);
}

pub fn render_terminal_result(f: &mut Frame, app: &App, result: &str, area: Rect) {
    let theme = app.theme();
    let is_vi = app.current_lang == "vi";
    
    let popup_area = centered_rect(70, 40, area);
    f.render_widget(Clear, popup_area);

    let title = if is_vi { "📋 Kết quả thực thi" } else { "📋 Execution Result" };
    let footer = if is_vi { 
        "C: Copy kết quả | Esc/Enter: Đóng" 
    } else { 
        "C: Copy output | Esc/Enter: Close" 
    };

    let block = Block::default()
        .title(Span::styled(title, Style::default().fg(theme.green).add_modifier(Modifier::BOLD)))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.green));

    let mut text = vec![
        Line::from(""),
    ];

    for line in result.lines() {
        text.push(Line::from(vec![
            Span::styled("  ", Style::default()),
            Span::styled(line, Style::default().fg(theme.fg).add_modifier(Modifier::BOLD)),
        ]));
    }

    text.push(Line::from(""));

    let paragraph = Paragraph::new(text)
        .block(block)
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, popup_area);

    // Footer info
    let footer_area = Rect {
        x: popup_area.x + 2,
        y: popup_area.y + popup_area.height - 2,
        width: popup_area.width - 4,
        height: 1,
    };
    let footer_widget = Paragraph::new(footer)
        .style(Style::default().fg(theme.border).add_modifier(Modifier::DIM));
    f.render_widget(footer_widget, footer_area);
}
