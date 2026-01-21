#[cfg(feature = "async")]
use super::{
    AbortBucketWorm, CompleteBucketWorm, DelBucket, DelBucketCors, DelBucketEncryption,
    DelBucketInventory, DelBucketLifecycle, DelBucketLogging, DelBucketPolicy,
    DelBucketTags, DelBucketWebsite, DelObjects, ExtendBucketWorm, GetBucketAcl, GetBucketCors,
    GetBucketEncryption, GetBucketInfo, GetBucketInventory, GetBucketLifecycle, GetBucketLocation,
    GetBucketLogging, GetBucketPolicy, GetBucketReferer, GetBucketRequestPayment, GetBucketStat,
    GetBucketTags, GetBucketTransferAcceleration, GetBucketVersioning, GetBucketWebsite,
    GetBucketWorm, InitiateBucketWorm, ListBucketInventory, ListObjectVersions, ListObjects,
    ListObjectsV1, ListUploads, PutBucket, PutBucketAcl, PutBucketCors, PutBucketEncryption,
    PutBucketInventory, PutBucketLifecycle, PutBucketLogging, PutBucketPolicy, PutBucketReferer,
    PutBucketRequestPayment, PutBucketTags, PutBucketTransferAcceleration, PutBucketVersioning,
    PutBucketWebsite,
};
#[cfg(feature = "sync")]
use super::{
    AbortBucketWormSync, CompleteBucketWormSync, DelBucketCorsSync, DelBucketEncryptionSync,
    DelBucketInventorySync, DelBucketLifecycleSync, DelBucketLoggingSync, DelBucketPolicySync,
    DelBucketSync, DelBucketTagsSync, DelBucketWebsiteSync, DelObjectsSync,
    ExtendBucketWormSync, GetBucketAclSync, GetBucketCorsSync, GetBucketEncryptionSync,
    GetBucketInfoSync, GetBucketInventorySync, GetBucketLifecycleSync, GetBucketLocationSync,
    GetBucketLoggingSync, GetBucketPolicySync, GetBucketRefererSync, GetBucketRequestPaymentSync,
    GetBucketStatSync, GetBucketTagsSync, GetBucketTransferAccelerationSync,
    GetBucketVersioningSync, GetBucketWebsiteSync, GetBucketWormSync, InitiateBucketWormSync,
    ListBucketInventorySync, ListObjectVersionsSync, ListObjectsSync, ListObjectsV1Sync,
    ListUploadsSync, PutBucketAclSync, PutBucketCorsSync, PutBucketEncryptionSync,
    PutBucketInventorySync, PutBucketLifecycleSync, PutBucketLoggingSync, PutBucketPolicySync,
    PutBucketRefererSync, PutBucketRequestPaymentSync, PutBucketSync, PutBucketTagsSync,
    PutBucketTransferAccelerationSync, PutBucketVersioningSync, PutBucketWebsiteSync,
};
#[cfg(any(feature = "async", feature = "sync"))]
use crate::OssObject;
use crate::oss::Oss;

/// Bucket handle that exposes bucket-level APIs (lifecycle, ACL, CORS, logging, etc.).
///
/// Bucket 句柄，提供生命周期、ACL、CORS、日志等桶级 API。
#[derive(Debug, Clone)]
pub struct OssBucket {
    pub(crate) oss: Oss,
}

