/**
 * @file: cli.rs
 * @author: Krisna Pranav
*/

mod cli;
mod error;
mod hasher;
mod report;
mod scanner;
mod sigdb;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, FerrumCommand};
use colored::Colorize;
use sigdb::SignatureDb;
use std::path::Path;

fn main() -> Result<()> {
    let cli = Cli::parse();
    report::print_banner();

    match cli.command {
        FerrumCommand::Scan { file, dir, sigdb } => {
            let db_path = Path::new(&sigdb);
            let db      = SignatureDb::load(db_path).map_err(|e| anyhow::anyhow!("{e}"))?;

            println!(
                "  {} {} signatures loaded\n",
                "Database:".dimmed(),
                db.len().to_string().cyan()
            );

            match (file, dir) {
                (Some(f), _) => scanner::scan_single(Path::new(&f), &db)?,
                (_, Some(d)) => { scanner::scan_directory(Path::new(&d), &db)?; }
                _            => eprintln!("{}", "  error: provide --file or --dir".red()),
            }
        }

        FerrumCommand::AddHash { hash, name, sigdb } => {
            let hash_type = hasher::detect_hash_type(&hash)
                .ok_or_else(|| anyhow::anyhow!("unknown hash length: {}", hash.len()))?;

            let db_path = Path::new(&sigdb);
            SignatureDb::append_entry(db_path, hash_type, &hash, &name)
                .map_err(|e| anyhow::anyhow!("{e}"))?;

            println!(
                "  {} added {} → {}",
                "OK".green().bold(),
                hash.cyan(),
                name.yellow()
            );
        }

        FerrumCommand::Stats { sigdb } => {
            let db_path = Path::new(&sigdb);
            let db      = SignatureDb::load(db_path).map_err(|e| anyhow::anyhow!("{e}"))?;

            println!(
                "  {}  {} signatures\n  {}  {}",
                "Entries:".dimmed(), db.len().to_string().cyan().bold(),
                "Database:".dimmed(), db_path.display().to_string().dimmed()
            );
        }
    }

    println!();
    Ok(())
}