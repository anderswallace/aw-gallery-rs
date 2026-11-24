use worker::{Request, Response, Result, RouteContext};

use crate::services::photo_service::PhotoService;

pub async fn health(_req: Request, _ctx: RouteContext<PhotoService>) -> Result<Response> {
    Response::ok("Ok")
}
