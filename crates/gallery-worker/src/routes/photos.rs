use worker::{FormEntry, Request, Response, Result, RouteContext};

use crate::services::photo_service::{PhotoService, PhotoUpload};

pub async fn upload(mut req: Request, ctx: RouteContext<PhotoService>) -> Result<Response> {
    let photo_service = ctx.data;
    let form = req.form_data().await?;
    let Some(camera) = form
        .get_field("camera")
        .filter(|value| !value.trim().is_empty())
    else {
        return Response::error("Camera is required", 400);
    };

    let film = form
        .get_field("film")
        .filter(|value| !value.trim().is_empty());

    let Some(FormEntry::File(file)) = form.get("file") else {
        return Response::error("Missing file", 400);
    };

    let bytes = file.bytes().await?;

    let photo = PhotoUpload {
        filename: file.name(),
        content_type: file.type_(),
        camera,
        film,
        bytes,
    };

    photo_service.upload(photo).await?;

    Response::ok("Photo uploaded")
}
