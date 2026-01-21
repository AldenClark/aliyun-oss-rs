//! A bucket is a container for storing objects; every object must belong to a bucket.
//!
//! Bucket 是对象存储的容器，每个对象必须属于某个 Bucket。

#[doc(hidden)]
pub use self::oss_bucket::OssBucket;
#[cfg(feature = "async")]
pub use self::{
    abort_bucket_worm::AbortBucketWorm,
    complete_bucket_worm::CompleteBucketWorm,
    del_bucket::DelBucket,
    del_bucket_cors::DelBucketCors,
    del_bucket_encryption::DelBucketEncryption,
    del_bucket_inventory::DelBucketInventory,
    del_bucket_lifecycle::DelBucketLifecycle,
    del_bucket_logging::DelBucketLogging,
    del_bucket_policy::DelBucketPolicy,
    del_bucket_website::DelBucketWebsite,
    del_objects::DelObjects,
    extend_bucket_worm::ExtendBucketWorm,
    get_bucket_acl::GetBucketAcl,
    get_bucket_cors::GetBucketCors,
    get_bucket_encryption::GetBucketEncryption,
    get_bucket_info::GetBucketInfo,
    get_bucket_inventory::GetBucketInventory,
    get_bucket_lifecycle::GetBucketLifecycle,
    get_bucket_location::GetBucketLocation,
    get_bucket_logging::GetBucketLogging,
    get_bucket_policy::GetBucketPolicy,
    get_bucket_referer::GetBucketReferer,
    get_bucket_request_payment::GetBucketRequestPayment,
    get_bucket_stat::GetBucketStat,
    get_bucket_tags::GetBucketTags,
    get_bucket_transfer_acceleration::GetBucketTransferAcceleration,
    get_bucket_versioning::GetBucketVersioning,
    get_bucket_website::GetBucketWebsite,
    get_bucket_worm::GetBucketWorm,
    initiate_bucket_worm::InitiateBucketWorm,
    list_bucket_inventory::ListBucketInventory,
    list_object_versions::ListObjectVersions,
    list_objects_v1::ListObjectsV1,
    list_multipart_uploads::ListUploads,
    list_objects::ListObjects,
    put_bucket::PutBucket,
    put_bucket_acl::PutBucketAcl,
    put_bucket_cors::{CorsRule, PutBucketCors},
    put_bucket_encryption::PutBucketEncryption,
    put_bucket_inventory::PutBucketInventory,
    put_bucket_lifecycle::PutBucketLifecycle,
    put_bucket_logging::PutBucketLogging,
    put_bucket_policy::PutBucketPolicy,
    put_bucket_referer::PutBucketReferer,
    put_bucket_request_payment::PutBucketRequestPayment,
    put_bucket_tags::PutBucketTags,
    put_bucket_transfer_acceleration::PutBucketTransferAcceleration,
    put_bucket_versioning::PutBucketVersioning,
    put_bucket_website::PutBucketWebsite,
    del_bucket_tags::DelBucketTags,
};

