use crate::configs::gallery_io_config::GalleryIoConfig;

pub trait HasGalleryIoConfig {
    // Gets Gallery IO config
    fn gallery_io_config(&self) -> &GalleryIoConfig;
}
