use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;

#[async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;

    async fn execute(&self, input: Value) -> Result<String>;
}

pub struct CalculatorTool;

#[async_trait]
impl Tool for CalculatorTool {
    fn name(&self) -> &str {
        "calculator"
    }

    fn description(&self) -> &str {
        "Performs basic arithmetic calculations."
    }

    async fn execute(&self, input: Value) -> Result<String> {
        let expression = input
            .get("expression")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .trim();

        let result = if let Some((left, right)) = expression.split_once('+') {
            left.trim().parse::<f64>()? + right.trim().parse::<f64>()?
        } else if let Some((left, right)) = expression.split_once('-') {
            left.trim().parse::<f64>()? - right.trim().parse::<f64>()?
        } else if let Some((left, right)) = expression.split_once('*') {
            left.trim().parse::<f64>()? * right.trim().parse::<f64>()?
        } else if let Some((left, right)) = expression.split_once('/') {
            let divisor = right.trim().parse::<f64>()?;

            if divisor == 0.0 {
                anyhow::bail!("division by zero");
            }

            left.trim().parse::<f64>()? / divisor
        } else {
            anyhow::bail!("unsupported expression: {expression}");
        };

        Ok(result.to_string())
    }
}
