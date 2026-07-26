use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proposal {
    pub id: Uuid,
    pub title: String,
    pub summary: String,
    pub non_goals: Vec<String>,
    pub status: ProposalStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProposalStatus {
    Draft,
    Proposed,
    Approved,
    Applied,
}

impl Default for Proposal {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            title: "Untitled Proposal".to_string(),
            summary: "".to_string(),
            non_goals: vec![],
            status: ProposalStatus::Draft,
        }
    }
}