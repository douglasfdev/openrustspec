use std::sync::Arc;
use openspec_core::{
    application::usecase::generate_proposal::{
        GenerateProposalCommand, GenerateProposalUseCase,
    },
    domain::model::spec::Spec,
    domain::ports::llm_provider::{
        self, GenerateSpecRequest, GenerateSpecResponse, LlmProvider,
    },
    error::{Error, Result},
};
use openspec_cli::{Cli, Commands, parse};

// 1. MockLlmProvider para simulação
struct MockLlmProvider;

#[async_trait::async_trait]
impl LlmProvider for MockLlmProvider {
    async fn generate_spec(
        &self,
        request: GenerateSpecRequest,
    ) -> Result<GenerateSpecResponse> {
        println!("\n[MockLlmProvider] Recebido objetivo: \"{}\"", request.objective);
        let mock_yaml_spec = format!(
            r#"
name: "API de Usuários"
description: "Uma proposta gerada a partir do objetivo: {}"
tasks:
  - id: 1
    description: "Criar a estrutura do módulo de usuário"
    completed: false
"#,
            request.objective
        );
        println!("[MockLlmProvider] Gerando especificação YAML simulada...");
        Ok(GenerateSpecResponse {
            spec_yaml: mock_yaml_spec,
        })
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("OpenRustSpec Agent starting...");

    // 2. Injeção de Dependência
    let llm_provider = Arc::new(MockLlmProvider);
    let generate_proposal_use_case = GenerateProposalUseCase::new(llm_provider.clone());

    // 3. Parse da CLI
    let cli = parse();

    // 4. Execução do Comando
    match cli.command {
        Commands::Propose { prompt } => {
            println!("\nComando '/rustsx:propose' detectado.");
            let command = GenerateProposalCommand {
                objective: prompt,
                context: "(No context provided)".to_string(), // Contexto virá da análise do projeto no futuro
            };
            match generate_proposal_use_case.execute(command).await {
                Ok(spec) => {
                    println!("\n--- Especificação Gerada com Sucesso ---");
                    println!("Nome: {}", spec.name);
                    println!("Descrição: {}", spec.description);
                    println!("Passos:");
                    for step in spec.steps {
                        println!("  - {} ({})", step.name, step.command);
                    }
                    println!("-------------------------------------");
                    println!("\nPróximo passo: Execute '/rustsx:apply' para aplicar esta especificação (ainda não implementado).");
                }
                Err(e) => {
                    eprintln!("Erro ao gerar especificação: {}", e);
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