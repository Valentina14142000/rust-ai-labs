use super::state::{AgentState, AgentStatus};

#[derive(Debug, Clone)]
pub struct Agent {
    pub name: String,
    pub description: String,
    pub state: AgentState,
}

impl Agent {
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            state: AgentState::new(String::new()),
        }
    }

    pub fn start(&mut self, task: String) {
        self.state.task = task;
        self.state.status = AgentStatus::Running;
        self.state.iteration = 0;
    }

    pub fn complete(&mut self) {
        self.state.status = AgentStatus::Completed;
    }

    pub fn fail(&mut self) {
        self.state.status = AgentStatus::Failed;
    }

    pub fn increment_iteration(&mut self) {
        self.state.iteration += 1;
    }
}
