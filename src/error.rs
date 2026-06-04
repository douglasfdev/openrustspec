// src/error.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum OpenRustSpecError {
    #[error("File I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("YAML parsing error: {0}")]
    Yaml(#[from] serde_yaml::Error),

    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Parsing failed: could not parse as YAML or JSON")]
    ParsingFailed,

    #[error("Generation error: {0}")]
    Generation(String),
}

pub type Result<T> = std::result::Result<T, OpenRustSpecError>;