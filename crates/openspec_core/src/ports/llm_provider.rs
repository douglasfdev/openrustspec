use crate::domain::Proposal;
use crate::Result;

#[async_trait::async_trait]
pub trait LlmProvider {
    async fn generate_proposal(&self, context: &str) -> Result<Proposal>;
}