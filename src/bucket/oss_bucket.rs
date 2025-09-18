#[cfg(feature = "async")]
use super::{
    AbortBucketWorm, CompleteBucketWorm, DelBucket, DelBucketCors, DelBucketEncryption,
    DelBucketInventory, DelBucketLifecycle, DelBucketLogging, DelBucketPolicy, DelBucketReferer,
    DelBucketWebsite, DelObjects, ExtendBucketWorm, GetBucketAcl, GetBucketCors,
    GetBucketEncryption, GetBucketInfo, GetBucketInventory, GetBucketLifecycle, GetBucketLocation,
    GetBucketLogging, GetBucketPolicy, GetBucketReferer, GetBucketStat, GetBucketWebsite,
    GetBucketWorm, InitiateBucketWorm, ListBucketInventory, ListObjects, ListUploads, PutBucket,
    PutBucketAcl, PutBucketCors, PutBucketEncryption, PutBucketInventory, PutBucketLifecycle,
    PutBucketLogging, PutBucketPolicy, PutBucketReferer, PutBucketWebsite,
};
#[cfg(feature = "sync")]
use super::{
    DelBucketLoggingSync, GetBucketAclSync, GetBucketLocationSync, GetBucketLoggingSync,
    PutBucketAclSync, PutBucketLoggingSync,
};
#[cfg(feature = "async")]
use crate::OssObject;
use crate::oss::Oss;

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
    /// Attach a temporary security token for STS authentication
    pub fn with_security_token(mut self, token: impl Into<String>) -> Self {
        self.oss.set_security_token(token);
        self
    }
    /// Update security token without consuming the bucket
    pub fn set_security_token(&mut self, token: impl Into<String>) {
        self.oss.set_security_token(token);
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
    /// Retrieve the lifecycle rules configured on the bucket
    #[cfg(feature = "async")]
    pub fn get_bucket_lifecycle(&self) -> GetBucketLifecycle {
        GetBucketLifecycle::new(self.oss.clone())
    }
    /// Configure lifecycle rules for the bucket
    #[cfg(feature = "async")]
    pub fn put_bucket_lifecycle(&self) -> PutBucketLifecycle {
        PutBucketLifecycle::new(self.oss.clone())
    }
    /// Delete all lifecycle rules for the bucket
    #[cfg(feature = "async")]
    pub fn del_bucket_lifecycle(&self) -> DelBucketLifecycle {
        DelBucketLifecycle::new(self.oss.clone())
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
    /// Retrieve the referer configuration
    #[cfg(feature = "async")]
    pub fn get_bucket_referer(&self) -> GetBucketReferer {
        GetBucketReferer::new(self.oss.clone())
    }
    /// Configure the referer whitelist
    #[cfg(feature = "async")]
    pub fn put_bucket_referer(&self) -> PutBucketReferer {
        PutBucketReferer::new(self.oss.clone())
    }
    /// Delete the referer configuration
    #[cfg(feature = "async")]
    pub fn del_bucket_referer(&self) -> DelBucketReferer {
        DelBucketReferer::new(self.oss.clone())
    }
    /// Retrieve the bucket policy
    #[cfg(feature = "async")]
    pub fn get_bucket_policy(&self) -> GetBucketPolicy {
        GetBucketPolicy::new(self.oss.clone())
    }
    /// Configure the bucket policy
    #[cfg(feature = "async")]
    pub fn put_bucket_policy(&self) -> PutBucketPolicy {
        PutBucketPolicy::new(self.oss.clone())
    }
    /// Delete the bucket policy
    #[cfg(feature = "async")]
    pub fn del_bucket_policy(&self) -> DelBucketPolicy {
        DelBucketPolicy::new(self.oss.clone())
    }
    /// Retrieve the bucket encryption configuration
    #[cfg(feature = "async")]
    pub fn get_bucket_encryption(&self) -> GetBucketEncryption {
        GetBucketEncryption::new(self.oss.clone())
    }
    /// Configure default bucket encryption
    #[cfg(feature = "async")]
    pub fn put_bucket_encryption(&self) -> PutBucketEncryption {
        PutBucketEncryption::new(self.oss.clone())
    }
    /// Delete the bucket encryption configuration
    #[cfg(feature = "async")]
    pub fn del_bucket_encryption(&self) -> DelBucketEncryption {
        DelBucketEncryption::new(self.oss.clone())
    }
    /// Initiate a WORM retention configuration
    #[cfg(feature = "async")]
    pub fn initiate_bucket_worm(&self) -> InitiateBucketWorm {
        InitiateBucketWorm::new(self.oss.clone())
    }
    /// Retrieve the WORM configuration
    #[cfg(feature = "async")]
    pub fn get_bucket_worm(&self) -> GetBucketWorm {
        GetBucketWorm::new(self.oss.clone())
    }
    /// Complete a WORM configuration
    #[cfg(feature = "async")]
    pub fn complete_bucket_worm(&self, worm_id: impl ToString) -> CompleteBucketWorm {
        CompleteBucketWorm::new(self.oss.clone(), worm_id)
    }
    /// Extend a WORM configuration
    #[cfg(feature = "async")]
    pub fn extend_bucket_worm(&self, worm_id: impl ToString) -> ExtendBucketWorm {
        ExtendBucketWorm::new(self.oss.clone(), worm_id)
    }
    /// Abort a WORM configuration
    #[cfg(feature = "async")]
    pub fn abort_bucket_worm(&self, worm_id: impl ToString) -> AbortBucketWorm {
        AbortBucketWorm::new(self.oss.clone(), worm_id)
    }
    /// Configure an inventory task
    #[cfg(feature = "async")]
    pub fn put_bucket_inventory(&self, inventory_id: impl ToString) -> PutBucketInventory {
        PutBucketInventory::new(self.oss.clone(), inventory_id)
    }
    /// Retrieve an inventory task configuration
    #[cfg(feature = "async")]
    pub fn get_bucket_inventory(&self, inventory_id: impl ToString) -> GetBucketInventory {
        GetBucketInventory::new(self.oss.clone(), inventory_id)
    }
    /// Delete an inventory task configuration
    #[cfg(feature = "async")]
    pub fn del_bucket_inventory(&self, inventory_id: impl ToString) -> DelBucketInventory {
        DelBucketInventory::new(self.oss.clone(), inventory_id)
    }
    /// List inventory task configurations
    #[cfg(feature = "async")]
    pub fn list_bucket_inventory(&self) -> ListBucketInventory {
        ListBucketInventory::new(self.oss.clone())
    }
    /// Retrieve the static website configuration
    #[cfg(feature = "async")]
    pub fn get_bucket_website(&self) -> GetBucketWebsite {
        GetBucketWebsite::new(self.oss.clone())
    }
    /// Configure the static website behavior
    #[cfg(feature = "async")]
    pub fn put_bucket_website(&self) -> PutBucketWebsite {
        PutBucketWebsite::new(self.oss.clone())
    }
    /// Delete the static website configuration
    #[cfg(feature = "async")]
    pub fn del_bucket_website(&self) -> DelBucketWebsite {
        DelBucketWebsite::new(self.oss.clone())
    }
    /// Retrieve the CORS rules configured on the bucket
    #[cfg(feature = "async")]
    pub fn get_bucket_cors(&self) -> GetBucketCors {
        GetBucketCors::new(self.oss.clone())
    }
    /// Replace the CORS rules configured on the bucket
    #[cfg(feature = "async")]
    pub fn put_bucket_cors(&self) -> PutBucketCors {
        PutBucketCors::new(self.oss.clone())
    }
    /// Delete all CORS rules configured on the bucket
    #[cfg(feature = "async")]
    pub fn del_bucket_cors(&self) -> DelBucketCors {
        DelBucketCors::new(self.oss.clone())
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
        .with_security_token("token")
        .set_custom_domain("cdn.example.com", false);
        assert_eq!(bucket.oss.bucket.as_deref(), Some("my-bucket"));
        assert_eq!(bucket.oss.endpoint.as_ref(), "oss-cn-example.aliyuncs.com");
        assert_eq!(bucket.oss.custom_domain.as_deref(), Some("cdn.example.com"));
        assert!(!bucket.oss.enable_https);
        assert_eq!(bucket.oss.security_token.as_deref(), Some("token"));

        let mut bucket = bucket.clone();
        bucket.set_security_token("token2");
        assert_eq!(bucket.oss.security_token.as_deref(), Some("token2"));
    }
}
