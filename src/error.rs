/**
 * @file: error.rs
 * @author: Krisna Pranav
*/

use std::fmt;

#[derive(Debug)]
pub enum FerrumError {
    Io(std::io::Error),
    InvalidSignature(String),
    DatabaseNotFound(String),
}

impl fmt::Display for FerrumError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(e)                  => write!(f, "I/O error: {e}"),
            Self::InvalidSignature(line) => write!(f, "malformed signature line: {line}"),
            Self::DatabaseNotFound(path) => write!(f, "signature database not found: {path}"),
        }
    }
}

impl std::error::Error for FerrumError {}

impl From<std::io::Error> for FerrumError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}