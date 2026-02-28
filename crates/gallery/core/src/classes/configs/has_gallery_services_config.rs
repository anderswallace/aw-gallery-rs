use crate::configs::gallery_services_config::GalleryServicesConfig;

pub trait HasGalleryServicesConfig {
    /// Gets Gallery Services config
    fn gallery_services_config(&self) -> &GalleryServicesConfig;
}
