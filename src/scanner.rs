/**
 * @file: scanner.rs
 * @author: Krisna Pranav
*/

use crate::{
    hasher,
    report::{self, ScanResult},
    sigdb::SignatureDb,
};
use anyhow::Result;
use std::path::Path;
use walkdir::WalkDir;

pub fn scan_file<'a>(
    path:  &'a Path,
    sigdb: &SignatureDb,
) -> Result<ScanResult<'a>> {
    let hashes = hasher::hash_file(path)?;

    for (hash, _kind) in [
        (&hashes.sha256, "SHA256"),
        (&hashes.sha1,   "SHA1"),
        (&hashes.md5,    "MD5"),
    ] {
        if let Some(sig) = sigdb.lookup(hash) {
            return Ok(ScanResult::detected(path, sig, hash));
        }
    }

    Ok(ScanResult::clean(path))
}

pub fn scan_directory(dir_path: &Path, sigdb: &SignatureDb) -> Result<(usize, usize)> {
    let mut total    = 0usize;
    let mut detected = 0usize;

    for entry in WalkDir::new(dir_path)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let path   = entry.path();
        let result = scan_file(path, sigdb)?;

        if result.is_malicious() { detected += 1; }
        result.print();
        total += 1;
    }

    report::print_summary(total, detected);
    Ok((total, detected))
}

pub fn scan_single(file_path: &Path, sigdb: &SignatureDb) -> Result<()> {
    println!("  {} {}\n", "Scanning:", file_path.display());

    let result = scan_file(file_path, sigdb)?;
    result.print();
    report::print_summary(1, usize::from(result.is_malicious()));
    Ok(())
}