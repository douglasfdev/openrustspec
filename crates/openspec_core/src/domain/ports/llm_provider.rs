
use crate::error::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateSpecRequest {
    pub objective: String,
    pub context: String, // e.g., project structure, relevant code snippets
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateSpecResponse {
    pub spec_yaml: String, // The LLM should generate the Spec in a structured format like YAML
}

/// Port for any Large Language Model provider.
/// This trait defines the contract for generating content, specifically a `Spec`
/// from a high-level objective.
#[async_trait]
pub trait LlmProvider {
    async fn generate_spec(
        &self,
        request: GenerateSpecRequest,
    ) -> Result<GenerateSpecResponse>;
}