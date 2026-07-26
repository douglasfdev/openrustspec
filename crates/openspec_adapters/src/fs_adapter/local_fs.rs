
use async_trait::async_trait;
use openspec_core::{
    domain::ports::fs_port::FileSystemPort,
    error::{Error, Result},
};
use tokio::fs;

/// Concrete adapter for the local file system.
pub struct LocalFileSystemAdapter;

impl LocalFileSystemAdapter {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl FileSystemPort for LocalFileSystemAdapter {
    async fn read_file(&self, path: &str) -> Result<String> {
        fs::read_to_string(path)
            .await
            .map_err(|e| Error::Infrastructure(format!("Failed to read file {}: {}", path, e)))
    }

    async fn write_file(&self, path: &str, content: &str) -> Result<()> {
        // Ensure the directory exists before writing the file
        if let Some(parent) = std::path::Path::new(path).parent() {
            fs::create_dir_all(parent)
                .await
                .map_err(|e| Error::Infrastructure(format!("Failed to create directory for {}: {}", path, e)))?;
        }

        fs::write(path, content)
            .await
            .map_err(|e| Error::Infrastructure(format!("Failed to write file {}: {}", path, e)))
    }
}