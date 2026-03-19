/**
 * @file: hasher.rs
 * @author: Krisna Pranav
*/

use anyhow::Result;
use digest::Digest;
use md5::Md5;
use sha1::Sha1;
use sha2::Sha256;
use std::{fs::File, io::Read, path::Path};

const CHUNK: usize = 65_536;

pub struct FileHashes {
    pub md5:    String,
    pub sha1:   String,
    pub sha256: String,
}

pub fn hash_file(path: &Path) -> Result<FileHashes> {
    let mut file = File::open(path)?;
    let mut buf  = vec![0u8; CHUNK];

    let mut h_md5    = Md5::new();
    let mut h_sha1   = Sha1::new();
    let mut h_sha256 = Sha256::new();

    loop {
        let n = file.read(&mut buf)?;
        if n == 0 { break; }

        h_md5.update(&buf[..n]);
        h_sha1.update(&buf[..n]);
        h_sha256.update(&buf[..n]);
    }

    Ok(FileHashes {
        md5:    hex::encode(h_md5.finalize()),
        sha1:   hex::encode(h_sha1.finalize()),
        sha256: hex::encode(h_sha256.finalize()),
    })
}

pub fn detect_hash_type(hash: &str) -> Option<&'static str> {
    match hash.len() {
        32  => Some("MD5"),
        40  => Some("SHA1"),
        64  => Some("SHA256"),
        _   => None,
    }
}