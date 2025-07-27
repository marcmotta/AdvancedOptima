// src/main.rs
/*
 * Main executable for AdvancedOptima
 */

use clap::Parser;
use advancedoptima::{Result, run};

#[derive(Parser)]
#[command(version, about = "AdvancedOptima - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
