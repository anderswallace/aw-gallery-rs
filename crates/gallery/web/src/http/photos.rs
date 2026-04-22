use axum::{Router, routing::get};

pub fn router() -> Router {
    Router::new().route("/api/photos", get(greeting))
}

async fn greeting() -> &'static str {
    "Hello World!"
}
