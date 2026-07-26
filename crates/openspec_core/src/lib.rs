pub mod domain;
pub mod application;
pub mod ports;

pub mod error;

pub type Result<T> = std::result::Result<T, error::Error>;