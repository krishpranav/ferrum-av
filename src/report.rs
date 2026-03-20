/**
 * @file: report.rs
 * @author: Krisna Pranav
*/

use crate::sigdb::Signature;
use colored::Colorize;
use std::path::Path;

#[derive(Debug)]
pub enum Verdict {
    Clean,
    Detected {
        hash_type:   String,
        hash:        String,
        threat_name: String,
    },
}

pub struct ScanResult<'a> {
    pub path:    &'a Path,
    pub verdict: Verdict,
}

impl<'a> ScanResult<'a> {
    pub fn clean(path: &'a Path) -> Self {
        Self { path, verdict: Verdict::Clean }
    }

    pub fn detected(path: &'a Path, sig: &Signature, hash: &str) -> Self {
        Self {
            path,
            verdict: Verdict::Detected {
                hash_type:   sig.hash_type.clone(),
                hash:        hash.to_string(),
                threat_name: sig.threat_name.clone(),
            },
        }
    }

    pub fn is_malicious(&self) -> bool {
        matches!(self.verdict, Verdict::Detected { .. })
    }

    pub fn print(&self) {
        match &self.verdict {
            Verdict::Clean => {
                println!(
                    "  {}  {}",
                    "CLEAN".green().bold(),
                    self.path.display()
                );
            }

            Verdict::Detected { hash_type, hash, threat_name } => {
                println!(
                    "  {} {}",
                    "DETECTED".red().bold(),
                    self.path.display()
                );
                println!("           {} : {}", hash_type.cyan(), hash.dimmed());
                println!("           {} : {}", "Threat".cyan(), threat_name.yellow().bold());
            }
        }
    }
}

pub fn print_summary(total: usize, hits: usize) {
    println!();
    println!(
        "  {} scanned — {} {}",
        total,
        hits,
        if hits == 0 {
            "threats found".green().bold().to_string()
        } else {
            format!("{} threat(s) detected", hits).red().bold().to_string()
        }
    );
}

pub fn print_banner() {
    println!(
        "\n  {} {}\n",
        "⬡ Ferrum AV".bold().white(),
        "v0.1.0".dimmed()
    );
}