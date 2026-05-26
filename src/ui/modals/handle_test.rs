use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};

use crate::app::App;

pub fn render_handle_test(f: &mut Frame, app: &App, area: Rect) {
    let theme = app.theme();
    f.render_widget(Clear, area);

    let outer_block = Block::default()
        .title(Span::styled(
            app.tr(" 🛠️ DEV TESTING PLAYGROUND ", " 🛠️ DEV TESTING PLAYGROUND "),
            Style::default().fg(theme.yellow).add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.yellow))
        .border_type(BorderType::Double)
        .style(Style::default().bg(theme.bg));

    f.render_widget(outer_block.clone(), area);
    let inner = outer_block.inner(area);

    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(0),
            Constraint::Length(2),
        ])
        .split(inner);

    let header_p = Paragraph::new(vec![
        Line::from(vec![Span::styled(
            app.tr(
                "  BẢNG ĐIỀU KHIỂN & CHẨN ĐOÁN DÀNH CHO NHÀ PHÁT TRIỂN",
                "  INTERACTIVE DEVELOPMENT & DIAGNOSTIC PLAYGROUND"
            ),
            Style::default().fg(theme.fg).add_modifier(Modifier::BOLD),
        )]),
        Line::from(Span::styled(
            app.tr(
                "  -------------------------------------------------------------",
                "  -------------------------------------------------------------"
            ),
            Style::default().fg(theme.border),
        )),
    ]);
    f.render_widget(header_p, main_layout[0]);

    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(main_layout[1]);

    let diag_lines = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled(app.tr("  📂 Thư mục hiện tại: ", "  📂 Current Directory: "), Style::default().fg(theme.cyan)),
            Span::styled(app.current_dir.clone(), Style::default().fg(theme.fg)),
        ]),
        Line::from(vec![
            Span::styled(app.tr("  🌐 Ngôn ngữ: ", "  🌐 Active Language: "), Style::default().fg(theme.cyan)),
            Span::styled(app.current_lang.clone(), Style::default().fg(theme.fg)),
        ]),
        Line::from(vec![
            Span::styled(app.tr("  🎨 Mã Theme: ", "  🎨 Active Theme ID: "), Style::default().fg(theme.cyan)),
            Span::styled(app.theme_id.clone(), Style::default().fg(theme.fg)),
        ]),
        Line::from(vec![
            Span::styled(app.tr("  ☀️ Chế độ sáng: ", "  ☀️ Light Mode Active: "), Style::default().fg(theme.cyan)),
            Span::styled(app.is_light_theme.to_string(), Style::default().fg(theme.fg)),
        ]),
        Line::from(vec![
            Span::styled(app.tr("  📝 Số thay đổi: ", "  📝 Changed Files count: "), Style::default().fg(theme.cyan)),
            Span::styled(app.files.len().to_string(), Style::default().fg(theme.fg)),
        ]),
        Line::from(vec![
            Span::styled(app.tr("  📊 Ngôn ngữ quét: ", "  📊 Scanned Languages: "), Style::default().fg(theme.cyan)),
            Span::styled(format!("{} entries", app.language_stats.len()), Style::default().fg(theme.fg)),
        ]),
        Line::from(vec![
            Span::styled(app.tr("  💻 Lệnh IDE mở: ", "  💻 Default Open IDE: "), Style::default().fg(theme.cyan)),
            Span::styled(app.editor.clone(), Style::default().fg(theme.fg)),
        ]),
        Line::from(vec![
            Span::styled(app.tr("  ⚡ Cảnh báo Xung đột: ", "  ⚡ Mock Merge Conflict: "), Style::default().fg(theme.cyan)),
            Span::styled(
                if app.has_conflicts {
                    format!("Yes ({} conflicts)", app.conflict_count)
                } else {
                    "No".to_string()
                },
                if app.has_conflicts {
                    Style::default().fg(theme.red).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(theme.green)
                }
            ),
        ]),
    ];

    let diag_block = Block::default()
        .title(Span::styled(
            app.tr(" 🔍 THÔNG TIN CHẨN ĐOÁN ", " 🔍 SYSTEM DIAGNOSTICS "),
            Style::default().fg(theme.cyan).add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.border))
        .border_type(BorderType::Rounded);

    let diag_p = Paragraph::new(diag_lines).block(diag_block);
    f.render_widget(diag_p, columns[0]);

    let play_lines = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("   ⚡ [1] : ", Style::default().fg(theme.border)),
            Span::styled(app.tr("Đổi giao diện Sáng / VS Code", "Toggle Light / VS Code theme"), Style::default().fg(theme.fg)),
        ]),
        Line::from(vec![
            Span::styled("   ⚡ [2] : ", Style::default().fg(theme.border)),
            Span::styled(app.tr("Gửi thông báo giả lập", "Trigger mock success alert"), Style::default().fg(theme.fg)),
        ]),
        Line::from(vec![
            Span::styled("   ⚡ [3] : ", Style::default().fg(theme.border)),
            Span::styled(app.tr("Dọn dữ liệu tỷ lệ ngôn ngữ", "Reset language stats array"), Style::default().fg(theme.fg)),
        ]),
        Line::from(vec![
            Span::styled("   ⚡ [4] : ", Style::default().fg(theme.border)),
            Span::styled(app.tr("Giả lập quét thay đổi Diff", "Simulate mock diff capture"), Style::default().fg(theme.fg)),
        ]),
        Line::from(vec![
            Span::styled("   ⚡ [5] : ", Style::default().fg(theme.border)),
            Span::styled(app.tr("Xuất JSON FFI lịch sử", "Dump FFI workspace history JSON"), Style::default().fg(theme.fg)),
        ]),
        Line::from(vec![
            Span::styled("   ⚡ [6] : ", Style::default().fg(theme.border)),
            Span::styled(app.tr("Bật/tắt giả lập xung đột", "Toggle mock merge conflict"), Style::default().fg(theme.fg)),
        ]),
        Line::from(vec![
            Span::styled("   ⚡ [7] : ", Style::default().fg(theme.border)),
            Span::styled(app.tr("Đổi ngôn ngữ giao diện Việt/Anh", "Toggle bilingual Vietnamese/English"), Style::default().fg(theme.fg)),
        ]),
    ];

    let play_block = Block::default()
        .title(Span::styled(
            app.tr(" ⚙️ TRÌNH KIỂM THỬ TƯƠNG TÁC ", " ⚙️ INTERACTIVE PLAYGROUND "),
            Style::default().fg(theme.green).add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.border))
        .border_type(BorderType::Rounded);

    let play_p = Paragraph::new(play_lines).block(play_block);
    f.render_widget(play_p, columns[1]);

    let footer_p = Paragraph::new(Line::from(vec![Span::styled(
        app.tr(
            "   Nhấn [Esc] hoặc [q] để ĐÓNG bảng điều khiển thử nghiệm.",
            "   Press [Esc] or [q] to CLOSE developer testing playground."
        ),
        Style::default().fg(theme.orange).add_modifier(Modifier::BOLD),
    )]));
    f.render_widget(footer_p, main_layout[2]);
}
