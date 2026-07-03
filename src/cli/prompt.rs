use crate::cli::logger;
use anyhow::Result;
use console::{style, Key, Term};
use std::io::{self, Write};

pub fn ask_confirm_default_no(prompt: &str) -> Result<bool> {
    print!("{}", style(prompt).yellow().bold());
    io::stdout().flush()?;

    let term = Term::stdout();
    loop {
        match term.read_key()? {
            Key::Char('y') | Key::Char('Y') => {
                logger::text("");
                return Ok(true);
            }
            Key::Enter | Key::Char('n') | Key::Char('N') | Key::Escape => {
                logger::text("");
                return Ok(false);
            }
            _ => {}
        }
    }
}

pub fn ask_confirm(prompt: &str) -> Result<bool> {
    print!("{}", style(prompt).yellow().bold());
    io::stdout().flush()?;

    let term = Term::stdout();
    loop {
        match term.read_key()? {
            Key::Enter | Key::Char('y') | Key::Char('Y') => {
                logger::text("");
                return Ok(true);
            }
            Key::Char('n') | Key::Char('N') | Key::Escape => {
                logger::text("");
                return Ok(false);
            }
            _ => {}
        }
    }
}

pub fn ask_confirm_default_yes(prompt: &str) -> Result<bool> {
    print!("{}", style(prompt).yellow().bold());
    io::stdout().flush()?;

    let term = Term::stdout();
    loop {
        match term.read_key()? {
            Key::Enter | Key::Char('y') | Key::Char('Y') => {
                logger::text("");
                return Ok(true);
            }
            Key::Char('n') | Key::Char('N') | Key::Escape => {
                logger::text("");
                return Ok(false);
            }
            _ => {}
        }
    }
}
