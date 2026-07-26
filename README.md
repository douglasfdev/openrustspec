
# OpenSpec-rs

✨ **An AI-powered framework for Spec-Driven Development.** ✨

OpenSpec is a tool that transforms business objectives, described in natural language, into functional code, automating software development tasks.

Imagine telling your terminal: `"create a new API endpoint to register a product with a name, price, and description"` and watching the code being generated and applied to your project. That's the vision of OpenSpec.

---

## 🚀 How It Works

The OpenSpec workflow is designed to be simple and powerful, acting as a software engineering co-pilot:

1.  **Objective (Propose):** You provide a high-level objective to OpenSpec. (e.g., "Refactor function X to be more performant").

2.  **Specification Generation (Spec):** OpenSpec uses a Large Language Model (LLM) to analyze your objective and the context of your code, generating a detailed, step-by-step action plan. This plan is the "Spec".

3.  **Execution (Apply):** Upon your approval, OpenSpec executes the "Spec", using its tools to read, modify, and create files, interact with the version control system (Git), and make the necessary changes to the code.

## ⚙️ Installation and Usage

There are two ways to install and use OpenSpec, depending on your needs.

### For End-Users (Recommended)

The easiest way to use OpenSpec is to download a pre-compiled binary for your operating system from the [Releases page](https://github.com/douglasfdev/openrustspec/releases/) on GitHub. This does **not** require you to have Rust installed.

1.  Download the `.zip` file for your OS (e.g., `openrustspec-x86_64-pc-windows-msvc.zip`).
2.  Unzip the file.
3.  Place the `openrustspec.exe` executable in a directory that is included in your system's PATH.
4.  You can now run the tool from any terminal:

```bash
openrustspec propose "Your objective here"
```

*(Note: This will be configured in a future step on our roadmap.)*

### For Developers

If you are a Rust developer and want to build from source, you can install the tool directly from `crates.io` (once published) using `cargo`:

```bash
cargo install openrustspec
```

This will compile the tool and install the `openrustspec` executable in your Cargo binary path (`~/.cargo/bin`).

---

## ✅ Future Implementation Roadmap

This is the list of features that will transform OpenSpec into a complete framework:

-   [ ] **Automated Plan Execution (`apply`):** Implement the functionality that interprets the AI-generated plan and applies the modifications (creating/editing files, etc.) to the project's source code.
-   [ ] **File System Integration:** Ability to safely read, write, and modify project files.
-   [ ] **Git Integration:** Ability to create new branches before applying changes, ensuring a non-destructive workflow.
-   [ ] **Interactive Mode:** Allow the agent to ask the user questions to clarify ambiguities during execution. (e.g., "I couldn't find function X, did you mean function Y?").
-   [ ] **Advanced CLI:** Use `clap` to pass objectives and configurations directly through the command line, instead of being hardcoded.
-   [ ] **Support for Multiple AI Providers via Argument (`--provider`):** Create adapters for other APIs (e.g., OpenAI, Anthropic) and allow the user to select one via a command-line argument like `--provider openai`.
-   [ ] **Setup Automated Releases:** Configure GitHub Actions to automatically build and release binaries (`.exe`, etc.) for Windows, macOS, and Linux on every new version tag.
