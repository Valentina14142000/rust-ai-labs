use anyhow::Result;

use super::provider::LlmProvider;

pub struct LlmClient {
    provider: Box<dyn LlmProvider>,
}

impl LlmClient {
    pub fn new(provider: Box<dyn LlmProvider>) -> Self {
        Self { provider }
    }

    pub async fn generate(&self, prompt: &str) -> Result<String> {
        self.provider.generate(prompt).await
    }
}
