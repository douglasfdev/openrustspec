# OpenRustSpec - An AI Agent for Spec-Driven Development

**OpenRustSpec** is a framework and AI agent for **Spec-Driven Development**, built in Rust with a hexagonal architecture. It transforms natural language descriptions into proposals, plans, and eventually code, guiding the development process interactively.

## Documentation

For more detailed documentation, please visit:

*   [Documentation in Brazilian Portuguese](./docs/pt-br/README.md)

## The New Architecture

`openrustspec` has been rewritten from the ground up to follow the principles of **Hexagonal Architecture (Ports & Adapters)**. This ensures a decoupled, testable, and extensible system ready for production.

The project structure is now a Cargo workspace with clearly divided responsibilities:

```text
openrustspec-rs/
├── crates/
│   ├── openspec_core/ # The Core: Domain + Application + Ports
│   ├── openspec_cli/  # Primary Adapter: The CLI with agent commands
│   └── ...            # Other adapters (LLM, FileSystem, etc.)
└── runtimes/
    └── agent/         # The main executable that ties everything together
```

## How to Use the Agent

`openrustspec` operates as a command-line agent. To install it and make it globally available, use Cargo:

```bash
cargo install --path .
```

### Main Commands

Interaction with the agent is done through specific commands that function as "skills".

#### 1. Propose a Change (`/rustsx:propose`)

Use this command to ask the AI to create a specification proposal from a natural language idea.

**Example:**

```bash
agent /rustsx:propose "I want to create a REST API for a blog system with posts and comments"
```

**What Happens:**

1.  The **CLI Adapter** (`openspec_cli`) parses the command.
2.  It invokes the `CreateProposal` **Use Case** in the `Application` layer.
3.  The use case calls the `LlmProvider` **Port**.
4.  The **LLM Adapter** (e.g., `OpenAiAdapter`) is activated, sends the prompt to the AI, and translates the response into a `Proposal` domain entity.
5.  The proposal is displayed to the user for approval.

#### 2. Apply a Proposal (`/rustsx:apply`)

Once a proposal has been generated and approved, this command instructs the agent to create a detailed execution plan and then apply it.

**Example:**

```bash
agent /rustsx:apply
```

**What Happens (Roadmap):**

1.  The agent identifies the last approved proposal.
2.  It uses the AI to generate a `Plan`, which is a list of concrete tasks (e.g., `CreateFile`, `ModifyFile`).
3.  It asks for user confirmation to execute the plan.
4.  It executes each task in the plan using the appropriate **Adapters** (e.g., `FileSystemAdapter` to create a file, `GitAdapter` to commit the change).

## Development Roadmap

-   [x] **Hexagonal Architecture Foundation**: Workspace and crates defined.
-   [x] **Proposal Flow (`/rustsx:propose`)**: Use case implemented with a mocked LLM.
-   [ ] **Application Flow (`/rustsx:apply`)**: Implement the use case to generate and execute plans.
-   [ ] **Real LLM Integration**: Replace the mock with a real adapter for OpenAI, reading the API key from a `config.yml` file.
-   [ ] **Persistence**: Implement a `Repository` to save and load the state of specifications and proposals.
-   [ ] **Code Generation**: Create `CodeGeneratorAdapters` that transform the specification into boilerplate code in various languages.