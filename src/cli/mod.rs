pub mod args;
pub mod clear_trash;
pub mod install;
pub mod logger;
pub mod profile;
pub mod prompt;
pub mod router;
pub mod spinner;
pub mod system;
pub mod uninstall;

pub use crate::locales::Locales;
pub use profile::{append_to_file, clean_profile_file};
pub use prompt::{ask_confirm, ask_confirm_default_no};

#[cfg(target_family = "unix")]
pub use profile::get_active_unix_profile;

#[cfg(target_os = "windows")]
pub use profile::get_windows_profile;

pub fn print_commands_help(locales: &Locales) {
    logger::info(&locales.cmd_help_diff);
    logger::info(&locales.cmd_help_go);
    logger::info(&locales.cmd_help_clear_trash);
    logger::info(&locales.cmd_help_un);
    logger::info(&locales.cmd_help_base);
}
