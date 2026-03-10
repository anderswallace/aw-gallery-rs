use std::sync::Arc;

use anyhow::{Context, Result};
use gallery_core::{
    configs::gallery_config::GalleryConfig, services::gallery_services::GalleryServices,
};

use crate::state::GalleryWebState;

/// Loads Gallery configuration and initializes the core services for the web API
pub async fn init_state_from_disk() -> Result<Arc<GalleryWebState>> {
    let config = GalleryConfig::load()?;
    let gallery_services = Arc::new(
        GalleryServices::from_config(&config)
            .await
            .context("Initializing Gallery services for web server")?,
    );

    Ok(Arc::new(GalleryWebState::new(config, gallery_services)))
}
