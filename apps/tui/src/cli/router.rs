use anyhow::Result;

use crate::cli::args::{Cli, Commands};
use crate::cli::{clear_trash, install, logger, system, touch_id, uninstall, Locales};

pub fn run(cli: &Cli, locales: &Locales) -> Result<()> {
    match &cli.command {
        Some(Commands::Diff) => {
            let msg = system::handle_diff(locales)?;
            logger::system(&msg);
        }
        Some(Commands::Go) => system::handle_go(locales)?,
        Some(Commands::Lang { lang }) => {
            let msg = system::handle_lang(lang, locales)?;
            println!("{}", msg);
        }
        Some(Commands::Install) => install::handle_install()?,
        Some(Commands::Uninstall) => uninstall::handle_uninstall()?,
        Some(Commands::Reset) => system::handle_restore(locales)?,
        Some(Commands::Test) => system::handle_test()?,
        Some(Commands::EnableTouchIdSudo) => touch_id::handle_enable_touch_id_sudo()?,
        Some(Commands::ClearTrash {
            node_modules,
            build_folders,
            devcleaner,
            select,
            path,
        }) => clear_trash::handle_clear_trash(
            path,
            *node_modules,
            *build_folders,
            *devcleaner,
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
