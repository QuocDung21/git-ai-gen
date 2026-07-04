mod dashboard;
pub mod handlers;
mod run_cli;

// Re-exported for binary target usage
#[allow(unused_imports)]
pub use dashboard::run_dashboard;
pub use run_cli::run_cli_command;
