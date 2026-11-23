use worker::{Context, Env, Request, Response, Result, Router, event};

mod photos;

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    Router::new()
        .post_async("/api/photos", photos::upload)
        .run(req, env)
        .await
}
