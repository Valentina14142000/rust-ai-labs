mod agent;
mod api;
mod llm;
mod memory;
mod tools;

use anyhow::Result;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting AgentForge...");

    let app = api::routes::create_router();

    let address = "0.0.0.0:3000";

    let listener = tokio::net::TcpListener::bind(address).await?;

    tracing::info!("AgentForge running at http://{}", address);

    axum::serve(listener, app).await?;

    Ok(())
}
