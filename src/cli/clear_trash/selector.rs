use anyhow::Result;
use console::{style, Key, Term};
use rust_i18n::t;
use std::io::{self, Write};

use crate::cleanup::{format_size_bytes, CleanupTarget, CleanupTask};
use crate::cli::logger;

pub(super) fn select_cleanup_folders(
    target: CleanupTarget,
    tasks: &[CleanupTask],
) -> Result<Option<Vec<CleanupTask>>> {
    let term = Term::stdout();
    let mut cursor = 0;
    let mut offset = 0;
    let mut selected = vec![false; tasks.len()];

    loop {
        render_select_cleanup_folders(&term, target, tasks, &selected, cursor, offset)?;

        match term.read_key()? {
            Key::ArrowUp => {
                cursor = cursor.saturating_sub(1);
                if cursor < offset {
                    offset = cursor;
                }
            }
            Key::ArrowDown => {
                if cursor + 1 < tasks.len() {
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
                let selected_tasks = tasks
                    .iter()
                    .zip(selected.iter())
                    .filter_map(|(task, selected)| selected.then(|| task.clone()))
                    .collect::<Vec<_>>();

                if selected_tasks.is_empty() {
                    logger::warn(&t!("cleanup_select_empty").to_string());
                    continue;
                }

                term.clear_screen()?;
                return Ok(Some(selected_tasks));
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
    tasks: &[CleanupTask],
    selected: &[bool],
    cursor: usize,
    offset: usize,
) -> Result<()> {
    term.clear_screen()?;
    logger::heading(&t!(target.select_title_key()).to_string());
    logger::info(&t!("cleanup_select_help").to_string());

    let page_size = cleanup_select_page_size();
    let end = tasks.len().min(offset + page_size);

    for index in offset..end {
        let cursor_marker = if index == cursor { ">" } else { " " };
        let check_marker = if selected[index] { "[x]" } else { "[ ]" };
        println!(
            "{} {} {} ({})",
            style(cursor_marker).cyan().bold(),
            style(check_marker).green(),
            tasks[index].path.display(),
            format_size_bytes(tasks[index].size_bytes)
        );
    }

    let selected_size = tasks
        .iter()
        .zip(selected.iter())
        .filter_map(|(task, selected)| selected.then_some(task.size_bytes))
        .sum::<u64>();

    println!(
        "{}",
        style(format!(
            "{} {} / {} | {} {}",
            t!("cleanup_select_counter"),
            selected.iter().filter(|selected| **selected).count(),
            tasks.len(),
            t!("cleanup_selected_size"),
            format_size_bytes(selected_size)
        ))
        .yellow()
    );
    io::stdout().flush()?;

    Ok(())
}

fn cleanup_select_page_size() -> usize {
    15
}
