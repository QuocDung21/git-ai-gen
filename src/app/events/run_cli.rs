use anyhow::Result;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::backend::Backend;
use ratatui::Terminal;

use std::io::Write;

pub fn run_cli_command<B: Backend + std::io::Write, F>(
    terminal: &mut Terminal<B>,
    mut cmd: F,
) -> Result<()>
where
    F: FnMut() -> Result<()>,
{
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        crossterm::event::DisableBracketedPaste
    )?;
    terminal.show_cursor()?;
    std::io::Write::flush(terminal.backend_mut())?;

    disable_raw_mode()?;

    let mut stdout = std::io::stdout();
    execute!(
        stdout,
        crossterm::terminal::Clear(crossterm::terminal::ClearType::All),
        crossterm::cursor::MoveTo(0, 0)
    )?;
    stdout.flush()?;

    if let Err(e) = cmd() {
        println!("❌ Error: {}", e);
    }

    println!("\n👉 Press Enter to return to Dashboard...");
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf)?;

    enable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        EnterAlternateScreen,
        crossterm::event::EnableBracketedPaste
    )?;
    terminal.clear()?;

    Ok(())
}
