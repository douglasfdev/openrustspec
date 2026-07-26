
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a single, executable step in a specification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecStep {
    pub name: String,
    pub command: String,
    pub args: Vec<String>,
}

/// The root aggregate representing a full specification for a task.
/// It contains a sequence of steps to be executed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spec {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub steps: Vec<SpecStep>,
}

impl Spec {
    pub fn new(name: String, description: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            description,
            steps: Vec::new(),
        }
    }

    pub fn add_step(&mut self, name: String, command: String, args: Vec<String>) {
        self.steps.push(SpecStep {
            name,
            command,
            args,
        });
    }
}