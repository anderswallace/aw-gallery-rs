use crate::services::s3_objects::s3_objects_service::S3ObjectsService;

pub trait HasS3ObjectsService {
    // Gets the S3 Objects service
    fn s3_objects_service(&self) -> &dyn S3ObjectsService;
}
