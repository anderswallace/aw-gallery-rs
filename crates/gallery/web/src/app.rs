use axum::Router;

use crate::http;

// Build routes with Gallery services state
pub fn build_router() -> Router {
    http::build()
}
