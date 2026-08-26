use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};

use crate::{
    agent::{Agent, executor::AgentExecutor},
    llm::{LlmClient, MockLlm},
};

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub service: &'static str,
}

pub async fn health() -> impl IntoResponse {
    Json(HealthResponse {
        status: "ok",
        service: "agentforge",
    })
}

pub async fn list_agents() -> impl IntoResponse {
    Json(vec!["research-agent", "analysis-agent", "coding-agent"])
}

#[derive(Debug, Deserialize)]
pub struct RunAgentRequest {
    pub prompt: String,
}

#[derive(Debug, Serialize)]
pub struct RunAgentResponse {
    pub status: String,
    pub result: String,
}

pub async fn run_agent(
    Json(request): Json<RunAgentRequest>,
) -> (StatusCode, Json<RunAgentResponse>) {
    let llm = LlmClient::new(Box::new(MockLlm));

    let executor = AgentExecutor::new(llm, 5);

    let mut agent = Agent::new(
        "research-agent",
        "An autonomous research and analysis agent.",
    );

    let result = executor.execute(&mut agent, request.prompt).await;

    match result {
        Ok(response) => (
            StatusCode::OK,
            Json(RunAgentResponse {
                status: "completed".to_string(),
                result: response,
            }),
        ),

        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(RunAgentResponse {
                status: "failed".to_string(),
                result: error.to_string(),
            }),
        ),
    }
}
