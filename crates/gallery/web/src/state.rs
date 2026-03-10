use std::sync::Arc;

use gallery_core::configs::gallery_config::GalleryConfig;
use gallery_core::services::gallery_services::GalleryServices;

/// Share application state for Gallery web API
pub struct GalleryWebState {
    config: GalleryConfig,
    services: Arc<GalleryServices>,
}

impl GalleryWebState {
    #[must_use]
    pub fn new(config: GalleryConfig, services: Arc<GalleryServices>) -> Self {
        Self { config, services }
    }

    #[must_use]
    pub fn config(&self) -> &GalleryConfig {
        &self.config
    }

    #[must_use]
    pub fn services(&self) -> &GalleryServices {
        self.services.as_ref()
    }

    #[must_use]
    pub fn services_arc(&self) -> Arc<GalleryServices> {
        Arc::clone(&self.services)
    }
}
