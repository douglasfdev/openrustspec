// src/main.rs

use clap::{Parser, Subcommand, ValueEnum};
use openrustspec::{
    core,
    generator::{Generator, MarkdownGenerator, PlanGenerator},
    parser,
};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Creates a new project and its initial specification from a prompt.
    New {
        /// The name of the new project.
        project_name: String,

        /// The natural language prompt to generate the specification from.
        #[arg(long)]
        from_prompt: String,
    },
    /// Generates artifacts (e.g., plan, docs) for an existing project.
    Generate {
        /// The name of the project.
        project_name: String,

        /// Path to the output file. If not provided, a default is used (e.g., plan.md).
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Output format.
        #[arg(short, long, value_enum, default_value_t = Format::Plan)]
        format: Format,
    },
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Format {
    Doc,
    Plan,
    Proposal,
}

impl Format {
    fn default_filename(&self) -> &'static str {
        match self {
            Format::Doc => "docs.md",
            Format::Plan => "plan.md",
            Format::Proposal => "proposal.md",
        }
    }
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::New { project_name, from_prompt } => {
            handle_new_project(project_name, from_prompt)?;
        }
        Commands::Generate { project_name, output, format } => {
            handle_generate(project_name, output, *format)?;
        }
    }

    Ok(())
}

/// Handles the `new` command to bootstrap a project.
fn handle_new_project(project_name: &str, _prompt: &str) -> anyhow::Result<()> {
    let project_dir = Path::new("rustspec").join(project_name);
    fs::create_dir_all(&project_dir)?;

    let spec_path = project_dir.join("spec.yaml");

    if spec_path.exists() {
        println!("Project '{}' already exists.", project_name);
        return Ok(());
    }

    // --- LLM Simulation ---
    // In a real scenario, the `_prompt` would be sent to an LLM API.
    // Here, we simulate the LLM's response with a hardcoded spec.
    let simulated_llm_output = r#"
openapi: 3.0.3
info:
  title: Simple CRUD API
  description: "Generated from prompt: create a simple crud api"
  version: 1.0.0
paths:
  /items:
    get:
      summary: List all items
    post:
      summary: Create a new item
  /items/{id}:
    get:
      summary: Get an item by its ID
    delete:
      summary: Delete an item by its ID
"#;
    // --- End Simulation ---

    fs::write(&spec_path, simulated_llm_output.trim())?;
    println!(
        "Successfully created new project '{}' with spec file at: {}",
        project_name,
        spec_path.display()
    );

    // Bonus: Automatically generate the initial plan
    println!("\nGenerating initial project plan...");
    handle_generate(project_name, &None, Format::Plan)?;

    Ok(())
}

/// Handles the `generate` command to create artifacts from a spec.
fn handle_generate(
    project_name: &str,
    output: &Option<PathBuf>,
    format: Format,
) -> anyhow::Result<()> {
    let project_dir = Path::new("rustspec").join(project_name);
    let spec_path = project_dir.join("spec.yaml");

    if !spec_path.exists() {
        println!(
            "Specification file not found for project '{}' at '{}'.",
            project_name,
            spec_path.display()
        );
        println!("Please create the project first using the 'new' command.");
        return Ok(());
    }

    let spec = parser::parse(&spec_path)?;
    let api = core::process(spec)?;

    let generator: Box<dyn Generator> = match format {
        Format::Doc | Format::Proposal => Box::new(MarkdownGenerator),
        Format::Plan => Box::new(PlanGenerator),
    };
    let output_content = generator.generate(&api)?;

    let output_path =
        project_dir.join(output.clone().unwrap_or_else(|| PathBuf::from(format.default_filename())));

    fs::write(&output_path, output_content)?;
    println!("Successfully generated output to: {}", output_path.display());

    Ok(())
}