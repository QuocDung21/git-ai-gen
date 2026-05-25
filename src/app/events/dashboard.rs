use anyhow::Result;
use crossterm::event::{self, Event};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use std::{io, time::Duration};

use crate::app::App;

use crate::app::events::handlers;

pub fn run_dashboard() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(
        stdout,
        EnterAlternateScreen,
        crossterm::event::EnableBracketedPaste
    )?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    let res = run_app(&mut terminal, &mut app);

    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        crossterm::event::DisableBracketedPaste
    )?;
    terminal.show_cursor()?;
    let _ = std::io::Write::flush(terminal.backend_mut());
    let _ = disable_raw_mode();

    res?;
    Ok(())
}

fn run_app<B: Backend + std::io::Write>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()> {
    loop {
        terminal.draw(|f| crate::ui::ui(f, app))?;

        // --- Long-running operations (pre-poll) ---
        if handlers::go_confirm::handle_go_pushing(app) {
            app.refresh_git_status();
            continue;
        }

        if handlers::go_confirm::handle_amend_pushing(app) {
            app.refresh_git_status();
            continue;
        }

        if handlers::diff_result::handle_kilo_generation(app) {
            continue;
        }

        if handlers::github::handle_github_cloning(app) {
            continue;
        }

        if event::poll(Duration::from_millis(250))? {
            match event::read()? {
                Event::Key(key) => {
                    if app.show_splash {
                        if key.code == event::KeyCode::Char('q') || key.code == event::KeyCode::Char('Q') {
                            std::process::exit(0);
                        }
                        app.show_splash = false;
                        continue;
                    }
                    if handlers::dispatch_modal_key(app, &key) {
                        continue;
                    }

                    // --- Standard controls (no modal or unhandled modal key) ---
                    handlers::navigation::handle_standard_keys(app, terminal, &key)?;
                }
                Event::Paste(text) => {
                    handlers::paste::handle_paste(app, &text);
                }
                _ => {}
            }
        }
    }
}

// Removed run_cli_command — it is now in the run_cli module
