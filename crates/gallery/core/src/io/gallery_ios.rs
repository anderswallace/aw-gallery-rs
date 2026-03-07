use std::sync::Arc;

use crate::{
    classes::io::has_s3_objects_io::HasS3ObjectsIO, io::s3_objects::s3_objects_io::S3ObjectsIO,
};

/// Aggregates GalleryIOs
pub struct GalleryIOs {
    s3_objects: Arc<dyn S3ObjectsIO>,
}

pub struct GalleryIOsParams {
    pub s3_objects: Arc<dyn S3ObjectsIO>,
}

impl GalleryIOs {
    #[must_use]
    pub fn new(params: GalleryIOsParams) -> Self {
        Self {
            s3_objects: params.s3_objects,
        }
    }

    #[must_use]
    pub fn s3_objects_io_arc(&self) -> Arc<dyn S3ObjectsIO> {
        Arc::clone(&self.s3_objects)
    }
}

impl HasS3ObjectsIO for GalleryIOs {
    fn s3_objects_io(&self) -> &dyn S3ObjectsIO {
        self.s3_objects.as_ref()
    }
}
