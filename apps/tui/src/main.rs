mod app;
mod cli;
mod ui;

pub use git_ai_core_shared::{cleanup, constant, git, helper, locales, models, theme};

rust_i18n::i18n!("../../core/git-ai-core/locales");

use crate::cli::args::Cli;
use crate::cli::logger;
use crate::helper::Helper;
use anyhow::Result;
use clap::Parser;

// =========================================================================
// MAIN & ROUTING
// =========================================================================

fn main() -> Result<()> {
    cli_log::init_cli_log!();

    let cli = Cli::parse();
    let locales = Helper::get_locales();

    if let Err(e) = crate::cli::router::run(&cli, &locales) {
        logger::error(&format!("{} {}", locales.error_prefix, e));
    }
    Ok(())
}
