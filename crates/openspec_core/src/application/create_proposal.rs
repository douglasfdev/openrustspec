use crate::domain::Proposal;
use crate::ports::llm_provider::LlmProvider;
use crate::Result;
use std::sync::Arc;

pub struct CreateProposalCommand {
    pub prompt: String,
}

pub struct CreateProposalUseCase {
    llm_provider: Arc<dyn LlmProvider>,
}

impl CreateProposalUseCase {
    pub fn new(llm_provider: Arc<dyn LlmProvider>) -> Self {
        Self { llm_provider }
    }

    pub async fn execute(&self, command: CreateProposalCommand) -> Result<Proposal> {
        let proposal = self.llm_provider.generate_proposal(&command.prompt).await?;
        // Aqui, no futuro, salvaremos a proposta usando um `SpecRepository`
        Ok(proposal)
    }
}