impl OssBucket {
    pub(crate) fn new(mut oss: Oss, bucket: impl Into<String>) -> Self {
        oss.set_bucket(bucket);
        OssBucket { oss }
    }
    /// Attach a temporary security token for STS authentication.
    ///
    /// 设置临时安全令牌用于 STS 鉴权。
    pub fn with_security_token(mut self, token: impl Into<String>) -> Self {
        self.oss.set_security_token(token);
        self
    }
    /// Update the security token in place without consuming the bucket handle.
    ///
    /// 就地更新安全令牌，不消耗句柄。
    pub fn set_security_token(&mut self, token: impl Into<String>) {
        self.oss.set_security_token(token);
    }
    /// Bind a custom domain and choose whether to use HTTPS.
    ///
    /// 绑定自定义域名并设置是否使用 HTTPS。
    pub fn set_custom_domain(mut self, custom_domain: impl Into<String>, enable_https: bool) -> Self {
        self.oss.set_custom_domain(custom_domain);
        self.oss.set_https(enable_https);
        self
    }
    /// Create an object handle under this bucket.
    ///
    /// 在当前 Bucket 下创建对象句柄。
    #[cfg(any(feature = "async", feature = "sync"))]
    pub fn object(&self, object: impl Into<String>) -> OssObject {
        OssObject::new(self.oss.clone(), object)
    }
    /// Create the bucket.
    ///
    /// 创建 Bucket。
    #[cfg(feature = "async")]
    pub fn put_bucket(&self) -> PutBucket {
        PutBucket::new(self.oss.clone())
    }
    /// Delete the bucket.
    ///
    /// 删除 Bucket。
    #[cfg(feature = "async")]
    pub fn del_bucket(&self) -> DelBucket {
        DelBucket::new(self.oss.clone())
    }
    /// List objects in the bucket (ListObjectsV2).
    ///
    /// 列举 Bucket 内对象（ListObjectsV2）。
    #[cfg(feature = "async")]
    pub fn list_objects(&self) -> ListObjects {
        ListObjects::new(self.oss.clone())
    }
    /// List objects in the bucket using the legacy ListObjects API.
    ///
    /// 使用旧版 ListObjects API 列举对象。
    #[cfg(feature = "async")]
    pub fn list_objects_v1(&self) -> ListObjectsV1 {
        ListObjectsV1::new(self.oss.clone())
    }
    /// Retrieve detailed bucket information.
    ///
    /// 获取 Bucket 详细信息。
    #[cfg(feature = "async")]
    pub fn get_bucket_info(&self) -> GetBucketInfo {
        GetBucketInfo::new(self.oss.clone())
    }
    /// Retrieve bucket storage size and object count.
    ///
    /// 获取 Bucket 的存储量与对象数量。
    #[cfg(feature = "async")]
    pub fn get_bucket_stat(&self) -> GetBucketStat {
        GetBucketStat::new(self.oss.clone())
    }
    /// Retrieve bucket lifecycle rules.
    ///
    /// 获取 Bucket 生命周期规则。
    #[cfg(feature = "async")]
    pub fn get_bucket_lifecycle(&self) -> GetBucketLifecycle {
        GetBucketLifecycle::new(self.oss.clone())
    }
    /// Retrieve bucket versioning configuration.
    ///
    /// 获取 Bucket 版本控制配置。
    #[cfg(feature = "async")]
    pub fn get_bucket_versioning(&self) -> GetBucketVersioning {
        GetBucketVersioning::new(self.oss.clone())
    }
    /// Configure bucket lifecycle rules.
    ///
    /// 配置 Bucket 生命周期规则。
    #[cfg(feature = "async")]
    pub fn put_bucket_lifecycle(&self) -> PutBucketLifecycle {
        PutBucketLifecycle::new(self.oss.clone())
    }
    /// Configure bucket versioning.
    ///
    /// 配置 Bucket 版本控制。
    #[cfg(feature = "async")]
    pub fn put_bucket_versioning(&self) -> PutBucketVersioning {
        PutBucketVersioning::new(self.oss.clone())
    }
    /// Delete all bucket lifecycle rules.
    ///
    /// 删除 Bucket 的全部生命周期规则。
    #[cfg(feature = "async")]
    pub fn del_bucket_lifecycle(&self) -> DelBucketLifecycle {
        DelBucketLifecycle::new(self.oss.clone())
    }
    /// Retrieve bucket location.
    ///
    /// 获取 Bucket 所在地域。
    #[cfg(feature = "async")]
    pub fn get_bucket_location(&self) -> GetBucketLocation {
        GetBucketLocation::new(self.oss.clone())
    }
    /// Retrieve bucket ACL.
    ///
    /// 获取 Bucket ACL。
    #[cfg(feature = "async")]
    pub fn get_bucket_acl(&self) -> GetBucketAcl {
        GetBucketAcl::new(self.oss.clone())
    }
    /// Set bucket ACL.
    ///
    /// 设置 Bucket ACL。
    #[cfg(feature = "async")]
    pub fn put_bucket_acl(&self) -> PutBucketAcl {
        PutBucketAcl::new(self.oss.clone())
    }
    /// Retrieve bucket referer configuration.
    ///
    /// 获取 Bucket Referer 配置。
    #[cfg(feature = "async")]
    pub fn get_bucket_referer(&self) -> GetBucketReferer {
        GetBucketReferer::new(self.oss.clone())
    }
    /// Configure bucket referer whitelist.
    ///
    /// 配置 Bucket Referer 白名单。
    #[cfg(feature = "async")]
    pub fn put_bucket_referer(&self) -> PutBucketReferer {
        PutBucketReferer::new(self.oss.clone())
    }
    /// Retrieve bucket tags.
    ///
    /// 获取 Bucket 标签。
    #[cfg(feature = "async")]
    pub fn get_bucket_tags(&self) -> GetBucketTags {
        GetBucketTags::new(self.oss.clone())
    }
    /// Configure bucket tags.
    ///
    /// 配置 Bucket 标签。
    #[cfg(feature = "async")]
    pub fn put_bucket_tags(
        &self,
        tags: Vec<(impl Into<String>, impl Into<String>)>,
    ) -> PutBucketTags {
        PutBucketTags::new(self.oss.clone(), tags)
    }
    /// Delete all bucket tags.
    ///
    /// 删除 Bucket 的全部标签。
    #[cfg(feature = "async")]
    pub fn del_bucket_tags(&self) -> DelBucketTags {
        DelBucketTags::new(self.oss.clone())
    }
    /// Retrieve bucket policy.
    ///
    /// 获取 Bucket Policy。
    #[cfg(feature = "async")]
    pub fn get_bucket_policy(&self) -> GetBucketPolicy {
        GetBucketPolicy::new(self.oss.clone())
    }
    /// Configure bucket policy.
    ///
    /// 配置 Bucket Policy。
    #[cfg(feature = "async")]
    pub fn put_bucket_policy(&self) -> PutBucketPolicy {
        PutBucketPolicy::new(self.oss.clone())
    }
    /// Delete bucket policy.
    ///
    /// 删除 Bucket Policy。
    #[cfg(feature = "async")]
    pub fn del_bucket_policy(&self) -> DelBucketPolicy {
        DelBucketPolicy::new(self.oss.clone())
    }
    /// Retrieve bucket encryption configuration.
    ///
    /// 获取 Bucket 默认加密配置。
    #[cfg(feature = "async")]
    pub fn get_bucket_encryption(&self) -> GetBucketEncryption {
        GetBucketEncryption::new(self.oss.clone())
    }
    /// Configure default bucket encryption.
    ///
    /// 配置 Bucket 默认加密。
    #[cfg(feature = "async")]
    pub fn put_bucket_encryption(&self) -> PutBucketEncryption {
        PutBucketEncryption::new(self.oss.clone())
    }
    /// Delete bucket encryption configuration.
    ///
    /// 删除 Bucket 默认加密配置。
    #[cfg(feature = "async")]
    pub fn del_bucket_encryption(&self) -> DelBucketEncryption {
        DelBucketEncryption::new(self.oss.clone())
    }
    /// Retrieve transfer acceleration configuration.
    ///
    /// 获取传输加速配置。
    #[cfg(feature = "async")]
    pub fn get_bucket_transfer_acceleration(&self) -> GetBucketTransferAcceleration {
        GetBucketTransferAcceleration::new(self.oss.clone())
    }
    /// Configure transfer acceleration for the bucket.
    ///
    /// 配置 Bucket 传输加速。
    #[cfg(feature = "async")]
    pub fn put_bucket_transfer_acceleration(&self) -> PutBucketTransferAcceleration {
        PutBucketTransferAcceleration::new(self.oss.clone())
    }
    /// Retrieve requester pays configuration.
    ///
    /// 获取请求者付费配置。
    #[cfg(feature = "async")]
    pub fn get_bucket_request_payment(&self) -> GetBucketRequestPayment {
        GetBucketRequestPayment::new(self.oss.clone())
    }
    /// Configure requester pays for the bucket.
    ///
    /// 配置 Bucket 请求者付费。
    #[cfg(feature = "async")]
    pub fn put_bucket_request_payment(&self) -> PutBucketRequestPayment {
        PutBucketRequestPayment::new(self.oss.clone())
    }
    /// Initiate WORM retention configuration.
    ///
    /// 初始化 WORM 合规保留策略。
    #[cfg(feature = "async")]
    pub fn initiate_bucket_worm(&self) -> InitiateBucketWorm {
        InitiateBucketWorm::new(self.oss.clone())
    }
    /// Retrieve WORM configuration.
    ///
    /// 获取 WORM 配置。
    #[cfg(feature = "async")]
    pub fn get_bucket_worm(&self) -> GetBucketWorm {
        GetBucketWorm::new(self.oss.clone())
    }
    /// Complete (lock) WORM configuration.
    ///
    /// 完成（锁定）WORM 配置。
    #[cfg(feature = "async")]
    pub fn complete_bucket_worm(&self, worm_id: impl Into<String>) -> CompleteBucketWorm {
        CompleteBucketWorm::new(self.oss.clone(), worm_id)
    }
    /// Extend WORM retention period.
    ///
    /// 延长 WORM 保留期限。
    #[cfg(feature = "async")]
    pub fn extend_bucket_worm(&self, worm_id: impl Into<String>) -> ExtendBucketWorm {
        ExtendBucketWorm::new(self.oss.clone(), worm_id)
    }
    /// Abort an unlocked WORM configuration.
    ///
    /// 终止未锁定的 WORM 配置。
    #[cfg(feature = "async")]
    pub fn abort_bucket_worm(&self, worm_id: impl Into<String>) -> AbortBucketWorm {
        AbortBucketWorm::new(self.oss.clone(), worm_id)
    }
    /// Configure an inventory task.
    ///
    /// 配置清单任务。
    #[cfg(feature = "async")]
    pub fn put_bucket_inventory(&self, inventory_id: impl Into<String>) -> PutBucketInventory {
        PutBucketInventory::new(self.oss.clone(), inventory_id)
    }
    /// Retrieve an inventory task configuration.
    ///
    /// 获取清单任务配置。
    #[cfg(feature = "async")]
    pub fn get_bucket_inventory(&self, inventory_id: impl Into<String>) -> GetBucketInventory {
        GetBucketInventory::new(self.oss.clone(), inventory_id)
    }
    /// Delete an inventory task configuration.
    ///
    /// 删除清单任务配置。
    #[cfg(feature = "async")]
    pub fn del_bucket_inventory(&self, inventory_id: impl Into<String>) -> DelBucketInventory {
        DelBucketInventory::new(self.oss.clone(), inventory_id)
    }
    /// List inventory task configurations.
    ///
    /// 列举清单任务配置。
    #[cfg(feature = "async")]
    pub fn list_bucket_inventory(&self) -> ListBucketInventory {
        ListBucketInventory::new(self.oss.clone())
    }
    /// List versions of objects in the bucket.
    ///
    /// 列举 Bucket 内对象版本。
    #[cfg(feature = "async")]
    pub fn list_object_versions(&self) -> ListObjectVersions {
        ListObjectVersions::new(self.oss.clone())
    }
    /// Retrieve static website configuration.
    ///
    /// 获取静态网站配置。
    #[cfg(feature = "async")]
    pub fn get_bucket_website(&self) -> GetBucketWebsite {
        GetBucketWebsite::new(self.oss.clone())
    }
    /// Configure static website behavior.
    ///
    /// 配置静态网站规则。
    #[cfg(feature = "async")]
    pub fn put_bucket_website(&self) -> PutBucketWebsite {
        PutBucketWebsite::new(self.oss.clone())
    }
    /// Delete static website configuration.
    ///
    /// 删除静态网站配置。
    #[cfg(feature = "async")]
    pub fn del_bucket_website(&self) -> DelBucketWebsite {
        DelBucketWebsite::new(self.oss.clone())
    }
    /// Retrieve bucket CORS rules.
    ///
    /// 获取 Bucket CORS 规则。
    #[cfg(feature = "async")]
    pub fn get_bucket_cors(&self) -> GetBucketCors {
        GetBucketCors::new(self.oss.clone())
    }
    /// Replace bucket CORS rules.
    ///
    /// 替换 Bucket CORS 规则。
    #[cfg(feature = "async")]
    pub fn put_bucket_cors(&self) -> PutBucketCors {
        PutBucketCors::new(self.oss.clone())
    }
    /// Delete all bucket CORS rules.
    ///
    /// 删除 Bucket 全部 CORS 规则。
    #[cfg(feature = "async")]
    pub fn del_bucket_cors(&self) -> DelBucketCors {
        DelBucketCors::new(self.oss.clone())
    }
    /// Retrieve bucket logging configuration.
    ///
    /// 获取 Bucket 日志配置。
    #[cfg(feature = "async")]
    pub fn get_bucket_logging(&self) -> GetBucketLogging {
        GetBucketLogging::new(self.oss.clone())
    }
    /// Configure bucket logging.
    ///
    /// 配置 Bucket 日志。
    #[cfg(feature = "async")]
    pub fn put_bucket_logging(
        &self,
        target_bucket: impl Into<String>,
        target_prefix: impl Into<String>,
    ) -> PutBucketLogging {
        PutBucketLogging::new(self.oss.clone(), target_bucket, target_prefix)
    }
    /// Delete bucket logging configuration.
    ///
    /// 删除 Bucket 日志配置。
    #[cfg(feature = "async")]
    pub fn del_bucket_logging(&self) -> DelBucketLogging {
        DelBucketLogging::new(self.oss.clone())
    }

