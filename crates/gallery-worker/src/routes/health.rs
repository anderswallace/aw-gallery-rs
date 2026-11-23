use worker::{Request, Response, Result, RouteContext};

pub async fn health(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    Response::ok("Ok")
}