#[cfg(feature = "sync")]
pub use self::{
    abort_bucket_worm_sync::AbortBucketWormSync,
    complete_bucket_worm_sync::CompleteBucketWormSync,
    del_bucket_cors_sync::DelBucketCorsSync,
    del_bucket_encryption_sync::DelBucketEncryptionSync,
    del_bucket_inventory_sync::DelBucketInventorySync,
    del_bucket_lifecycle_sync::DelBucketLifecycleSync,
    del_bucket_logging_sync::DelBucketLoggingSync,
    del_bucket_policy_sync::DelBucketPolicySync,
    del_bucket_sync::DelBucketSync,
    del_bucket_tags_sync::DelBucketTagsSync,
    del_bucket_website_sync::DelBucketWebsiteSync,
    del_objects_sync::DelObjectsSync,
    extend_bucket_worm_sync::ExtendBucketWormSync,
    get_bucket_acl_sync::GetBucketAclSync,
    get_bucket_cors_sync::GetBucketCorsSync,
    get_bucket_encryption_sync::GetBucketEncryptionSync,
    get_bucket_info_sync::GetBucketInfoSync,
    get_bucket_inventory_sync::GetBucketInventorySync,
    get_bucket_lifecycle_sync::GetBucketLifecycleSync,
    get_bucket_location_sync::GetBucketLocationSync,
    get_bucket_logging_sync::GetBucketLoggingSync,
    get_bucket_policy_sync::GetBucketPolicySync,
    get_bucket_referer_sync::GetBucketRefererSync,
    get_bucket_request_payment_sync::GetBucketRequestPaymentSync,
    get_bucket_stat_sync::GetBucketStatSync,
    get_bucket_tags_sync::GetBucketTagsSync,
    get_bucket_transfer_acceleration_sync::GetBucketTransferAccelerationSync,
    get_bucket_versioning_sync::GetBucketVersioningSync,
    get_bucket_website_sync::GetBucketWebsiteSync,
    get_bucket_worm_sync::GetBucketWormSync,
    initiate_bucket_worm_sync::InitiateBucketWormSync,
    list_bucket_inventory_sync::ListBucketInventorySync,
    list_multipart_uploads_sync::ListUploadsSync,
    list_object_versions_sync::ListObjectVersionsSync,
    list_objects_sync::ListObjectsSync,
    list_objects_v1_sync::ListObjectsV1Sync,
    put_bucket_acl_sync::PutBucketAclSync,
    put_bucket_cors_sync::{CorsRule as CorsRuleSync, PutBucketCorsSync},
    put_bucket_encryption_sync::PutBucketEncryptionSync,
    put_bucket_inventory_sync::PutBucketInventorySync,
    put_bucket_lifecycle_sync::PutBucketLifecycleSync,
    put_bucket_logging_sync::PutBucketLoggingSync,
    put_bucket_policy_sync::PutBucketPolicySync,
    put_bucket_referer_sync::PutBucketRefererSync,
    put_bucket_request_payment_sync::PutBucketRequestPaymentSync,
    put_bucket_sync::PutBucketSync,
    put_bucket_tags_sync::PutBucketTagsSync,
    put_bucket_transfer_acceleration_sync::PutBucketTransferAccelerationSync,
    put_bucket_versioning_sync::PutBucketVersioningSync,
    put_bucket_website_sync::PutBucketWebsiteSync,
};

#[cfg(feature = "async")]
mod abort_bucket_worm;
#[cfg(feature = "async")]
mod complete_bucket_worm;
#[cfg(feature = "async")]
mod del_bucket;
#[cfg(feature = "async")]
mod del_bucket_cors;
#[cfg(feature = "async")]
mod del_bucket_encryption;
#[cfg(feature = "async")]
mod del_bucket_inventory;
#[cfg(feature = "async")]
mod del_bucket_lifecycle;
#[cfg(feature = "async")]
mod del_bucket_logging;
#[cfg(feature = "async")]
mod del_bucket_policy;
#[cfg(feature = "async")]
mod del_bucket_website;
#[cfg(feature = "async")]
mod del_objects;
#[cfg(feature = "async")]
mod extend_bucket_worm;
#[cfg(feature = "async")]
mod get_bucket_acl;
#[cfg(feature = "async")]
mod get_bucket_cors;
#[cfg(feature = "async")]
mod get_bucket_encryption;
#[cfg(feature = "async")]
mod get_bucket_info;
#[cfg(feature = "async")]
mod get_bucket_inventory;
#[cfg(feature = "async")]
mod get_bucket_lifecycle;
#[cfg(feature = "async")]
mod get_bucket_location;
#[cfg(feature = "async")]
mod get_bucket_logging;
#[cfg(feature = "async")]
mod get_bucket_policy;
#[cfg(feature = "async")]
mod get_bucket_referer;
#[cfg(feature = "async")]
mod get_bucket_request_payment;
#[cfg(feature = "async")]
mod get_bucket_stat;
#[cfg(feature = "async")]
mod get_bucket_tags;
#[cfg(feature = "async")]
mod get_bucket_transfer_acceleration;
#[cfg(feature = "async")]
mod get_bucket_versioning;
#[cfg(feature = "async")]
mod get_bucket_website;
#[cfg(feature = "async")]
mod get_bucket_worm;
#[cfg(feature = "async")]
mod initiate_bucket_worm;
#[cfg(feature = "async")]
mod list_bucket_inventory;
#[cfg(feature = "async")]
mod list_object_versions;
#[cfg(feature = "async")]
mod list_objects_v1;
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
mod put_bucket_cors;
#[cfg(feature = "async")]
mod put_bucket_encryption;
#[cfg(feature = "async")]
mod put_bucket_inventory;
#[cfg(feature = "async")]
mod put_bucket_lifecycle;
#[cfg(feature = "async")]
mod put_bucket_logging;
#[cfg(feature = "async")]
mod put_bucket_policy;
#[cfg(feature = "async")]
mod put_bucket_referer;
#[cfg(feature = "async")]
mod put_bucket_request_payment;
#[cfg(feature = "async")]
mod put_bucket_tags;
#[cfg(feature = "async")]
mod put_bucket_transfer_acceleration;
#[cfg(feature = "async")]
mod put_bucket_versioning;
#[cfg(feature = "async")]
mod put_bucket_website;
#[cfg(feature = "async")]
mod del_bucket_tags;

