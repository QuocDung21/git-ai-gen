use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "git-ai",
    version,
    about = "🤖 ULTIMATE GIT-AI CLI\nA tool to help you write Git Commits using AI rapidly."
)]
pub struct Cli {
    #[arg(value_name = "PATH")]
    pub path: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
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
