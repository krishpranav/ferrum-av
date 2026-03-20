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
        let mut count    = 0usize;

        for pattern in &[
            format!("{}/**/*.yar",  rules_dir.display()),
            format!("{}/**/*.yara", rules_dir.display()),
        ] {
            for entry in glob(pattern).map_err(|e| FerrumError::YaraCompile(e.to_string()))? {
                let path = entry.map_err(|e| FerrumError::YaraCompile(e.to_string()))?;
                let src  = std::fs::read_to_string(&path)?;

                compiler
                    .add_source(src.as_str())
                    .map_err(|e| FerrumError::YaraCompile(format!("{}: {e}", path.display())))?;

                count += 1;
            }
        }

        if count == 0 {
            return Ok(None);
        }

        let rules = compiler.build();
        Ok(Some(Self { rules }))
    }

    pub fn scan_file(&self, path: &Path) -> Result<Vec<YaraMatch>, FerrumError> {
        let mut scanner = Scanner::new(&self.rules);

        let bytes = std::fs::read(path)?;
        let results = scanner
            .scan(&bytes)
            .map_err(|e| FerrumError::YaraScan(e.to_string()))?;

        let matches = results
            .matching_rules()
            .map(|rule| YaraMatch {
                rule:      rule.identifier().to_string(),
                namespace: rule.namespace().to_string(),
                tags:      rule.tags().map(|t| t.identifier().to_string()).collect(),
            })
            .collect();

        Ok(matches)
    }
}

#[derive(Debug, Clone)]
pub struct YaraMatch {
    pub rule:      String,
    pub namespace: String,
    pub tags:      Vec<String>,
}