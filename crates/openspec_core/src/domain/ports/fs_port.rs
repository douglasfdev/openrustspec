
use crate::error::Result;
use async_trait::async_trait;

/// Port for any file system operations.
/// This trait defines the contract for reading and writing files,
/// abstracting the underlying file system.
#[async_trait]
pub trait FileSystemPort {
    async fn read_file(&self, path: &str) -> Result<String>;
    async fn write_file(&self, path: &str, content: &str) -> Result<()>;
}