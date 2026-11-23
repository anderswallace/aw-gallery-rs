use worker::{Request, Response, Result, RouteContext};

pub async fn upload(mut _req: Request, _ctx: RouteContext<()>) -> Result<Response> {
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