    /// Delete multiple objects.
    ///
    /// 批量删除对象。
    #[cfg(feature = "async")]
    pub fn del_objects(&self, files: Vec<impl Into<String>>) -> DelObjects {
        DelObjects::new(self.oss.clone(), files)
    }
    /// List multipart uploads that are initiated but not completed.
    ///
    /// 列举已初始化但未完成的分片上传。
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
        let bucket = OssBucket::new(Oss::new("id", "secret", "cn-example"), "my-bucket")
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

#[cfg(feature = "sync")]
impl OssBucket {
    /// Create the bucket (sync).
    ///
    /// 创建 Bucket（同步）。
    pub fn put_bucket_sync(&self) -> PutBucketSync {
        PutBucketSync::new(self.oss.clone())
    }
    /// Delete the bucket (sync).
    ///
    /// 删除 Bucket（同步）。
    pub fn del_bucket_sync(&self) -> DelBucketSync {
        DelBucketSync::new(self.oss.clone())
    }
    /// List objects in the bucket (ListObjectsV2) (sync).
    ///
    /// 列举 Bucket 内对象（ListObjectsV2）（同步）。
    pub fn list_objects_sync(&self) -> ListObjectsSync {
        ListObjectsSync::new(self.oss.clone())
    }
    /// List objects in the bucket using the legacy ListObjects API (sync).
    ///
    /// 使用旧版 ListObjects API 列举对象（同步）。
    pub fn list_objects_v1_sync(&self) -> ListObjectsV1Sync {
        ListObjectsV1Sync::new(self.oss.clone())
    }
    /// Retrieve detailed bucket information (sync).
    ///
    /// 获取 Bucket 详细信息（同步）。
    pub fn get_bucket_info_sync(&self) -> GetBucketInfoSync {
        GetBucketInfoSync::new(self.oss.clone())
    }
    /// Retrieve bucket storage size and object count (sync).
    ///
    /// 获取 Bucket 的存储量与对象数量（同步）。
    pub fn get_bucket_stat_sync(&self) -> GetBucketStatSync {
        GetBucketStatSync::new(self.oss.clone())
    }
    /// Retrieve bucket lifecycle rules (sync).
    ///
    /// 获取 Bucket 生命周期规则（同步）。
    pub fn get_bucket_lifecycle_sync(&self) -> GetBucketLifecycleSync {
        GetBucketLifecycleSync::new(self.oss.clone())
    }
    /// Retrieve bucket versioning configuration (sync).
    ///
    /// 获取 Bucket 版本控制配置（同步）。
    pub fn get_bucket_versioning_sync(&self) -> GetBucketVersioningSync {
        GetBucketVersioningSync::new(self.oss.clone())
    }
    /// Configure bucket lifecycle rules (sync).
    ///
    /// 配置 Bucket 生命周期规则（同步）。
    pub fn put_bucket_lifecycle_sync(&self) -> PutBucketLifecycleSync {
        PutBucketLifecycleSync::new(self.oss.clone())
    }
    /// Configure bucket versioning (sync).
    ///
    /// 配置 Bucket 版本控制（同步）。
    pub fn put_bucket_versioning_sync(&self) -> PutBucketVersioningSync {
        PutBucketVersioningSync::new(self.oss.clone())
    }
    /// Delete all bucket lifecycle rules (sync).
    ///
    /// 删除 Bucket 的全部生命周期规则（同步）。
    pub fn del_bucket_lifecycle_sync(&self) -> DelBucketLifecycleSync {
        DelBucketLifecycleSync::new(self.oss.clone())
    }
    /// Retrieve bucket location (sync).
    ///
    /// 获取 Bucket 所在地域（同步）。
    pub fn get_bucket_location_sync(&self) -> GetBucketLocationSync {
        GetBucketLocationSync::new(self.oss.clone())
    }
    /// Retrieve bucket ACL (sync).
    ///
    /// 获取 Bucket ACL（同步）。
    pub fn get_bucket_acl_sync(&self) -> GetBucketAclSync {
        GetBucketAclSync::new(self.oss.clone())
    }
    /// Set bucket ACL (sync).
    ///
    /// 设置 Bucket ACL（同步）。
    pub fn put_bucket_acl_sync(&self) -> PutBucketAclSync {
        PutBucketAclSync::new(self.oss.clone())
    }
    /// Retrieve bucket referer configuration (sync).
    ///
    /// 获取 Bucket Referer 配置（同步）。
    pub fn get_bucket_referer_sync(&self) -> GetBucketRefererSync {
        GetBucketRefererSync::new(self.oss.clone())
    }
    /// Configure bucket referer whitelist (sync).
    ///
    /// 配置 Bucket Referer 白名单（同步）。
    pub fn put_bucket_referer_sync(&self) -> PutBucketRefererSync {
        PutBucketRefererSync::new(self.oss.clone())
    }
    /// Retrieve bucket tags (sync).
    ///
    /// 获取 Bucket 标签（同步）。
    pub fn get_bucket_tags_sync(&self) -> GetBucketTagsSync {
        GetBucketTagsSync::new(self.oss.clone())
    }
    /// Configure bucket tags (sync).
    ///
    /// 配置 Bucket 标签（同步）。
    pub fn put_bucket_tags_sync(
        &self,
        tags: Vec<(impl Into<String>, impl Into<String>)>,
    ) -> PutBucketTagsSync {
        PutBucketTagsSync::new(self.oss.clone(), tags)
    }
    /// Delete all bucket tags (sync).
    ///
    /// 删除 Bucket 的全部标签（同步）。
    pub fn del_bucket_tags_sync(&self) -> DelBucketTagsSync {
        DelBucketTagsSync::new(self.oss.clone())
    }
    /// Retrieve bucket policy (sync).
    ///
    /// 获取 Bucket Policy（同步）。
    pub fn get_bucket_policy_sync(&self) -> GetBucketPolicySync {
        GetBucketPolicySync::new(self.oss.clone())
    }
    /// Configure bucket policy (sync).
    ///
    /// 配置 Bucket Policy（同步）。
    pub fn put_bucket_policy_sync(&self) -> PutBucketPolicySync {
        PutBucketPolicySync::new(self.oss.clone())
    }
    /// Delete bucket policy (sync).
    ///
    /// 删除 Bucket Policy（同步）。
    pub fn del_bucket_policy_sync(&self) -> DelBucketPolicySync {
        DelBucketPolicySync::new(self.oss.clone())
    }
    /// Retrieve bucket encryption configuration (sync).
    ///
    /// 获取 Bucket 加密配置（同步）。
    pub fn get_bucket_encryption_sync(&self) -> GetBucketEncryptionSync {
        GetBucketEncryptionSync::new(self.oss.clone())
    }
    /// Configure bucket encryption (sync).
    ///
    /// 配置 Bucket 加密（同步）。
    pub fn put_bucket_encryption_sync(&self) -> PutBucketEncryptionSync {
        PutBucketEncryptionSync::new(self.oss.clone())
    }
    /// Delete bucket encryption configuration (sync).
    ///
    /// 删除 Bucket 加密配置（同步）。
    pub fn del_bucket_encryption_sync(&self) -> DelBucketEncryptionSync {
        DelBucketEncryptionSync::new(self.oss.clone())
    }
    /// Retrieve transfer acceleration configuration (sync).
    ///
    /// 获取传输加速配置（同步）。
    pub fn get_bucket_transfer_acceleration_sync(&self) -> GetBucketTransferAccelerationSync {
        GetBucketTransferAccelerationSync::new(self.oss.clone())
    }
    /// Configure transfer acceleration (sync).
    ///
    /// 配置传输加速（同步）。
    pub fn put_bucket_transfer_acceleration_sync(&self) -> PutBucketTransferAccelerationSync {
        PutBucketTransferAccelerationSync::new(self.oss.clone())
    }
    /// Retrieve requester pays configuration (sync).
    ///
    /// 获取请求者付费配置（同步）。
    pub fn get_bucket_request_payment_sync(&self) -> GetBucketRequestPaymentSync {
        GetBucketRequestPaymentSync::new(self.oss.clone())
    }
    /// Configure requester pays for the bucket (sync).
    ///
    /// 配置 Bucket 请求者付费（同步）。
    pub fn put_bucket_request_payment_sync(&self) -> PutBucketRequestPaymentSync {
        PutBucketRequestPaymentSync::new(self.oss.clone())
    }
    /// Initiate WORM retention configuration (sync).
    ///
    /// 初始化 WORM 合规保留策略（同步）。
    pub fn initiate_bucket_worm_sync(&self) -> InitiateBucketWormSync {
        InitiateBucketWormSync::new(self.oss.clone())
    }
    /// Retrieve WORM configuration (sync).
    ///
    /// 获取 WORM 配置（同步）。
    pub fn get_bucket_worm_sync(&self) -> GetBucketWormSync {
        GetBucketWormSync::new(self.oss.clone())
    }
    /// Complete (lock) WORM configuration (sync).
    ///
    /// 完成（锁定）WORM 配置（同步）。
    pub fn complete_bucket_worm_sync(&self, worm_id: impl Into<String>) -> CompleteBucketWormSync {
        CompleteBucketWormSync::new(self.oss.clone(), worm_id)
    }
    /// Extend WORM retention period (sync).
    ///
    /// 延长 WORM 保留期限（同步）。
    pub fn extend_bucket_worm_sync(&self, worm_id: impl Into<String>) -> ExtendBucketWormSync {
        ExtendBucketWormSync::new(self.oss.clone(), worm_id)
    }
    /// Abort an unlocked WORM configuration (sync).
    ///
    /// 终止未锁定的 WORM 配置（同步）。
    pub fn abort_bucket_worm_sync(&self, worm_id: impl Into<String>) -> AbortBucketWormSync {
        AbortBucketWormSync::new(self.oss.clone(), worm_id)
    }
    /// Configure an inventory task (sync).
    ///
    /// 配置清单任务（同步）。
    pub fn put_bucket_inventory_sync(&self, inventory_id: impl Into<String>) -> PutBucketInventorySync {
        PutBucketInventorySync::new(self.oss.clone(), inventory_id)
    }
    /// Retrieve an inventory task configuration (sync).
    ///
    /// 获取清单任务配置（同步）。
    pub fn get_bucket_inventory_sync(&self, inventory_id: impl Into<String>) -> GetBucketInventorySync {
        GetBucketInventorySync::new(self.oss.clone(), inventory_id)
    }
    /// Delete an inventory task configuration (sync).
    ///
    /// 删除清单任务配置（同步）。
    pub fn del_bucket_inventory_sync(&self, inventory_id: impl Into<String>) -> DelBucketInventorySync {
        DelBucketInventorySync::new(self.oss.clone(), inventory_id)
    }
    /// List inventory task configurations (sync).
    ///
    /// 列举清单任务配置（同步）。
    pub fn list_bucket_inventory_sync(&self) -> ListBucketInventorySync {
        ListBucketInventorySync::new(self.oss.clone())
    }
    /// List versions of objects in the bucket (sync).
    ///
    /// 列举 Bucket 内对象版本（同步）。
    pub fn list_object_versions_sync(&self) -> ListObjectVersionsSync {
        ListObjectVersionsSync::new(self.oss.clone())
    }
    /// Retrieve static website configuration (sync).
    ///
    /// 获取静态网站配置（同步）。
    pub fn get_bucket_website_sync(&self) -> GetBucketWebsiteSync {
        GetBucketWebsiteSync::new(self.oss.clone())
    }
    /// Configure static website behavior (sync).
    ///
    /// 配置静态网站规则（同步）。
    pub fn put_bucket_website_sync(&self) -> PutBucketWebsiteSync {
        PutBucketWebsiteSync::new(self.oss.clone())
    }
    /// Delete static website configuration (sync).
    ///
    /// 删除静态网站配置（同步）。
    pub fn del_bucket_website_sync(&self) -> DelBucketWebsiteSync {
        DelBucketWebsiteSync::new(self.oss.clone())
    }
    /// Retrieve bucket CORS rules (sync).
    ///
    /// 获取 Bucket CORS 规则（同步）。
    pub fn get_bucket_cors_sync(&self) -> GetBucketCorsSync {
        GetBucketCorsSync::new(self.oss.clone())
    }
    /// Replace bucket CORS rules (sync).
    ///
    /// 替换 Bucket CORS 规则（同步）。
    pub fn put_bucket_cors_sync(&self) -> PutBucketCorsSync {
        PutBucketCorsSync::new(self.oss.clone())
    }
    /// Delete all bucket CORS rules (sync).
    ///
    /// 删除 Bucket 全部 CORS 规则（同步）。
    pub fn del_bucket_cors_sync(&self) -> DelBucketCorsSync {
        DelBucketCorsSync::new(self.oss.clone())
    }
    /// Retrieve bucket logging configuration (sync).
    ///
    /// 获取 Bucket 日志配置（同步）。
    pub fn get_bucket_logging_sync(&self) -> GetBucketLoggingSync {
        GetBucketLoggingSync::new(self.oss.clone())
    }
    /// Configure bucket logging (sync).
    ///
    /// 配置 Bucket 日志（同步）。
    pub fn put_bucket_logging_sync(
        &self,
        target_bucket: impl Into<String>,
        target_prefix: impl Into<String>,
    ) -> PutBucketLoggingSync {
        PutBucketLoggingSync::new(self.oss.clone(), target_bucket, target_prefix)
    }
    /// Delete bucket logging configuration (sync).
    ///
    /// 删除 Bucket 日志配置（同步）。
    pub fn del_bucket_logging_sync(&self) -> DelBucketLoggingSync {
        DelBucketLoggingSync::new(self.oss.clone())
    }
    /// Delete multiple objects (sync).
    ///
    /// 批量删除对象（同步）。
    pub fn del_objects_sync(&self, files: Vec<impl Into<String>>) -> DelObjectsSync {
        DelObjectsSync::new(self.oss.clone(), files)
    }
    /// List multipart uploads that are initiated but not completed (sync).
    ///
    /// 列举已初始化但未完成的分片上传（同步）。
    pub fn multipart_list_uploads_sync(&self) -> ListUploadsSync {
        ListUploadsSync::new(self.oss.clone())
    }
}
