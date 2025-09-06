//! A bucket is a container for storing objects; every object must belong to a bucket.

#[doc(hidden)]
pub use self::oss_bucket::OssBucket;
#[cfg(feature = "async")]
pub use self::{
    del_bucket::DelBucket, del_bucket_logging::DelBucketLogging, del_objects::DelObjects,
    get_bucket_acl::GetBucketAcl, get_bucket_info::GetBucketInfo,
    get_bucket_location::GetBucketLocation, get_bucket_logging::GetBucketLogging,
    get_bucket_stat::GetBucketStat, list_multipart_uploads::ListUploads, list_objects::ListObjects,
    put_bucket::PutBucket, put_bucket_acl::PutBucketAcl, put_bucket_logging::PutBucketLogging,
};

#[cfg(feature = "sync")]
pub use self::{
    del_bucket_logging_sync::DelBucketLoggingSync, get_bucket_acl_sync::GetBucketAclSync,
    get_bucket_location_sync::GetBucketLocationSync, get_bucket_logging_sync::GetBucketLoggingSync,
    put_bucket_acl_sync::PutBucketAclSync, put_bucket_logging_sync::PutBucketLoggingSync,
};

#[cfg(feature = "async")]
mod del_bucket;
#[cfg(feature = "async")]
mod del_bucket_logging;
#[cfg(feature = "async")]
mod del_objects;
#[cfg(feature = "async")]
mod get_bucket_acl;
#[cfg(feature = "async")]
mod get_bucket_info;
#[cfg(feature = "async")]
mod get_bucket_location;
#[cfg(feature = "async")]
mod get_bucket_logging;
#[cfg(feature = "async")]
mod get_bucket_stat;
#[cfg(feature = "async")]
mod list_multipart_uploads;
#[cfg(feature = "async")]
mod list_objects;
mod oss_bucket;
#[cfg(feature = "async")]
mod put_bucket;
#[cfg(feature = "async")]
mod put_bucket_acl;
#[cfg(feature = "async")]
mod put_bucket_logging;

#[cfg(feature = "sync")]
mod del_bucket_logging_sync;
#[cfg(feature = "sync")]
mod get_bucket_acl_sync;
#[cfg(feature = "sync")]
mod get_bucket_location_sync;
#[cfg(feature = "sync")]
mod get_bucket_logging_sync;
#[cfg(feature = "sync")]
mod put_bucket_acl_sync;
#[cfg(feature = "sync")]
mod put_bucket_logging_sync;
