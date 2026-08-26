use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait LlmProvider: Send + Sync {
    async fn generate(&self, prompt: &str) -> Result<String>;
}
