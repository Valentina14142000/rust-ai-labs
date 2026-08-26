use anyhow::Result;
use serde_json::json;

use crate::{
    llm::LlmClient,
    tools::{ToolRegistry, builtin::default_tools},
};

use super::agent::Agent;

pub struct AgentExecutor {
    max_iterations: usize,
    llm: LlmClient,
    tools: ToolRegistry,
}

impl AgentExecutor {
    pub fn new(llm: LlmClient, max_iterations: usize) -> Self {
        Self {
            max_iterations,
            llm,
            tools: default_tools(),
        }
    }

    pub async fn execute(&self, agent: &mut Agent, task: String) -> Result<String> {
        agent.start(task.clone());

        tracing::info!(
            agent = %agent.name,
            task = %task,
            "Starting agent execution"
        );

        for _ in 0..self.max_iterations {
            agent.increment_iteration();

            tracing::debug!(
                agent = %agent.name,
                iteration = agent.state.iteration,
                "Agent iteration"
            );

            if task.to_lowercase().contains("calculate") {
                let input = json!({
                    "operation": "multiply",
                    "a": 12,
                    "b": 8
                });

                let tool = self
                    .tools
                    .get("calculator")
                    .ok_or_else(|| anyhow::anyhow!("Calculator tool not found"))?;

                let result = tool.execute(input).await?;

                agent.complete();

                return Ok(format!("I used the calculator tool. Result: {}", result));
            }

            let prompt = format!(
                "You are the {} agent.

Description:
{}

User task:
{}

Iteration:
{}",
                agent.name, agent.description, agent.state.task, agent.state.iteration
            );

            let response = self.llm.generate(&prompt).await?;

            agent.complete();

            return Ok(response);
        }

        agent.fail();

        anyhow::bail!("Agent exceeded maximum iterations")
    }
}
