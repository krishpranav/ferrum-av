/**
 * @file: cli.rs
 * @author: Krisna Pranav
*/

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name    = "ferrum",
    version = "0.1.0",
    about   = "A transparent, hash-based antivirus engine",
    long_about = None,
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: FerrumCommand,
}

#[derive(Subcommand, Debug)]
pub enum FerrumCommand {
    Scan {
        #[arg(long, conflicts_with = "dir")]
        file: Option<String>,

        #[arg(long, conflicts_with = "file")]
        dir: Option<String>,

        #[arg(long, default_value = "signatures/signatures.txt")]
        sigdb: String,
    },

    AddHash {
        #[arg(long)]
        hash: String,

        #[arg(long)]
        name: String,

        #[arg(long, default_value = "signatures/signatures.txt")]
        sigdb: String,
    },

    Stats {
        #[arg(long, default_value = "signatures/signatures.txt")]
        sigdb: String,
    },
}