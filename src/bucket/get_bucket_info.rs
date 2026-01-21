use crate::common::body_to_bytes;
use crate::{
    Error,
    common::{Acl, DataRedundancyType, Owner, StorageClass},
    error::normal_error,
    request::{Oss, OssRequest},
};
use http::Method;
use serde_derive::Deserialize;

// Returned content
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct BucketList {
    pub bucket: BucketInfo,
}

/// Detailed bucket information.
///
/// Bucket 详细信息。
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BucketInfo {
    /// Access monitoring status.
    ///
    /// 访问监控状态。
    pub access_monitor: String,
    /// Comment.
    ///
    /// 备注。
    pub comment: String,
    /// Creation date.
    ///
    /// 创建时间。
    pub creation_date: String,
    /// Cross-region replication status.
    ///
    /// 跨区域复制状态。
    pub cross_region_replication: String,
    /// Data redundancy type.
    ///
    /// 冗余类型。
    pub data_redundancy_type: DataRedundancyType,
    /// Public endpoint.
    ///
    /// 公网 Endpoint。
    pub extranet_endpoint: String,
    /// Internal endpoint.
    ///
    /// 内网 Endpoint。
    pub intranet_endpoint: String,
    /// Region.
    ///
    /// 地域。
    pub location: String,
    /// Bucket name.
    ///
    /// Bucket 名称。
    pub name: String,
    /// Resource group ID.
    ///
    /// 资源组 ID。
    pub resource_group_id: String,
    /// Storage class.
    ///
    /// 存储类型。
    pub storage_class: StorageClass,
    /// Transfer acceleration status.
    ///
    /// 传输加速状态。
    pub transfer_acceleration: String,
    /// Owner information.
    ///
    /// 所有者信息。
    pub owner: Owner,
    /// Access control list.
    ///
    /// 访问控制列表。
    pub access_control_list: AccessControlList,
    /// Server-side encryption configuration.
    ///
    /// 服务端加密配置。
    pub server_side_encryption_rule: ServerSideEncryptionRule,
    /// Logging configuration.
    ///
    /// 日志配置。
    pub bucket_policy: BucketPolicy,
}

/// Bucket access control information.
///
/// Bucket 访问控制信息。
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AccessControlList {
    /// ACL grant.
    ///
    /// ACL 授权。
    pub grant: Acl,
}

/// Bucket server-side encryption information.
///
/// Bucket 服务端加密信息。
#[derive(Debug, Deserialize)]
pub struct ServerSideEncryptionRule {
    /// Default server-side encryption algorithm.
    ///
    /// 默认服务端加密算法。
    #[serde(rename = "SSEAlgorithm")]
    pub sse_algorithm: String,
}

/// Bucket logging information.
///
/// Bucket 日志信息。
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BucketPolicy {
    /// Bucket name that stores logs.
    ///
    /// 存储日志的 Bucket 名称。
    pub log_bucket: String,
    /// Prefix for log objects.
    ///
    /// 日志对象前缀。
    pub log_prefix: String,
}

/// Retrieve detailed bucket information.
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/document_detail/31968.html) for details.
///
/// 获取 Bucket 详细信息。
///
/// 详情参见 [阿里云文档](https://help.aliyun.com/document_detail/31968.html)。
pub struct GetBucketInfo {
    req: OssRequest,
}
impl GetBucketInfo {
    pub(super) fn new(oss: Oss) -> Self {
        let mut req = OssRequest::new(oss, Method::GET);
        req.insert_query("bucketInfo", "");
        GetBucketInfo { req }
    }
    /// Send the request and return bucket info.
    ///
    /// 发送请求并返回 Bucket 信息。
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
