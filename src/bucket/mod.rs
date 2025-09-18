//! A bucket is a container for storing objects; every object must belong to a bucket.

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
    del_bucket_referer::DelBucketReferer,
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
    get_bucket_stat::GetBucketStat,
    get_bucket_website::GetBucketWebsite,
    get_bucket_worm::GetBucketWorm,
    initiate_bucket_worm::InitiateBucketWorm,
    list_bucket_inventory::ListBucketInventory,
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
    put_bucket_website::PutBucketWebsite,
};

#[cfg(feature = "sync")]
pub use self::{
    del_bucket_logging_sync::DelBucketLoggingSync, get_bucket_acl_sync::GetBucketAclSync,
    get_bucket_location_sync::GetBucketLocationSync, get_bucket_logging_sync::GetBucketLoggingSync,
    put_bucket_acl_sync::PutBucketAclSync, put_bucket_logging_sync::PutBucketLoggingSync,
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
mod del_bucket_referer;
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
mod get_bucket_stat;
#[cfg(feature = "async")]
mod get_bucket_website;
#[cfg(feature = "async")]
mod get_bucket_worm;
#[cfg(feature = "async")]
mod initiate_bucket_worm;
#[cfg(feature = "async")]
mod list_bucket_inventory;
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
mod put_bucket_website;

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

#[cfg(feature = "async")]
#[derive(Debug, Default, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(rename = "CORSConfiguration")]
pub(super) struct CorsConfiguration {
    #[serde(rename = "CORSRule", default)]
    pub(crate) rules: Vec<put_bucket_cors::CorsRule>,
}

#[cfg(feature = "async")]
#[derive(Debug, Default, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(rename = "RefererConfiguration")]
pub struct RefererConfiguration {
    #[serde(rename = "AllowEmptyReferer")]
    pub allow_empty_referer: bool,
    #[serde(rename = "RefererList", default)]
    pub referer_list: RefererList,
}

#[cfg(feature = "async")]
#[derive(Debug, Default, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RefererList {
    #[serde(rename = "Referer", default)]
    pub items: Vec<String>,
}

#[cfg(feature = "async")]
impl RefererConfiguration {
    pub fn referers(&self) -> &[String] {
        &self.referer_list.items
    }
}

#[cfg(feature = "async")]
#[derive(Debug, Default, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(rename = "WebsiteConfiguration")]
pub struct WebsiteConfiguration {
    #[serde(rename = "IndexDocument", skip_serializing_if = "Option::is_none")]
    pub index_document: Option<IndexDocument>,
    #[serde(rename = "ErrorDocument", skip_serializing_if = "Option::is_none")]
    pub error_document: Option<ErrorDocument>,
    #[serde(rename = "RoutingRules", skip_serializing_if = "Option::is_none")]
    pub routing_rules: Option<RoutingRules>,
}

#[cfg(feature = "async")]
#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct IndexDocument {
    #[serde(rename = "Suffix")]
    pub suffix: String,
}

#[cfg(feature = "async")]
#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ErrorDocument {
    #[serde(rename = "Key")]
    pub key: String,
}

#[cfg(feature = "async")]
#[derive(Debug, Default, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RoutingRules {
    #[serde(rename = "RoutingRule", default)]
    pub rules: Vec<RoutingRule>,
}

#[cfg(feature = "async")]
#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RoutingRule {
    #[serde(rename = "Condition", skip_serializing_if = "Option::is_none")]
    pub condition: Option<RoutingRuleCondition>,
    #[serde(rename = "Redirect")]
    pub redirect: RoutingRuleRedirect,
}

#[cfg(feature = "async")]
#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RoutingRuleCondition {
    #[serde(rename = "KeyPrefixEquals", skip_serializing_if = "Option::is_none")]
    pub key_prefix_equals: Option<String>,
    #[serde(
        rename = "HttpErrorCodeReturnedEquals",
        skip_serializing_if = "Option::is_none"
    )]
    pub http_error_code_returned_equals: Option<String>,
}

#[cfg(feature = "async")]
#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "PascalCase")]
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

#[cfg(feature = "async")]
#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(rename = "ServerSideEncryptionConfiguration")]
pub struct BucketEncryption {
    #[serde(rename = "Rule")]
    pub rule: ServerSideEncryptionRule,
}

#[cfg(feature = "async")]
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

#[cfg(feature = "async")]
#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ServerSideEncryptionRule {
    #[serde(rename = "ApplyServerSideEncryptionByDefault")]
    pub default_sse: ApplyServerSideEncryptionByDefault,
}

#[cfg(feature = "async")]
#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ApplyServerSideEncryptionByDefault {
    #[serde(rename = "SSEAlgorithm")]
    pub sse_algorithm: String,
    #[serde(rename = "KMSMasterKeyID", skip_serializing_if = "Option::is_none")]
    pub kms_master_key_id: Option<String>,
    #[serde(rename = "KMSDataEncryption", skip_serializing_if = "Option::is_none")]
    pub kms_data_encryption: Option<String>,
}

#[cfg(feature = "async")]
#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(rename = "WormConfiguration")]
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
