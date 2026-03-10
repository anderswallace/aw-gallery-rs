use axum::Router;
use axum::routing::get;

#[tokio::main]
async fn main() {
    // build application with single route
    let app = Router::new().route("/", get(greeting));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn greeting() -> &'static str {
    "Hello World!"
}
