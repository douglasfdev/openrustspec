
use std::sync::Arc;
use openspec_adapters::llm_adapter::ollama::OllamaAdapter;
use openspec_core::application::usecase::generate_proposal::GenerateProposalUseCase;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // --- Composition Root ---
    // This is where we "wire up" our application.

    // 1. Configuration (hardcoded for now)
    let ollama_endpoint = "http://localhost:11434/api/generate".to_string();
    let ollama_model = "llama3".to_string();

    // 2. Instantiate Adapters
    let llm_adapter = Arc::new(OllamaAdapter::new(ollama_model, ollama_endpoint)?);

    // 3. Instantiate Use Cases and inject dependencies
    let generate_proposal_use_case = GenerateProposalUseCase::new(llm_adapter);

    // --- Execution ---

    // 4. Define a high-level objective
    let objective = "Create a new Rust function that takes two integers and returns their sum.".to_string();
    let context = "The project is a simple Rust library. No external dependencies needed.".to_string();

    println!("Generating spec for objective: '{}'...", objective);

    // 5. Execute the use case
    let spec = generate_proposal_use_case.execute(objective, context).await?;

    // 6. Print the result
    println!("\n--- Generated Spec ---");
    println!("{}", serde_yaml::to_string(&spec)?);

    Ok(())
}