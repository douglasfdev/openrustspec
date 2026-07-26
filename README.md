# OpenRustSpec - Agente de IA para Spec-Driven Development

**OpenRustSpec** é um framework e agente de IA para **Desenvolvimento Orientado a Especificações** (*Spec-Driven Development*), construído em Rust com uma arquitetura hexagonal. Ele transforma descrições em linguagem natural em propostas, planos e, eventualmente, em código, guiando o processo de desenvolvimento de forma interativa.

## Documentação

Para uma documentação mais detalhada, visite:

*   [Documentação em Português (Brasil)](./docs/pt-br/README.md)

## A Nova Arquitetura

O `openrustspec` foi reescrito do zero para seguir os princípios da **Arquitetura Hexagonal (Ports & Adapters)**. Isso garante um sistema desacoplado, testável e extensível, pronto para produção.

A estrutura do projeto agora é um workspace Cargo com responsabilidades claramente divididas:

```text
openrustspec-rs/
├── crates/
│   ├── openspec_core/ # O coração: Domain + Application + Ports
│   ├── openspec_cli/  # Primary Adapter: A CLI com os comandos do agente
│   └── ...            # Outros adapters (LLM, FileSystem, etc.)
└── runtimes/
    └── agent/         # O executável principal que une tudo
```

## Como Usar o Agente

O `openrustspec` opera como um agente de linha de comando. Para instalá-lo e torná-lo disponível globalmente, use o Cargo:

```bash
cargo install --path .
```

### Comandos Principais

A interação com o agente é feita através de comandos específicos que funcionam como "skills".

#### 1. Propor uma Mudança (`/rustsx:propose`)

Use este comando para pedir à IA que crie uma proposta de especificação a partir de uma ideia em linguagem natural.

**Exemplo:**

```bash
agent /rustsx:propose "Quero criar uma API REST para um sistema de blog com posts e comentários"
```

**O que acontece:**

1.  O **Adapter de CLI** (`openspec_cli`) parseia o comando.
2.  Ele invoca o **Caso de Uso** `CreateProposal` na camada de `Application`.
3.  O caso de uso chama o **Port** `LlmProvider`.
4.  O **Adapter de LLM** (ex: `OpenAiAdapter`) é ativado, envia o prompt para a IA e traduz a resposta para uma entidade de domínio `Proposal`.
5.  A proposta é exibida para o usuário para aprovação.

#### 2. Aplicar uma Proposta (`/rustsx:apply`)

Uma vez que uma proposta foi gerada e aprovada, este comando instrui o agente a criar um plano de execução detalhado e, em seguida, aplicar esse plano.

**Exemplo:**

```bash
agent /rustsx:apply
```

**O que acontece (Roadmap):**

1.  O agente identifica a última proposta aprovada.
2.  Usa a IA para gerar um `Plan`, que é uma lista de tarefas concretas (ex: `CreateFile`, `ModifyFile`).
3.  Pede a confirmação do usuário para executar o plano.
4.  Executa cada tarefa do plano usando os **Adapters** apropriados (ex: `FileSystemAdapter` para criar um arquivo, `GitAdapter` para commitar a mudança).

## Roadmap de Desenvolvimento

-   [x] **Fundação da Arquitetura Hexagonal**: Workspace e crates definidos.
-   [x] **Fluxo de Proposta (`/rustsx:propose`)**: Implementação do caso de uso com um LLM mockado.
-   [ ] **Fluxo de Aplicação (`/rustsx:apply`)**: Implementação do caso de uso para gerar e executar planos.
-   [ ] **Integração Real com LLM**: Substituir o mock por um adapter real para OpenAI, lendo a chave de um `config.yml`.
-   [ ] **Persistência**: Implementar um `Repository` para salvar e carregar o estado das especificações e propostas.
-   [ ] **Geração de Código**: Criar `CodeGeneratorAdapters` que transformam a especificação em código boilerplate em várias linguagens.