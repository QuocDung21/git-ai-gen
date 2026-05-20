use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame, Terminal,
};
use std::process::Command;
use std::{io, time::Duration};

// --- QUẢN LÝ TRẠNG THÁI CỦA DASHBOARD ---
struct App {
    status_message: String,
    git_status_lines: Vec<String>,
    current_lang: String,
}

impl App {
    fn new() -> Self {
        let mut app = App {
            status_message: "Sẵn sàng tạo Commit Message!".to_string(),
            git_status_lines: Vec::new(),
            current_lang: crate::get_ai_language(),
        };
        app.refresh_git_status();
        app
    }

    fn refresh_git_status(&mut self) {
        self.git_status_lines.clear();
        if let Ok(output) = Command::new("git").args(["status", "-s"]).output() {
            let status_text = String::from_utf8_lossy(&output.stdout);
            if status_text.trim().is_empty() {
                self.git_status_lines
                    .push("✅ Thư mục làm việc sạch sẽ (Không có thay đổi).".to_string());
            } else {
                for line in status_text.lines() {
                    self.git_status_lines.push(format!(" {}", line));
                }
            }
        } else {
            self.git_status_lines
                .push("❌ Không thể đọc trạng thái Git.".to_string());
        }
    }
}

pub fn run_dashboard() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    res?;
    Ok(())
}

fn run_app<B: Backend + std::io::Write>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()), // Thoát
                    KeyCode::Char('d') => match crate::handle_diff() {
                        // Diff
                        Ok(msg) => app.status_message = msg,
                        Err(e) => app.status_message = format!("❌ Lỗi: {}", e),
                    },
                    KeyCode::Char('g') => {
                        // Go
                        run_cli_command(terminal, || crate::handle_go())?;
                        app.refresh_git_status();
                        app.status_message = "✅ Đã xử lý xong tác vụ Go!".to_string();
                    }
                    KeyCode::Char('r') => {
                        // Reset
                        run_cli_command(terminal, || crate::handle_restore())?;
                        app.refresh_git_status();
                        app.status_message = "🔄 Đã reset cấu hình hệ thống.".to_string();
                    }
                    // Trong vòng lặp run_app ở dashboard.rs
                    KeyCode::Char('l') => {
                        let next_lang = if app.current_lang == "Vietnamese" {
                            "en"
                        } else {
                            "vi"
                        };
                        match crate::handle_lang(next_lang) {
                            Ok(msg) => {
                                app.status_message = msg;
                                app.current_lang = crate::get_ai_language();
                            }
                            Err(e) => app.status_message = format!("❌ Lỗi: {}", e),
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

fn run_cli_command<B: Backend + std::io::Write, F>(
    terminal: &mut Terminal<B>,
    mut cmd: F,
) -> Result<()>
where
    F: FnMut() -> Result<()>,
{
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    print!("{}[2J{}[1;1H", 27 as char, 27 as char);

    if let Err(e) = cmd() {
        println!("❌ Đã xảy ra lỗi: {}", e);
    }

    println!("\n👉 Nhấn phím Enter để quay lại Dashboard...");
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf)?;

    enable_raw_mode()?;
    execute!(terminal.backend_mut(), EnterAlternateScreen)?;
    terminal.clear()?;

    Ok(())
}

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(f.size());

    // 1. HEADER
    let lang_display = format!(" [Lang: {}] ", app.current_lang);
    let header_text = Line::from(vec![
        Span::styled(
            " 🤖 ULTIMATE GIT-AI ",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" | Bảng điều khiển |"),
        Span::styled(
            lang_display,
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
    ]);
    let header = Paragraph::new(header_text).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan)),
    );
    f.render_widget(header, chunks[0]);

    // 2. MAIN CONTENT
    let mut status_text = vec![
        Line::from(Span::styled(
            "📂 Trạng thái Git hiện tại:",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
    ];

    for line in &app.git_status_lines {
        let color = if line.contains(" M") || line.contains("M ") {
            Color::Yellow
        } else if line.contains(" A") || line.contains("A ") || line.contains("??") {
            Color::Green
        } else if line.contains(" D") || line.contains("D ") {
            Color::Red
        } else {
            Color::White
        };
        status_text.push(Line::from(Span::styled(
            line.clone(),
            Style::default().fg(color),
        )));
    }

    status_text.push(Line::from(""));
    status_text.push(Line::from(Span::styled(
        &app.status_message,
        Style::default()
            .fg(Color::Blue)
            .add_modifier(Modifier::BOLD),
    )));

    let main_content = Paragraph::new(status_text)
        .block(Block::default().title(" Tổng quan ").borders(Borders::ALL))
        .wrap(Wrap { trim: true });
    f.render_widget(main_content, chunks[1]);

    // 3. FOOTER
    // 3. FOOTER
    let footer_text = Line::from(vec![
        Span::styled(
            "[d]",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" Diff | "),
        Span::styled(
            "[g]",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" Go | "),
        Span::styled(
            "[l]",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" Lang | "), // Phím tắt mới
        Span::styled(
            "[r]",
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        ),
        Span::raw(" Reset | "),
        Span::styled(
            "[q]",
            Style::default()
                .fg(Color::Gray)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" Thoát"),
    ]);
    let footer = Paragraph::new(footer_text)
        .alignment(ratatui::layout::Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray)),
        );
    f.render_widget(footer, chunks[2]);
}
