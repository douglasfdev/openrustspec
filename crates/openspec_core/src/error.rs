
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("LLM provider error: {0}")]
    LlmProvider(String),

    #[error("Infrastructure error: {0}")]
    Infrastructure(String),

    #[error("YAML parsing error: {0}")]
    YamlParsing(#[from] serde_yaml::Error),

    #[error("Unknown error: {0}")]
    Unknown(String),
}