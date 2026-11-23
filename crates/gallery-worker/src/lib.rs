use worker::{Context, Env, Request, Response, Result, Router, event};

mod routes;

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    Router::new()
        .post_async("/api/photos", routes::photos::upload)
        .get_async("/api/health", routes::health::health)
        .run(req, env)
        .await
}
