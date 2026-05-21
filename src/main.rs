mod app;
mod cli;
mod constant;
mod git;
mod helper;
mod ui;

use crate::helper::Helper;
use crate::cli::logger;
use anyhow::Result;
use clap::{Parser, Subcommand};

// =========================================================================
// CLAP CLI CONFIGURATION
// =========================================================================

#[derive(Parser)]
#[command(
    name = "git-ai",
    version,
    about = "🤖 ULTIMATE GIT-AI CLI\nA tool to help you write Git Commits using AI rapidly."
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(visible_alias = "d")]
    Diff,

    #[command(visible_alias = "g")]
    Go,

    #[command(visible_alias = "l")]
    Lang { lang: String },

    #[command(visible_alias = "i")]
    Install,

    #[command(visible_alias = "u")]
    Uninstall,

    #[command(visible_alias = "r")]
    Reset,

    #[command(visible_alias = "t")]
    Test,
}

// =========================================================================
// MAIN & ROUTING
// =========================================================================

fn main() -> Result<()> {
    let cli = Cli::parse();
    let locales = Helper::get_locales();

    if let Err(e) = run(&cli, &locales) {
        logger::error(&format!("{} {}", locales.error_prefix, e));
    }
    Ok(())
}

fn run(cli: &Cli, locales: &crate::cli::Locales) -> Result<()> {
    match &cli.command {
        Some(Commands::Diff) => {
            let msg = crate::cli::system::handle_diff(locales)?;
            logger::system(&msg);
        }
        Some(Commands::Go) => crate::cli::system::handle_go(locales)?,
        Some(Commands::Lang { lang }) => {
            let msg = crate::cli::system::handle_lang(lang, locales)?;
            println!("{}", msg);
        }
        Some(Commands::Install) => crate::cli::install::handle_install()?,
        Some(Commands::Uninstall) => crate::cli::uninstall::handle_uninstall()?,
        Some(Commands::Reset) => crate::cli::system::handle_restore(locales)?,
        Some(Commands::Test) => crate::cli::system::handle_test()?,
        None => {
            crate::app::events::run_dashboard()?;
        }
    }
    Ok(())
}
