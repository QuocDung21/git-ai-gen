use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};

use crate::app::App;

pub fn render_settings(f: &mut Frame, app: &App, area: Rect) {
    let theme = app.theme();
    let is_vi = app.current_lang == "vi";
    f.render_widget(Clear, area);

    let mut content = vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            if is_vi {
                "  ⚙️  CÀI ĐẶT HỆ THỐNG"
            } else {
                "  ⚙️  SYSTEM SETTINGS"
            },
            Style::default()
                .fg(theme.purple)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
    ];

    let options = vec![
        (
            app.auto_push,
            if is_vi {
                "Tự động đẩy code lên Remote (Push)"
            } else {
                "Auto Push committed changes to Remote"
            },
        ),
        (
            app.auto_stage_all,
            if is_vi {
                "Tự động Stage tất cả thay đổi"
            } else {
                "Auto Stage all unstaged changes"
            },
        ),
        (
            app.kilo_ai_enabled,
            if is_vi {
                "Sử dụng Kilo AI tạo Commit Message"
            } else {
                "Enable Kilo AI for Commit Messages"
            },
        ),
        (
            app.splash_enabled,
            if is_vi {
                "Hiển thị màn hình chào mừng (Splash)"
            } else {
                "Show startup splash screen"
            },
        ),
    ];

    for (i, (enabled, text)) in options.iter().enumerate() {
        let is_selected = i == app.selected_setting_index;
        let checkbox = if *enabled { "[X]" } else { "[ ]" };
        
        let prefix = if is_selected { " ➜ " } else { "   " };
        let style = if is_selected {
            Style::default()
                .fg(theme.select_fg)
                .bg(theme.select_bg)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(theme.fg)
        };

        let box_color = if *enabled { theme.green } else { theme.red };

        content.push(Line::from(vec![
            Span::styled(prefix, Style::default().fg(theme.purple).add_modifier(Modifier::BOLD)),
            Span::styled(format!("{} ", checkbox), Style::default().fg(box_color).add_modifier(Modifier::BOLD)),
            Span::styled(text.to_string(), style),
        ]));
        content.push(Line::from(""));
    }

    content.push(Line::from(vec![Span::styled(
        if is_vi {
            "  [↑/↓] Di chuyển  [Space/Enter] Bật/Tắt  [Esc] Đóng"
        } else {
            "  [↑/↓] Navigate   [Space/Enter] Toggle   [Esc] Close"
        },
        Style::default().fg(theme.border),
    )]));

    let block = Block::default()
        .title(Span::styled(
            if is_vi { " ⚙️ CÀI ĐẶT " } else { " ⚙️ SETTINGS " },
            Style::default()
                .fg(theme.purple)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.purple))
        .border_type(BorderType::Double)
        .style(Style::default().bg(theme.bg));

    let paragraph = Paragraph::new(content)
        .alignment(ratatui::layout::Alignment::Left)
        .block(block);
    f.render_widget(paragraph, area);
}
