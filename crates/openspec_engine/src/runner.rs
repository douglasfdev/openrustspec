
use openspec_core::{domain::model::spec::Spec, domain::ports::fs_port::FileSystemPort};
use anyhow::Result;
use std::sync::Arc;

/// The SpecRunner is responsible for executing the steps defined in a Spec.
pub struct SpecRunner {
    fs_adapter: Arc<dyn FileSystemPort + Send + Sync>,
}

impl SpecRunner {
    pub fn new(fs_adapter: Arc<dyn FileSystemPort + Send + Sync>) -> Self {
        Self { fs_adapter }
    }

    pub async fn run(&self, spec: &Spec) -> Result<()> {
        println!("--- Starting Spec Execution ---");
        println!("Spec Name: {}", spec.name);

        for (index, step) in spec.steps.iter().enumerate() {
            println!("\n[Step {}/{}] {}", index + 1, spec.steps.len(), step.name);
            println!("  Command: {}", step.command);
            println!("  Args: {:?}", step.args);

            match step.command.as_str() {
                "write_file" => {
                    if let (Some(path), Some(content)) = (step.args.get(0), step.args.get(1)) {
                        println!("  -> Executing: Writing {} bytes to '{}'", content.len(), path);
                        self.fs_adapter.write_file(path, content).await?;
                        println!("  -> Success!");
                    } else {
                        println!("  -> Error: 'write_file' command requires 2 arguments: path and content.");
                    }
                }
                "run_shell" => {
                    println!("  -> (Simulating) Running shell command...");
                }
                _ => {
                    println!("  -> Warning: Unknown command '{}'", step.command);
                }
            }
        }

        println!("\n--- Spec Execution Finished ---");
        Ok(())
    }
}