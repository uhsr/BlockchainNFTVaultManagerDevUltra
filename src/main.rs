// src/main.rs
/*
 * Main executable for BlockchainNFTVaultManagerDevUltra
 */

use clap::Parser;
use blockchainnftvaultmanagerdevultra::{Result, run};

#[derive(Parser)]
#[command(version, about = "BlockchainNFTVaultManagerDevUltra - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
