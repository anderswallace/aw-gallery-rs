use anyhow::Result;
use std::sync::Arc;

use crate::configs::gallery_io_config::GalleryIoConfig;
use crate::configs::io::gallery_s3_objects_io_impl::GalleryS3ObjectsIoImpl;
use crate::io::s3_objects::null_s3_objects_io::NullS3ObjectsIO;
use crate::io::s3_objects::s3_objects_io::S3ObjectsIO;

/// Initializes the S3 Objects IO based on config
pub fn init_s3_objects_io(config: &GalleryIoConfig) -> Result<Arc<dyn S3ObjectsIO>> {
    let io: Arc<dyn S3ObjectsIO> = match config.s3_objects {
        GalleryS3ObjectsIoImpl::Aws => Arc::new(AwsS3ObjectsIO::new(&config.s3)?),
        GalleryS3ObjectsIoImpl::Null => Arc::new(NullS3ObjectsIO),
    };
    Ok(io)
}
