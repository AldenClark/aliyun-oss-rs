use crate::common::body_to_bytes_sync;
use crate::{
    Error,
    common::StorageClass,
    error::normal_error_sync,
    request_sync::{Oss, OssRequest},
};
use http::Method;
use serde_derive::Deserialize;

// Return value
/// Basic bucket information.
///
/// Bucket 基本信息。
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BucketBase {
    /// Bucket name.
    ///
    /// Bucket 名称。
    pub name: String,
    /// Region.
    ///
    /// 地域。
    pub region: String,
    /// Region identifier in OSS.
    ///
    /// OSS 地域标识。
    pub location: String,
    /// Public endpoint.
    ///
    /// 外网 Endpoint。
    pub extranet_endpoint: String,
    /// Internal endpoint.
    ///
    /// 内网 Endpoint。
    pub intranet_endpoint: String,
    /// Storage class.
    ///
    /// 存储类型。
    pub storage_class: StorageClass,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct Buckets {
    pub bucket: Option<Vec<BucketBase>>,
}

// Result set of bucket list query
/// Internal bucket list payload.
///
/// 内部 Bucket 列表载荷。
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct ListAllMyBucketsResult {
    /// Marker for the next query when results are truncated.
    ///
    /// 结果被截断时的下一次查询 marker。
    pub next_marker: Option<String>,
    /// Bucket list.
    ///
    /// Bucket 列表。
    pub buckets: Buckets,
}

/// Result set of bucket list query.
///
/// Bucket 列表查询结果。
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListAllMyBuckets {
    /// Marker for the next query when results are truncated.
    ///
    /// 结果被截断时的下一次查询 marker。
    pub next_marker: Option<String>,
    /// Bucket list.
    ///
    /// Bucket 列表。
    pub buckets: Option<Vec<BucketBase>>,
}

/// List buckets.
///
/// Filters can be set via the `set_` methods. See the [Alibaba Cloud documentation](https://help.aliyun.com/document_detail/31957.html) for details.
///
/// ```ignore
/// let client = OssClient::new("AccessKey ID", "AccessKey Secret", "cn-beijing");
/// let buckets = client.list_buckets().set_prefix("rust").send();
/// println!("{:#?}", buckets);
/// ```
///
/// 列举 Bucket。
///
/// 可通过 `set_` 方法设置过滤条件；详情见 [阿里云文档](https://help.aliyun.com/document_detail/31957.html)。
///
/// ```ignore
/// let client = OssClient::new("AccessKey ID", "AccessKey Secret", "cn-beijing");
/// let buckets = client.list_buckets().set_prefix("rust").send();
/// println!("{:#?}", buckets);
/// ```
pub struct ListBucketsSync {
    req: OssRequest,
}

impl ListBucketsSync {
    pub(super) fn new(oss: Oss) -> Self {
        ListBucketsSync { req: OssRequest::new(oss, Method::GET) }
    }

    /// Limit bucket names to those starting with the given prefix.
    ///
    /// Prefix requirements:
    /// - Cannot be empty and must not exceed 63 bytes
    /// - May contain only lowercase letters, numbers, and hyphens, and cannot start with a hyphen
    ///
    /// 仅返回以指定前缀开头的 Bucket。
    ///
    /// Prefix requirements:
    /// - Cannot be empty and must not exceed 63 bytes
    /// - May contain only lowercase letters, numbers, and hyphens, and cannot start with a hyphen
    pub fn set_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.req.insert_query("prefix", prefix.into());
        self
    }
    /// Start returning results after the given marker.
    ///
    /// 从指定 marker 之后开始返回结果。
    pub fn set_marker(mut self, marker: impl Into<String>) -> Self {
        self.req.insert_query("marker", marker.into());
        self
    }
    /// Set maximum number of buckets to return (1-1000, default 100).
    ///
    /// 设置返回 Bucket 的最大数量（1-1000，默认 100）。
    pub fn set_max_keys(mut self, max_keys: u32) -> Self {
        self.req.insert_query("max-keys", max_keys.to_string());
        self
    }
    /// Specify the resource group ID.
    ///
    /// 指定资源组 ID。
    pub fn set_group_id(mut self, group_id: impl Into<String>) -> Self {
        self.req.insert_header("x-oss-resource-group-id", group_id.into());
        self
    }
    /// Set the endpoint used for this query; it does not limit the bucket regions.
    ///
    /// Defaults to oss.aliyuncs.com. If unreachable, use an accessible endpoint.
    ///
    /// 设置本次查询的 Endpoint；不会限制 Bucket 所在地域。
    ///
    /// 默认 oss.aliyuncs.com，不可达时请设置可访问的 Endpoint。
    pub fn set_endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.req.set_endpoint(endpoint.into());
        self
    }
    /// Send the request.
    ///
    /// 发送请求。
    pub fn send(self) -> Result<ListAllMyBuckets, Error> {
        // Build the HTTP request
        let response = self.req.send_to_oss()?;
        // Parse the response
        let status_code = response.status();
        match status_code {
            code if code.is_success() => {
                let response_bytes =
                    body_to_bytes_sync(response.into_body()).map_err(|_| Error::OssInvalidResponse(None))?;
                let result: ListAllMyBucketsResult = serde_xml_rs::from_reader(&*response_bytes)
                    .map_err(|_| Error::OssInvalidResponse(Some(response_bytes)))?;
                Ok(ListAllMyBuckets { next_marker: result.next_marker, buckets: result.buckets.bucket })
            }
            _ => Err(normal_error_sync(response)),
        }
    }
}
