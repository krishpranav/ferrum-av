/**
 * @file: sigdb.rs
 * @author: Krisna Pranav
*/

use crate::error::FerrumError;
use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Write},
    path::Path,
};

#[derive(Debug, Clone)]
pub struct Signature {
    pub hash_type:   String,
    pub hash:        String,
    pub threat_name: String,
}

pub struct SignatureDb {
    entries: HashMap<String, Signature>,
}

impl SignatureDb {
    pub fn load(path: &Path) -> Result<Self, FerrumError> {
        if !path.exists() {
            return Err(FerrumError::DatabaseNotFound(
                path.display().to_string(),
            ));
        }

        let file    = File::open(path)?;
        let reader  = BufReader::new(file);
        let mut map = HashMap::new();

        for line in reader.lines() {
            let line = line?;
            let trimmed = line.trim();

            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }

            let parts: Vec<&str> = trimmed.splitn(3, '|').collect();
            if parts.len() != 3 {
                return Err(FerrumError::InvalidSignature(trimmed.to_string()));
            }

            let sig = Signature {
                hash_type:   parts[0].trim().to_uppercase(),
                hash:        parts[1].trim().to_lowercase(),
                threat_name: parts[2].trim().to_string(),
            };

            map.insert(sig.hash.clone(), sig);
        }

        Ok(Self { entries: map })
    }

    pub fn lookup(&self, hash: &str) -> Option<&Signature> {
        self.entries.get(&hash.to_lowercase())
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn append_entry(
        path:        &Path,
        hash_type:   &str,
        hash:        &str,
        threat_name: &str,
    ) -> Result<(), FerrumError> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?;

        writeln!(file, "{}|{}|{}", hash_type.to_uppercase(), hash.to_lowercase(), threat_name)?;
        Ok(())
    }
}