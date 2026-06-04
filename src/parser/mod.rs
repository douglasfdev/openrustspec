// src/parser/mod.rs

pub mod spec;

use crate::error::{OpenRustSpecError, Result};
use spec::OpenApiSpec;
use std::fs;
use std::path::Path;

/// Parses an OpenAPI specification from a file.
/// It tries to parse as YAML first, then falls back to JSON.
pub fn parse<P: AsRef<Path>>(path: P) -> Result<OpenApiSpec> {
    let content = fs::read_to_string(path)?;

    // Try parsing as YAML first
    if let Ok(spec) = serde_yaml::from_str::<OpenApiSpec>(&content) {
        return Ok(spec);
    }

    // Fallback to JSON
    if let Ok(spec) = serde_json::from_str::<OpenApiSpec>(&content) {
        return Ok(spec);
    }

    Err(OpenRustSpecError::ParsingFailed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn parses_valid_yaml_spec() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("spec.yaml");
        let mut file = File::create(&file_path).unwrap();
        writeln!(
            file,
            "openapi: 3.0.0\ninfo:\n  title: My API\n  version: 1.0.0\npaths:\n  /test:\n    get:\n      summary: A test endpoint"
        )
        .unwrap();

        let spec = parse(&file_path).unwrap();
        assert_eq!(spec.openapi, "3.0.0");
        assert_eq!(spec.info.title, "My API");
        assert_eq!(spec.info.version, "1.0.0");
        assert!(spec.paths.contains_key("/test"));
    }

    #[test]
    fn parses_valid_json_spec() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("spec.json");
        let mut file = File::create(&file_path).unwrap();
        writeln!(
            file,
            r#"{{"openapi": "3.0.0", "info": {{"title": "My API", "version": "1.0.0"}}, "paths": {{}}}}"#
        )
        .unwrap();

        let spec = parse(&file_path).unwrap();
        assert_eq!(spec.openapi, "3.0.0");
        assert_eq!(spec.info.title, "My API");
        assert_eq!(spec.info.version, "1.0.0");
    }

    #[test]
    fn returns_error_for_invalid_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("spec.txt");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "this is not a valid spec").unwrap();

        let result = parse(&file_path);
        assert!(result.is_err());
        match result.unwrap_err() {
            OpenRustSpecError::ParsingFailed => (),
            _ => panic!("Incorrect error type"),
        }
    }
}