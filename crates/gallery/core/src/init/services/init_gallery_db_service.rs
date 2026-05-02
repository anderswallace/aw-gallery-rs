use std::sync::Arc;

use anyhow::Result;

use crate::{
    configs::services::gallery_db_service_impl::GalleryDbServiceImpl,
    services::db::{db_service::DbService, null_db_service::NullDbService},
};

/// Initialize the configured Gallery DB service
pub fn init_gallery_db_service(impl_type: GalleryDbServiceImpl) -> Result<Arc<dyn DbService>> {
    Ok(match impl_type {
        GalleryDbServiceImpl::Null => Arc::new(NullDbService),
    })
}
