use std::sync::Arc;

use anyhow::Result;

use crate::{
    configs::services::gallery_s3_objects_service_impl::GalleryS3ObjectsServiceImpl,
    io::s3_objects::s3_objects_io::S3ObjectsIO,
    services::s3_objects::{
        null_s3_objects_service::NullS3ObjectsService,
        orchestrator_s3_objects_service::OrchestratorS3ObjectsService,
        s3_objects_service::S3ObjectsService,
    },
};

/// Initialize the configured Gallery S3 objects service
pub fn init_gallery_s3_objects_service(
    impl_type: GalleryS3ObjectsServiceImpl,
    s3_objects_io: Arc<dyn S3ObjectsIO>,
) -> Result<Arc<dyn S3ObjectsService>> {
    Ok(match impl_type {
        GalleryS3ObjectsServiceImpl::Orchestrator => {
            Arc::new(OrchestratorS3ObjectsService::new(s3_objects_io))
        }
        GalleryS3ObjectsServiceImpl::Null => Arc::new(NullS3ObjectsService),
    })
}
