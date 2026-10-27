use axum::{Router, routing::get};

pub fn router() -> Router {
    Router::new().route("/api/login", get(method))
}

async fn method() -> &'static str {
    "Hello World!"
}
