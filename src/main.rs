rust_i18n::i18n!("locales");

mod app;
mod cli;
mod constant;
mod ffi;
mod git;
mod helper;
mod locales;
mod models;
pub mod theme;
mod ui;

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
