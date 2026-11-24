use worker::{Request, Response, Result, RouteContext};

use crate::services::photo_service::PhotoService;

pub async fn upload(mut _req: Request, _ctx: RouteContext<PhotoService>) -> Result<Response> {
    // let form = req.form_data().await?;
    // let camera = form.get_field("camera");
    // let film = form
    //     .get_field("film")
    //     .filter(|value| !value.trim().is_empty());
    //
    // let Some(FormEntry::File(file)) = form.get("file") else {
    //     return Response::error("Missing file", 400);
    // };

    Response::ok("Photo uploaded")
}
