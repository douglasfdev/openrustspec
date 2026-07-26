# Documentação Detalhada - OpenRustSpec

Bem-vindo à documentação detalhada do OpenRustSpec em Português do Brasil.

## Visão Geral da Arquitetura

Nesta seção, vamos explorar em profundidade cada camada da nossa arquitetura hexagonal:

*   **Domain**: O coração da aplicação, contendo as regras de negócio puras.
*   **Application**: Os casos de uso que orquestram os fluxos de trabalho.
*   **Ports**: As interfaces (traits) que definem os contratos com o mundo exterior.
*   **Adapters**: As implementações concretas dos ports.

*(Esta seção será expandida com mais detalhes sobre cada componente.)*

## Guia de Contribuição

Interessado em contribuir? Ótimo! Aqui estão os passos para começar:

1.  Faça um fork do repositório.
2.  Clone o seu fork localmente: `git clone ...`
3.  Configure o ambiente de desenvolvimento Rust.
4.  Compile o projeto com `cargo build --workspace`.
5.  Comece a desenvolver!

*(Esta seção será expandida com guias sobre como adicionar novos adapters, casos de uso e mais.)*