#[cfg(feature = "sync")]
mod abort_bucket_worm_sync;
#[cfg(feature = "sync")]
mod complete_bucket_worm_sync;
#[cfg(feature = "sync")]
mod del_bucket_cors_sync;
#[cfg(feature = "sync")]
mod del_bucket_encryption_sync;
#[cfg(feature = "sync")]
mod del_bucket_inventory_sync;
#[cfg(feature = "sync")]
mod del_bucket_lifecycle_sync;
#[cfg(feature = "sync")]
mod del_bucket_logging_sync;
#[cfg(feature = "sync")]
mod del_bucket_policy_sync;
#[cfg(feature = "sync")]
mod del_bucket_sync;
#[cfg(feature = "sync")]
mod del_bucket_tags_sync;
#[cfg(feature = "sync")]
mod del_bucket_website_sync;
#[cfg(feature = "sync")]
mod del_objects_sync;
#[cfg(feature = "sync")]
mod extend_bucket_worm_sync;
#[cfg(feature = "sync")]
mod get_bucket_acl_sync;
#[cfg(feature = "sync")]
mod get_bucket_cors_sync;
#[cfg(feature = "sync")]
mod get_bucket_encryption_sync;
#[cfg(feature = "sync")]
mod get_bucket_info_sync;
#[cfg(feature = "sync")]
mod get_bucket_inventory_sync;
#[cfg(feature = "sync")]
mod get_bucket_lifecycle_sync;
#[cfg(feature = "sync")]
mod get_bucket_location_sync;
#[cfg(feature = "sync")]
mod get_bucket_logging_sync;
#[cfg(feature = "sync")]
mod get_bucket_policy_sync;
#[cfg(feature = "sync")]
mod get_bucket_referer_sync;
#[cfg(feature = "sync")]
mod get_bucket_request_payment_sync;
#[cfg(feature = "sync")]
mod get_bucket_stat_sync;
#[cfg(feature = "sync")]
mod get_bucket_tags_sync;
#[cfg(feature = "sync")]
mod get_bucket_transfer_acceleration_sync;
#[cfg(feature = "sync")]
mod get_bucket_versioning_sync;
#[cfg(feature = "sync")]
mod get_bucket_website_sync;
#[cfg(feature = "sync")]
mod get_bucket_worm_sync;
#[cfg(feature = "sync")]
mod initiate_bucket_worm_sync;
#[cfg(feature = "sync")]
mod list_bucket_inventory_sync;
#[cfg(feature = "sync")]
mod list_multipart_uploads_sync;
#[cfg(feature = "sync")]
mod list_object_versions_sync;
#[cfg(feature = "sync")]
mod list_objects_sync;
#[cfg(feature = "sync")]
mod list_objects_v1_sync;
#[cfg(feature = "sync")]
mod put_bucket_acl_sync;
#[cfg(feature = "sync")]
mod put_bucket_cors_sync;
#[cfg(feature = "sync")]
mod put_bucket_encryption_sync;
#[cfg(feature = "sync")]
mod put_bucket_inventory_sync;
#[cfg(feature = "sync")]
mod put_bucket_lifecycle_sync;
#[cfg(feature = "sync")]
mod put_bucket_logging_sync;
#[cfg(feature = "sync")]
mod put_bucket_policy_sync;
#[cfg(feature = "sync")]
mod put_bucket_referer_sync;
#[cfg(feature = "sync")]
mod put_bucket_request_payment_sync;
#[cfg(feature = "sync")]
mod put_bucket_sync;
#[cfg(feature = "sync")]
mod put_bucket_tags_sync;
#[cfg(feature = "sync")]
mod put_bucket_transfer_acceleration_sync;
#[cfg(feature = "sync")]
mod put_bucket_versioning_sync;
#[cfg(feature = "sync")]
mod put_bucket_website_sync;

#[cfg(feature = "async")]
#[derive(Debug, Default, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(rename = "CORSConfiguration")]
pub(super) struct CorsConfiguration {
    #[serde(rename = "CORSRule", default)]
    pub(crate) rules: Vec<put_bucket_cors::CorsRule>,
}

