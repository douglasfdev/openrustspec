
# OpenSpec-rs

✨ **Um framework para Desenvolvimento Orientado a Especificações (Spec-Driven Development), potencializado por IA.** ✨

O OpenSpec é uma ferramenta que transforma objetivos de negócio, descritos em linguagem natural, em código funcional, automatizando tarefas de desenvolvimento de software.

Imagine poder dizer ao seu terminal: `"crie um endpoint na API para cadastrar um novo produto com nome, preço e descrição"` e ver o código sendo gerado e aplicado no seu projeto. Essa é a visão do OpenSpec.

---

## 🚀 Como Funciona?

O fluxo de trabalho do OpenSpec é desenhado para ser simples e poderoso, atuando como um co-piloto de engenharia de software:

1.  **Objetivo (Propose):** Você fornece um objetivo em alto nível para o OpenSpec. (Ex: "Refatore a função X para ser mais performática").

2.  **Geração da Especificação (Spec):** O OpenSpec utiliza um modelo de linguagem (LLM) para analisar seu objetivo e o contexto do seu código, gerando um plano de ação detalhado, passo a passo. Esse plano é a "Spec".

3.  **Execução (Apply):** Após a sua aprovação, o OpenSpec executa a "Spec", utilizando suas ferramentas para ler, modificar e criar arquivos, interagir com o sistema de controle de versão (Git) e realizar as mudanças necessárias no código.

## ⚙️ Instalação e Uso

Existem duas maneiras de instalar e usar o OpenSpec, dependendo de suas necessidades.

### Para Usuários Finais (Recomendado)

A maneira mais fácil de usar o OpenSpec é baixar um binário pré-compilado para o seu sistema operacional a partir da [página de Releases](https://github.com/douglasfdev/openrustspec/releases/) no GitHub. Este método **não** exige que você tenha o Rust instalado.

1.  Baixe o arquivo `.zip` para o seu SO (ex: `openrustspec-x86_64-pc-windows-msvc.zip`).
2.  Descompacte o arquivo.
3.  Coloque o executável `openrustspec.exe` em um diretório que esteja incluído no PATH do seu sistema.
4.  Agora você pode executar a ferramenta de qualquer terminal:

```bash
openrustspec propose "Seu objetivo aqui"
```

*(Observação: Esta funcionalidade será configurada em um passo futuro do nosso roadmap.)*

### Para Desenvolvedores

Se você é um desenvolvedor Rust e deseja compilar a partir do código-fonte, pode instalar a ferramenta diretamente do `crates.io` (após a publicação) usando o `cargo`:

```bash
cargo install openrustspec
```

Isso irá compilar a ferramenta e instalar o executável `openrustspec` no diretório de binários do Cargo (`~/.cargo/bin`).

---

## ✅ Roadmap de Implementações Futuras

Esta é a lista de funcionalidades que transformarão o OpenSpec em um framework completo:

-   [ ] **Execução Automatizada do Plano (`apply`):** Implementar a funcionalidade que interpreta o plano gerado pela IA e aplica as modificações (criar/editar arquivos, etc.) no código-fonte do projeto.
-   [ ] **Integração com Sistema de Arquivos:** Habilidade de ler, escrever e modificar arquivos do projeto de forma segura.
-   [ ] **Integração com Git:** Capacidade de criar novas branches antes de aplicar mudanças, garantindo um fluxo de trabalho não-destrutivo.
-   [ ] **Modo Interativo:** Permitir que o agente faça perguntas ao usuário para esclarecer ambiguidades durante a execução. (Ex: "Não encontrei a função X, você quis dizer a função Y?").
-   [ ] **CLI Avançada:** Utilizar `clap` para passar objetivos e configurações diretamente pela linha de comando, em vez de estarem fixos no código.
-   [ ] **Suporte a Múltiplos Provedores de IA via Argumento (`--provider`):** Criar adaptadores para outras APIs (ex: OpenAI, Anthropic) e permitir que o usuário escolha um através de um argumento de linha de comando como `--provider openai`.
-   [ ] **Configurar Releases Automatizadas:** Configurar o GitHub Actions para compilar e lançar automaticamente binários (`.exe`, etc.) para Windows, macOS e Linux a cada nova tag de versão.
