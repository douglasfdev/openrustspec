
use crate::domain::{
    model::spec::Spec,
    ports::llm_provider::{GenerateSpecRequest, LlmProvider},
};
use crate::error::Result;
use std::sync::Arc;

/// Use Case for generating a new proposal (Spec) from a high-level objective.
pub struct GenerateProposalUseCase {
    llm_provider: Arc<dyn LlmProvider + Send + Sync>,
}

impl GenerateProposalUseCase {
    pub fn new(llm_provider: Arc<dyn LlmProvider + Send + Sync>) -> Self {
        Self { llm_provider }
    }

    pub async fn execute(&self, objective: String, context: String) -> Result<Spec> {
        // 1. Create the request for the LLM provider
        let request = GenerateSpecRequest {
            objective,
            context,
        };

        // 2. Call the LLM provider through the port
        let response = self.llm_provider.generate_spec(request).await?;

        // 3. Parse the YAML response into a domain aggregate
        //    (Error handling is crucial here)
        let spec: Spec = serde_yaml::from_str(&response.spec_yaml)?;

        // 4. Return the structured Spec
        Ok(spec)
    }
}