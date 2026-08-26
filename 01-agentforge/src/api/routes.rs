use axum::{
    Router,
    routing::{get, post},
};

use super::handlers;

pub fn create_router() -> Router {
    Router::new()
        .route("/health", get(handlers::health))
        .route("/agents", get(handlers::list_agents))
        .route("/agents/run", post(handlers::run_agent))
}
