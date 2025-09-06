use crate::{
    common::{Acl, DataRedundancyType, Owner, StorageClass},
    error::normal_error,
    request::{Oss, OssRequest},
    Error,
};
use http::Method;
use crate::common::body_to_bytes;
use serde_derive::Deserialize;

// Returned content
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct BucketList {
    pub bucket: BucketInfo,
}

/// Detailed bucket information
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BucketInfo {
    /// Access monitoring status
    pub access_monitor: String,
    /// Comment
    pub comment: String,
    /// Creation date
    pub creation_date: String,
    /// Cross-region replication status
    pub cross_region_replication: String,
    /// Data redundancy type
    pub data_redundancy_type: DataRedundancyType,
    /// Public endpoint
    pub extranet_endpoint: String,
    /// Internal endpoint
    pub intranet_endpoint: String,
    /// Region
    pub location: String,
    /// Name
    pub name: String,
    /// Resource group
    pub resource_group_id: String,
    /// Storage class
    pub storage_class: StorageClass,
    /// Transfer acceleration status
    pub transfer_acceleration: String,
    /// Owner information
    pub owner: Owner,
    /// Access control
    pub access_control_list: AccessControlList,
    /// Server-side encryption information
    pub server_side_encryption_rule: ServerSideEncryptionRule,
    /// Logging information
    pub bucket_policy: BucketPolicy,
}

/// Bucket access control information
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AccessControlList {
    /// Access control
    pub grant: Acl,
}

/// Bucket server-side encryption information
#[derive(Debug, Deserialize)]
pub struct ServerSideEncryptionRule {
    /// Default server-side encryption algorithm
    #[serde(rename = "SSEAlgorithm")]
    pub sse_algorithm: String,
}

/// Bucket logging information
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BucketPolicy {
    /// Name of the bucket storing logs
    pub log_bucket: String,
    /// Directory for storing log files
    pub log_prefix: String,
}

/// Retrieve detailed information of the bucket
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/document_detail/31968.html) for details
pub struct GetBucketInfo {
    req: OssRequest,
}
impl GetBucketInfo {
    pub(super) fn new(oss: Oss) -> Self {
        let mut req = OssRequest::new(oss, Method::GET);
        req.insert_query("bucketInfo", "");
        GetBucketInfo { req }
    }
    /// Send the request
    pub async fn send(self) -> Result<BucketInfo, Error> {
        // Build the HTTP request
        let response = self.req.send_to_oss()?.await?;
        // Parse the response
        let status_code = response.status();
        match status_code {
            code if code.is_success() => {
                let response_bytes = body_to_bytes(response.into_body())
                    .await
                    .map_err(|_| Error::OssInvalidResponse(None))?;
                let bucket_info: BucketList = serde_xml_rs::from_reader(&*response_bytes)
                    .map_err(|_| Error::OssInvalidResponse(Some(response_bytes)))?;
                Ok(bucket_info.bucket)
            }
            _ => Err(normal_error(response).await),
        }
    }
}