#[cfg(any(feature = "async", feature = "sync"))]
#[derive(Debug, Default, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(rename = "RefererConfiguration")]
/// Referer configuration payload.
///
/// Referer 配置载荷。
pub struct RefererConfiguration {
    #[serde(rename = "AllowEmptyReferer")]
    pub allow_empty_referer: bool,
    #[serde(rename = "RefererList", default)]
    pub referer_list: RefererList,
}

#[cfg(any(feature = "async", feature = "sync"))]
#[derive(Debug, Default, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "PascalCase")]
/// Referer list payload.
///
/// Referer 列表载荷。
pub struct RefererList {
    #[serde(rename = "Referer", default)]
    pub items: Vec<String>,
}

#[cfg(any(feature = "async", feature = "sync"))]
impl RefererConfiguration {
    pub fn referers(&self) -> &[String] {
        &self.referer_list.items
    }
}

#[cfg(any(feature = "async", feature = "sync"))]
#[derive(Debug, Default, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(rename = "WebsiteConfiguration")]
/// Static website configuration payload.
///
/// 静态网站配置载荷。
pub struct WebsiteConfiguration {
    #[serde(rename = "IndexDocument", skip_serializing_if = "Option::is_none")]
    pub index_document: Option<IndexDocument>,
    #[serde(rename = "ErrorDocument", skip_serializing_if = "Option::is_none")]
    pub error_document: Option<ErrorDocument>,
    #[serde(rename = "RoutingRules", skip_serializing_if = "Option::is_none")]
    pub routing_rules: Option<RoutingRules>,
}

#[cfg(any(feature = "async", feature = "sync"))]
#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "PascalCase")]
/// Index document configuration.
///
/// 索引页配置。
pub struct IndexDocument {
    #[serde(rename = "Suffix")]
    pub suffix: String,
}

#[cfg(any(feature = "async", feature = "sync"))]
#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "PascalCase")]
/// Error document configuration.
///
/// 错误页配置。
pub struct ErrorDocument {
    #[serde(rename = "Key")]
    pub key: String,
}

#[cfg(any(feature = "async", feature = "sync"))]
#[derive(Debug, Default, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "PascalCase")]
/// Routing rules payload.
///
/// 路由规则载荷。
pub struct RoutingRules {
    #[serde(rename = "RoutingRule", default)]
    pub rules: Vec<RoutingRule>,
}

#[cfg(any(feature = "async", feature = "sync"))]
#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "PascalCase")]
/// Routing rule entry.
///
/// 路由规则条目。
pub struct RoutingRule {
    #[serde(rename = "Condition", skip_serializing_if = "Option::is_none")]
    pub condition: Option<RoutingRuleCondition>,
    #[serde(rename = "Redirect")]
    pub redirect: RoutingRuleRedirect,
}

#[cfg(any(feature = "async", feature = "sync"))]
#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "PascalCase")]
/// Routing rule condition.
///
/// 路由规则条件。
pub struct RoutingRuleCondition {
    #[serde(rename = "KeyPrefixEquals", skip_serializing_if = "Option::is_none")]
    pub key_prefix_equals: Option<String>,
    #[serde(
        rename = "HttpErrorCodeReturnedEquals",
        skip_serializing_if = "Option::is_none"
    )]
    pub http_error_code_returned_equals: Option<String>,
}

#[cfg(any(feature = "async", feature = "sync"))]
#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "PascalCase")]
/// Routing rule redirect action.
///
/// 路由规则重定向动作。
pub struct RoutingRuleRedirect {
    #[serde(rename = "Protocol", skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(rename = "HostName", skip_serializing_if = "Option::is_none")]
    pub host_name: Option<String>,
    #[serde(rename = "ReplaceKeyWith", skip_serializing_if = "Option::is_none")]
    pub replace_key_with: Option<String>,
    #[serde(
        rename = "ReplaceKeyPrefixWith",
        skip_serializing_if = "Option::is_none"
    )]
    pub replace_key_prefix_with: Option<String>,
    #[serde(rename = "HttpRedirectCode", skip_serializing_if = "Option::is_none")]
    pub http_redirect_code: Option<String>,
}

#[cfg(any(feature = "async", feature = "sync"))]
#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(rename = "ServerSideEncryptionConfiguration")]
/// Server-side encryption configuration payload.
///
/// 服务端加密配置载荷。
pub struct BucketEncryption {
    #[serde(rename = "Rule")]
    pub rule: ServerSideEncryptionRule,
}

