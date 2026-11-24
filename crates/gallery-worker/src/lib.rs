use worker::{Context, Env, Request, Response, Result, Router, event};

use crate::infrastructure::photo_bucket::PhotoBucket;
use crate::services::photo_service::PhotoService;

mod infrastructure;
mod routes;
mod services;

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let bucket_service = PhotoBucket::new(env.bucket("anders-gallery-photos")?);

    let service = PhotoService::new(env.d1("anders_gallery_db")?, bucket_service);

    Router::with_data(service)
        .post_async("/api/photos", routes::photos::upload)
        .get_async("/api/health", routes::health::health)
        .run(req, env)
        .await
}
