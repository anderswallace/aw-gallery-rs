use anyhow::Result;
use std::sync::Arc;

use axum::{Router, extract::State, http::StatusCode, routing::get};
use gallery_core::classes::services::has_db_service::HasDbService;

use crate::state::GalleryWebState;

pub fn router() -> Router<Arc<GalleryWebState>> {
    Router::new().route("/api/photos", get(method))
}

// Simple method to test using Gallery core services
async fn method(State(state): State<Arc<GalleryWebState>>) -> Result<String, (StatusCode, String)> {
    let services = state.services().db_service();
    let message = services.db_method().await.map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("DB error: {err}"),
        )
    })?;
    Ok(message)
}