#[cfg(any(feature = "async", feature = "sync"))]
impl Default for BucketEncryption {
    fn default() -> Self {
        BucketEncryption {
            rule: ServerSideEncryptionRule {
                default_sse: ApplyServerSideEncryptionByDefault {
                    sse_algorithm: "AES256".into(),
                    kms_master_key_id: None,
                    kms_data_encryption: None,
                },
            },
        }
    }
}

#[cfg(any(feature = "async", feature = "sync"))]
#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "PascalCase")]
/// Server-side encryption rule.
///
/// 服务端加密规则。
pub struct ServerSideEncryptionRule {
    #[serde(rename = "ApplyServerSideEncryptionByDefault")]
    pub default_sse: ApplyServerSideEncryptionByDefault,
}

#[cfg(any(feature = "async", feature = "sync"))]
#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "PascalCase")]
/// Default server-side encryption rule.
///
/// 默认服务端加密规则。
pub struct ApplyServerSideEncryptionByDefault {
    #[serde(rename = "SSEAlgorithm")]
    pub sse_algorithm: String,
    #[serde(rename = "KMSMasterKeyID", skip_serializing_if = "Option::is_none")]
    pub kms_master_key_id: Option<String>,
    #[serde(rename = "KMSDataEncryption", skip_serializing_if = "Option::is_none")]
    pub kms_data_encryption: Option<String>,
}

#[cfg(any(feature = "async", feature = "sync"))]
#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(rename = "WormConfiguration")]
/// WORM configuration payload.
///
/// WORM 配置载荷。
pub struct BucketWormConfiguration {
    #[serde(rename = "WormId", skip_serializing_if = "Option::is_none")]
    pub worm_id: Option<String>,
    #[serde(rename = "State", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "CreationDate", skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(
        rename = "RetentionPeriodInDays",
        skip_serializing_if = "Option::is_none"
    )]
    pub retention_period_in_days: Option<u32>,
}

#[cfg(any(feature = "async", feature = "sync"))]
#[derive(Debug, Default, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(rename = "VersioningConfiguration")]
/// Versioning configuration payload.
///
/// 版本控制配置载荷。
pub struct VersioningConfiguration {
    #[serde(rename = "Status")]
    pub status: VersioningStatus,
}

#[cfg(any(feature = "async", feature = "sync"))]
#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "PascalCase")]
/// Versioning status.
///
/// 版本控制状态。
pub enum VersioningStatus {
    Enabled,
    Suspended,
}

#[cfg(any(feature = "async", feature = "sync"))]
impl Default for VersioningStatus {
    fn default() -> Self {
        VersioningStatus::Suspended
    }
}

#[cfg(any(feature = "async", feature = "sync"))]
#[derive(Debug, Default, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(rename = "TransferAccelerationConfiguration")]
/// Transfer acceleration configuration payload.
///
/// 传输加速配置载荷。
pub struct TransferAccelerationConfiguration {
    #[serde(rename = "Enabled")]
    pub enabled: bool,
}

#[cfg(any(feature = "async", feature = "sync"))]
#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(rename = "RequestPaymentConfiguration")]
/// Request payment configuration payload.
///
/// 请求者付费配置载荷。
pub struct RequestPaymentConfiguration {
    #[serde(rename = "Payer")]
    pub payer: RequestPayer,
}

#[cfg(any(feature = "async", feature = "sync"))]
#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "PascalCase")]
/// Request payer.
///
/// 请求付费方。
pub enum RequestPayer {
    Requester,
    BucketOwner,
}

#[cfg(any(feature = "async", feature = "sync"))]
impl Default for RequestPayer {
    fn default() -> Self {
        RequestPayer::BucketOwner
    }
}

#[cfg(any(feature = "async", feature = "sync"))]
#[derive(Debug, Default, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(rename = "Tagging")]
/// Bucket tagging payload.
///
/// Bucket 标签载荷。
pub struct BucketTagging {
    #[serde(rename = "TagSet")]
    pub tag_set: TagSet,
}

#[cfg(any(feature = "async", feature = "sync"))]
#[derive(Debug, Default, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "PascalCase")]
/// Tag set payload.
///
/// 标签集合载荷。
pub struct TagSet {
    #[serde(rename = "Tag", default)]
    pub tags: Vec<Tag>,
}

#[cfg(any(feature = "async", feature = "sync"))]
#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "PascalCase")]
/// Tag entry.
///
/// 标签条目。
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[cfg(any(feature = "async", feature = "sync"))]
impl BucketTagging {
    pub fn tags(&self) -> &[Tag] {
        &self.tag_set.tags
    }
}
