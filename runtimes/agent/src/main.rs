use std::sync::Arc;
use openspec_core::{
    application::create_proposal::{CreateProposalCommand, CreateProposalUseCase},
    domain::{Proposal, ProposalStatus},
    ports::llm_provider::LlmProvider,
    Result,
};
use openspec_cli::{Cli, Commands, parse};

// 1. MockLlmProvider para simulação
struct MockLlmProvider;

#[async_trait::async_trait]
impl LlmProvider for MockLlmProvider {
    async fn generate_proposal(&self, context: &str) -> Result<Proposal> {
        println!("\n[MockLlmProvider] Recebido prompt: \"{}\"", context);
        let mut proposal = Proposal::default();
        proposal.title = "API de Usuários".to_string();
        proposal.summary = format!("Uma proposta gerada a partir do prompt: '{}'", context);
        proposal.non_goals = vec!["Não incluirá autenticação OAuth2".to_string()];
        proposal.status = ProposalStatus::Proposed;
        println!("[MockLlmProvider] Gerando proposta simulada...");
        Ok(proposal)
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("OpenRustSpec Agent starting...");

    // 2. Injeção de Dependência
    let llm_provider = Arc::new(MockLlmProvider);
    let create_proposal_use_case = CreateProposalUseCase::new(llm_provider.clone());

    // 3. Parse da CLI
    let cli = parse();

    // 4. Execução do Comando
    match cli.command {
        Commands::Propose { prompt } => {
            println!("\nComando '/rustsx:propose' detectado.");
            let command = CreateProposalCommand { prompt };
            match create_proposal_use_case.execute(command).await {
                Ok(proposal) => {
                    println!("\n--- Proposta Gerada com Sucesso ---");
                    println!("ID: {}", proposal.id);
                    println!("Título: {}", proposal.title);
                    println!("Resumo: {}", proposal.summary);
                    println!("Status: {:?}", proposal.status);
                    println!("Não Escopo: {:?}", proposal.non_goals);
                    println!("-------------------------------------");
                    println!("\nPróximo passo: Execute '/rustsx:apply' para aplicar esta proposta (ainda não implementado).");
                }
                Err(e) => {
                    eprintln!("Erro ao gerar proposta: {}", e);
                }
            }
        }
        Commands::Apply => {
            println!("\nComando '/rustsx:apply' detectado.");
            println!("Funcionalidade ainda não implementada.");
        }
    }

    Ok(())
}