use anyhow::Result;
use axum::{routing::get, Router};
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    let listener = TcpListener::bind("0.0.0.0:3000").await?;

    info!("Starting server at port 3000");

    axum::serve(listener, app).await?;

    Ok(())
}
