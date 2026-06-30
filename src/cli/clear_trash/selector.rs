use anyhow::Result;
use console::{style, Key, Term};
use rust_i18n::t;
use std::io::{self, Write};
use std::path::PathBuf;

use crate::cli::logger;

use super::cleanup::CleanupTarget;

pub(super) fn select_cleanup_folders(
    target: CleanupTarget,
    folders: &[PathBuf],
) -> Result<Option<Vec<PathBuf>>> {
    let term = Term::stdout();
    let mut cursor = 0;
    let mut offset = 0;
    let mut selected = vec![false; folders.len()];

    loop {
        render_select_cleanup_folders(&term, target, folders, &selected, cursor, offset)?;

        match term.read_key()? {
            Key::ArrowUp => {
                cursor = cursor.saturating_sub(1);
                if cursor < offset {
                    offset = cursor;
                }
            }
            Key::ArrowDown => {
                if cursor + 1 < folders.len() {
                    cursor += 1;
                }
                let page_size = cleanup_select_page_size();
                if cursor >= offset + page_size {
                    offset = cursor + 1 - page_size;
                }
            }
            Key::Char(' ') => {
                selected[cursor] = !selected[cursor];
            }
            Key::Char('a') | Key::Char('A') => {
                let should_select = selected.iter().any(|selected| !selected);
                selected.fill(should_select);
            }
            Key::Enter => {
                let selected_paths = folders
                    .iter()
                    .zip(selected.iter())
                    .filter_map(|(path, selected)| selected.then(|| path.clone()))
                    .collect::<Vec<_>>();

                if selected_paths.is_empty() {
                    logger::warn(&t!("cleanup_select_empty").to_string());
                    continue;
                }

                term.clear_screen()?;
                return Ok(Some(selected_paths));
            }
            Key::Escape | Key::Char('q') | Key::Char('Q') => {
                term.clear_screen()?;
                return Ok(None);
            }
            _ => {}
        }
    }
}

fn render_select_cleanup_folders(
    term: &Term,
    target: CleanupTarget,
    folders: &[PathBuf],
    selected: &[bool],
    cursor: usize,
    offset: usize,
) -> Result<()> {
    term.clear_screen()?;
    logger::heading(&t!(target.select_title_key()).to_string());
    logger::info(&t!("cleanup_select_help").to_string());

    let page_size = cleanup_select_page_size();
    let end = folders.len().min(offset + page_size);

    for index in offset..end {
        let cursor_marker = if index == cursor { ">" } else { " " };
        let check_marker = if selected[index] { "[x]" } else { "[ ]" };
        println!(
            "{} {} {}",
            style(cursor_marker).cyan().bold(),
            style(check_marker).green(),
            folders[index].display()
        );
    }

    println!(
        "{}",
        style(format!(
            "{} {} / {}",
            t!("cleanup_select_counter"),
            selected.iter().filter(|selected| **selected).count(),
            folders.len()
        ))
        .yellow()
    );
    io::stdout().flush()?;

    Ok(())
}

fn cleanup_select_page_size() -> usize {
    15
}
