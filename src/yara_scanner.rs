/**
 * @file: yara_scanner.rs
 * @author: Krisna Pranav
*/

use crate::error::FerrumError;
use glob::glob;
use std::path::Path;
use yara_x::Compiler;
use yara_x::Rules;
use yara_x::Scanner;

pub struct YaraEngine {
    rules: Rules,
}

impl YaraEngine {
    pub fn load(rules_dir: &Path) -> Result<Option<Self>, FerrumError> {
        if !rules_dir.exists() {
            return Ok(None);
        }

        let mut compiler = Compiler::new();
        let mut count = 0usize;

        if count == 0 {
            return Ok(None);
        }

        let rules = compiler.build();
        Ok(Some(Self { rules }))
    }
}

#[derive(Debug, Clone)]
pub struct YaraMatch {
    pub rule: String,
    pub namespace: String,
    pub tags: Vec<String>,
}