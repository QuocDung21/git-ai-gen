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

use crate::cli::logger;
use crate::helper::Helper;
use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

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
    #[arg(value_name = "PATH")]
    path: Option<PathBuf>,

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

    #[command(visible_alias = "ct", visible_alias = "trash")]
    ClearTrash {
        #[arg(long = "node-modules", visible_alias = "nm")]
        node_modules: bool,

        #[arg(long = "build-folders", visible_alias = "bf")]
        build_folders: bool,

        #[arg(long, visible_alias = "pick")]
        select: bool,

        #[arg(value_name = "PATH", default_value = ".")]
        path: PathBuf,
    },
}

// =========================================================================
// MAIN & ROUTING
// =========================================================================

fn main() -> Result<()> {
    cli_log::init_cli_log!();

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
        Some(Commands::ClearTrash {
            node_modules,
            build_folders,
            select,
            path,
        }) => crate::cli::clear_trash::handle_clear_trash(
            path,
            *node_modules,
            *build_folders,
            *select,
        )?,
        None => {
            if let Some(path) = &cli.path {
                std::env::set_current_dir(path)?;
            }
            crate::app::events::run_dashboard()?;
        }
    }
    Ok(())
}
