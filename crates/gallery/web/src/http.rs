use anyhow::{Context, Result};
use axum::Router;

mod photos;

pub async fn serve() -> Result<()> {
    // build application with single route
    let app = api_router();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app)
        .await
        .context("Error running HTTP server")
}

fn api_router() -> Router {
    photos::router()
}
