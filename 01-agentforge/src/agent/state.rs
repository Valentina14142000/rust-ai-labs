use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentState {
    pub id: Uuid,
    pub task: String,
    pub status: AgentStatus,
    pub iteration: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AgentStatus {
    Idle,
    Running,
    Completed,
    Failed,
}

impl AgentState {
    pub fn new(task: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            task,
            status: AgentStatus::Idle,
            iteration: 0,
        }
    }
}
