use axum::{http::StatusCode, routing::get, Router, ServiceExt};
use tracing::info;
use tracing_subscriber::FmtSubscriber;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[derive(Debug)]
enum ActionMessage {
    Update,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;

    let app = Router::new().route("/", get(ping));

    info!("MUD has started");

    let listener = tokio::net::TcpListener::bind("localhost:7700").await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn ping() -> String {
    format!("OK mud v{VERSION}")
}
