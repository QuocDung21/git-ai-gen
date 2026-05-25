use anyhow::Result;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::backend::Backend;
use ratatui::Terminal;

pub fn run_cli_command<B: Backend + std::io::Write, F>(
    terminal: &mut Terminal<B>,
    mut cmd: F,
) -> Result<()>
where
    F: FnMut() -> Result<()>,
{
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        crossterm::event::DisableBracketedPaste
    )?;
    terminal.show_cursor()?;

    print!("{}[2J{}[1;1H", 27 as char, 27 as char);

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
