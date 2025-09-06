#[cfg(feature = "async")]
use super::{
    DelBucket, DelBucketLogging, DelObjects, GetBucketAcl, GetBucketInfo, GetBucketLocation,
    GetBucketLogging, GetBucketStat, ListObjects, ListUploads, PutBucket, PutBucketAcl,
    PutBucketLogging,
};
#[cfg(feature = "sync")]
use super::{
    DelBucketLoggingSync, GetBucketAclSync, GetBucketLocationSync, GetBucketLoggingSync,
    PutBucketAclSync, PutBucketLoggingSync,
};
use crate::oss::Oss;
#[cfg(feature = "async")]
use crate::OssObject;

/// OSS bucket implementing APIs for creating buckets, retrieving bucket information, listing objects, and more
#[derive(Debug, Clone)]
pub struct OssBucket {
    pub(crate) oss: Oss,
}

impl OssBucket {
    pub(crate) fn new(mut oss: Oss, bucket: impl ToString, endpoint: impl ToString) -> Self {
        oss.set_bucket(bucket);
        oss.set_endpoint(endpoint);
        OssBucket { oss }
    }
    /// Set a custom domain
    ///
    pub fn set_custom_domain(mut self, custom_domain: impl ToString, enable_https: bool) -> Self {
        self.oss.set_custom_domain(custom_domain);
        self.oss.set_https(enable_https);
        self
    }
    /// Initialize an OssObject
    #[cfg(feature = "async")]
    pub fn object(&self, object: impl ToString) -> OssObject {
        OssObject::new(self.oss.clone(), object)
    }
    /// Create a bucket
    #[cfg(feature = "async")]
    pub fn put_bucket(&self) -> PutBucket {
        PutBucket::new(self.oss.clone())
    }
    /// Delete a bucket
    #[cfg(feature = "async")]
    pub fn del_bucket(&self) -> DelBucket {
        DelBucket::new(self.oss.clone())
    }
    /// List all objects in the bucket
    #[cfg(feature = "async")]
    pub fn list_objects(&self) -> ListObjects {
        ListObjects::new(self.oss.clone())
    }
    /// Retrieve detailed information of the bucket
    #[cfg(feature = "async")]
    pub fn get_bucket_info(&self) -> GetBucketInfo {
        GetBucketInfo::new(self.oss.clone())
    }
    /// Retrieve the bucket's storage size and file count
    #[cfg(feature = "async")]
    pub fn get_bucket_stat(&self) -> GetBucketStat {
        GetBucketStat::new(self.oss.clone())
    }
    /// Retrieve the bucket location
    #[cfg(feature = "async")]
    pub fn get_bucket_location(&self) -> GetBucketLocation {
        GetBucketLocation::new(self.oss.clone())
    }
    /// Get the bucket access control
    #[cfg(feature = "async")]
    pub fn get_bucket_acl(&self) -> GetBucketAcl {
        GetBucketAcl::new(self.oss.clone())
    }
    /// Set the bucket access control
    #[cfg(feature = "async")]
    pub fn put_bucket_acl(&self) -> PutBucketAcl {
        PutBucketAcl::new(self.oss.clone())
    }
    /// Get the bucket logging configuration
    #[cfg(feature = "async")]
    pub fn get_bucket_logging(&self) -> GetBucketLogging {
        GetBucketLogging::new(self.oss.clone())
    }
    /// Configure the bucket logging
    #[cfg(feature = "async")]
    pub fn put_bucket_logging(
        &self,
        target_bucket: impl ToString,
        target_prefix: impl ToString,
    ) -> PutBucketLogging {
        PutBucketLogging::new(self.oss.clone(), target_bucket, target_prefix)
    }
    /// Delete the bucket logging configuration
    #[cfg(feature = "async")]
    pub fn del_bucket_logging(&self) -> DelBucketLogging {
        DelBucketLogging::new(self.oss.clone())
    }

    #[cfg(feature = "sync")]
    /// Retrieve the bucket location (synchronous)
    pub fn get_bucket_location_sync(&self) -> GetBucketLocationSync {
        GetBucketLocationSync::new(self.oss.clone())
    }
    #[cfg(feature = "sync")]
    /// Get the bucket access control (synchronous)
    pub fn get_bucket_acl_sync(&self) -> GetBucketAclSync {
        GetBucketAclSync::new(self.oss.clone())
    }
    #[cfg(feature = "sync")]
    /// Set the bucket access control (synchronous)
    pub fn put_bucket_acl_sync(&self) -> PutBucketAclSync {
        PutBucketAclSync::new(self.oss.clone())
    }
    #[cfg(feature = "sync")]
    /// Get the bucket logging configuration (synchronous)
    pub fn get_bucket_logging_sync(&self) -> GetBucketLoggingSync {
        GetBucketLoggingSync::new(self.oss.clone())
    }
    #[cfg(feature = "sync")]
    /// Configure the bucket logging (synchronous)
    pub fn put_bucket_logging_sync(
        &self,
        target_bucket: impl ToString,
        target_prefix: impl ToString,
    ) -> PutBucketLoggingSync {
        PutBucketLoggingSync::new(self.oss.clone(), target_bucket, target_prefix)
    }
    #[cfg(feature = "sync")]
    /// Delete the bucket logging configuration (synchronous)
    pub fn del_bucket_logging_sync(&self) -> DelBucketLoggingSync {
        DelBucketLoggingSync::new(self.oss.clone())
    }
    /// Delete multiple objects
    #[cfg(feature = "async")]
    pub fn del_objects(&self, files: Vec<impl ToString>) -> DelObjects {
        DelObjects::new(self.oss.clone(), files)
    }
    /// List multipart uploads that have been initiated but not completed
    #[cfg(feature = "async")]
    pub fn multipart_list_uploads(&self) -> ListUploads {
        ListUploads::new(self.oss.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bucket_creation_and_custom_domain() {
        let bucket = OssBucket::new(
            Oss::new("id", "secret"),
            "my-bucket",
            "oss-cn-example.aliyuncs.com",
        )
        .set_custom_domain("cdn.example.com", false);
        assert_eq!(bucket.oss.bucket.as_deref(), Some("my-bucket"));
        assert_eq!(bucket.oss.endpoint.as_ref(), "oss-cn-example.aliyuncs.com");
        assert_eq!(bucket.oss.custom_domain.as_deref(), Some("cdn.example.com"));
        assert!(!bucket.oss.enable_https);
    }
}
