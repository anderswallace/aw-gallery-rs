use std::sync::Arc;

use anyhow::Result;

use crate::{
    classes::{
        configs::{
            has_gallery_io_config::HasGalleryIoConfig,
            has_gallery_services_config::HasGalleryServicesConfig,
        },
        services::{has_db_service::HasDbService, has_s3_objects_service::HasS3ObjectsService},
    },
    init::{
        io::init_gallery_ios::init_gallery_ios,
        services::{
            init_gallery_db_service::init_gallery_db_service,
            init_gallery_s3_objects_service::init_gallery_s3_objects_service,
        },
    },
    io::gallery_ios::GalleryIOs,
    services::{db::db_service::DbService, s3_objects::s3_objects_service::S3ObjectsService},
};

/// Aggregates Gallery services
pub struct GalleryServices {
    s3_objects: Arc<dyn S3ObjectsService>,
    db: Arc<dyn DbService>,
}

impl GalleryServices {
    /// Builds Gallery services from config
    pub async fn from_config<T>(config: &T) -> Result<Self>
    where
        T: HasGalleryServicesConfig + HasGalleryIoConfig + Send + Sync + 'static,
    {
        let services_config = config.gallery_services_config();
        let ios: GalleryIOs = init_gallery_ios(config).await?;

        let s3_objects =
            init_gallery_s3_objects_service(services_config.s3_objects, ios.s3_objects_io_arc())?;

        let db = init_gallery_db_service(services_config.db)?;

        Ok(Self { s3_objects, db })
    }
}

impl HasS3ObjectsService for GalleryServices {
    fn s3_objects_service(&self) -> &dyn S3ObjectsService {
        self.s3_objects.as_ref()
    }
}

impl HasDbService for GalleryServices {
    fn db_service(&self) -> &dyn DbService {
        self.db.as_ref()
    }
}
