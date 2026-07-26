use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("I/O Error: {0}")]
    Io(#[from] std::io::Error),

    #[error("LLM Provider Error: {0}")]
    LlmProvider(String),

    #[error("Unknown error")]
    Unknown,
}