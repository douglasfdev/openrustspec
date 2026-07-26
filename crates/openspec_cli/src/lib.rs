use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// (New) Propose a change or new feature based on a prompt.
    #[command(name = "/rustsx:propose")]
    Propose {
        /// The natural language prompt describing the change.
        prompt: String,
    },
    /// (New) Apply the last approved proposal.
    #[command(name = "/rustsx:apply")]
    Apply,
}

pub fn parse() -> Cli {
    Cli::parse()
}