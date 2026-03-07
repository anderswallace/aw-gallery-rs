use anyhow::Result;

use crate::{
    classes::configs::has_gallery_io_config::HasGalleryIoConfig,
    init::io::init_s3_objects_io::init_s3_objects_io,
    io::gallery_ios::{GalleryIOs, GalleryIOsParams},
};

pub async fn init_gallery_ios<T>(config: &T) -> Result<GalleryIOs>
where
    T: HasGalleryIoConfig + Send + Sync + 'static,
{
    let io_config = config.gallery_io_config();
    let s3_objects = init_s3_objects_io(io_config).await?;

    Ok(GalleryIOs::new(GalleryIOsParams { s3_objects }))
}
