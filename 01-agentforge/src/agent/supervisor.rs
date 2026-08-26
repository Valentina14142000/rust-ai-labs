use std::collections::HashMap;

use super::agent::Agent;

pub struct AgentSupervisor {
    agents: HashMap<String, Agent>,
}

impl AgentSupervisor {
    pub fn new() -> Self {
        Self {
            agents: HashMap::new(),
        }
    }

    pub fn register(&mut self, agent: Agent) {
        self.agents.insert(agent.name.clone(), agent);
    }

    pub fn get(&self, name: &str) -> Option<&Agent> {
        self.agents.get(name)
    }

    pub fn list(&self) -> Vec<String> {
        self.agents.keys().cloned().collect()
    }
}

impl Default for AgentSupervisor {
    fn default() -> Self {
        Self::new()
    }
}
