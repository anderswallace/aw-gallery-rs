use crate::io::s3_objects::s3_objects_io::S3ObjectsIO;

pub trait HasS3ObjectsIO {
    // Gets the S3 Objects IO
    fn s3_objects_io(&self) -> &dyn S3ObjectsIO;
}
