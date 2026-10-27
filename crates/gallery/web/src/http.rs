use anyhow::{Context, Result};
use axum::Router;

mod login;

use crate::app::build_router;

pub async fn serve() -> Result<()> {
    let router = build_router();

    // build application with single route

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router)
        .await
        .context("Error running HTTP server")
}

// aggregate all http routes
pub fn build() -> Router {
    login::router()
}
