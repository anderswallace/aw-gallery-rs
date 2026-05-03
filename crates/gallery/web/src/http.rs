use anyhow::{Context, Result};
use axum::Router;
use std::sync::Arc;

use crate::{
    app::{build_router, init_state_from_disk},
    state::GalleryWebState,
};

mod photos;

pub async fn serve() -> Result<()> {
    let state = init_state_from_disk().await?;
    let router = build_router(state);

    // build application with single route

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router)
        .await
        .context("Error running HTTP server")
}

// aggregate all http routes
pub fn build() -> Router<Arc<GalleryWebState>> {
    photos::router()
}
