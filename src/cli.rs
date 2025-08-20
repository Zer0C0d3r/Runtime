//! Command line interface matching the exact behavior of standard uptime
//!
//! This module handles argument parsing to exactly match uptime's behavior

use clap::{Args, Parser};
use runtime::{OutputFormat, RuntimeArgs};

#[derive(Parser)]
#[command(
    version,
    about = "* Modern colorful uptime utility with interactive dashboard *",
    long_about = "A modern replacement for the classic uptime command with beautiful colors,\nanimations, and multiple output formats."
)]
struct Cli {
    #[command(flatten)]
    output: Outputs,

    /// Show container uptime indicators
    #[arg(short, long, default_value_t = false)]
    container: bool,
}

#[derive(Args)]
#[group(multiple = false)]
struct Outputs {
    /// Show interactive colorful dashboard (default)
    #[arg(short, long)]
    interactive: bool,

    /// Show uptime values in raw machine-readable format
    #[arg(short, long)]
    raw: bool,

    /// Show uptime in pretty human-readable format
    #[arg(short, long)]
    pretty: bool,

    /// Show system boot timestamp
    #[arg(short, long)]
    since: bool,

    /// Show standard uptime format (like original uptime)
    #[arg(long)]
    standard: bool,
}
/// Parse command line arguments
///
/// # Returns
/// `RuntimeArgs` struct containing parsed arguments
pub fn parse_args() -> RuntimeArgs {
    let cli = Cli::parse();
    let output = &cli.output;
    let format = match (
        output.standard,
        output.pretty,
        output.raw,
        output.since,
        output.interactive,
    ) {
        (true, _, _, _, _) => OutputFormat::Standard,
        (_, true, _, _, _) => OutputFormat::Pretty,
        (_, _, true, _, _) => OutputFormat::Raw,
        (_, _, _, true, _) => OutputFormat::Since,
        _ => OutputFormat::Interactive,
    };

    RuntimeArgs {
        format,
        show_container: cli.container,
    }
}
