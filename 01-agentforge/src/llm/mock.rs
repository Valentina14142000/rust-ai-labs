use anyhow::Result;
use async_trait::async_trait;

use super::provider::LlmProvider;

pub struct MockLlm;

#[async_trait]
impl LlmProvider for MockLlm {
    async fn generate(&self, prompt: &str) -> Result<String> {
        tracing::debug!("Mock LLM received prompt: {}", prompt);

        Ok(format!(
            "Agent reasoning complete. I received your request: {}",
            prompt
        ))
    }
}
