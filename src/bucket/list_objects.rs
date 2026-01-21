use crate::common::body_to_bytes;
use crate::{
    Error,
    common::{Owner, StorageClass},
    error::normal_error,
    request::{Oss, OssRequest},
};
use http::Method;
use serde_derive::Deserialize;
use std::cmp;

// Returned content
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
/// Result of listing objects (ListObjectsV2).
///
/// 列举对象结果（ListObjectsV2）。
pub struct ObjectsList {
    /// Continuation token for subsequent requests.
    ///
    /// 下一次列举的 continuation token。
    pub next_continuation_token: Option<String>,
    /// Object list.
    ///
    /// 对象列表。
    pub contents: Option<Vec<ObjectInfo>>,
    /// Common prefixes.
    ///
    /// 共同前缀。
    pub common_prefixes: Option<Vec<CommonPrefixes>>,
}

/// Object information.
///
/// 对象信息。
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ObjectInfo {
    /// Object key.
    ///
    /// 对象 Key。
    pub key: String,
    /// Last modified time.
    ///
    /// 最近修改时间。
    pub last_modified: String,
    /// ETag of the object (not a reliable MD5 checksum).
    ///
    /// 对象 ETag（不建议作为 MD5 校验）。
    pub e_tag: String,
    #[serde(rename = "Type")]
    pub type_field: String,
    /// Object size in bytes.
    ///
    /// 对象大小（字节）。
    pub size: u64,
    /// Storage class.
    ///
    /// 存储类型。
    pub storage_class: StorageClass,
    /// Restore status if applicable.
    ///
    /// 解冻状态（如适用）。
    pub restore_info: Option<String>,
    /// Bucket owner information.
    ///
    /// Bucket 所有者信息。
    pub owner: Option<Owner>,
}

/// Group prefixes for listing results.
///
/// 列举结果的分组前缀。
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CommonPrefixes {
    /// Prefix value.
    ///
    /// 前缀值。
    pub prefix: String,
}

/// List objects in a bucket (ListObjectsV2).
///
/// By default, retrieves the first 1000 objects.
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/document_detail/187544.html) for details.
///
/// 列举 Bucket 中的对象（ListObjectsV2）。
///
/// 默认获取前 1000 个对象。
///
/// 详情参见 [阿里云文档](https://help.aliyun.com/document_detail/187544.html)。
pub struct ListObjects {
    req: OssRequest,
}

impl ListObjects {
    pub(super) fn new(oss: Oss) -> Self {
        let mut req = OssRequest::new(oss, Method::GET);
        req.insert_query("list-type", "2");
        req.insert_query("max-keys", "1000");
        ListObjects { req }
    }
    /// Group object keys by delimiter (populate `CommonPrefixes`).
    ///
    /// 使用分隔符对对象 Key 分组（填充 `CommonPrefixes`）。
    pub fn set_delimiter(mut self, delimiter: impl Into<String>) -> Self {
        self.req.insert_query("delimiter", delimiter.into());
        self
    }
    /// Specify the starting point by key (start-after).
    ///
    /// `start-after` is used for pagination and must be less than 1024 bytes.
    ///
    /// If the key does not exist, listing starts from the next key in order.
    ///
    /// 指定起始对象 Key（start-after）。
    ///
    /// `start-after` 用于分页，长度需小于 1024 字节。
    ///
    /// 若该 Key 不存在，将从其后续 Key 开始列举。
    pub fn set_start_after(mut self, start_after: impl Into<String>) -> Self {
        self.req.insert_query("start-after", start_after.into());
        self
    }
    /// Set the continuation token returned by a previous call.
    ///
    /// The token is from `NextContinuationToken`.
    ///
    /// 设置上次请求返回的 continuation token。
    ///
    /// Token 来源于 `NextContinuationToken`。
    pub fn set_continuation_token(mut self, continuation_token: impl Into<String>) -> Self {
        self.req.insert_query("continuation-token", continuation_token.into());
        self
    }
    /// Restrict results to keys with the given prefix.
    ///
    /// 限定返回指定前缀的对象。
    pub fn set_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.req.insert_query("prefix", prefix.into());
        self
    }
    /// Specify the maximum number of objects to return.
    ///
    /// When a delimiter is set, this counts both objects and groups.
    ///
    /// Default is 1000, valid range is 1-1000.
    ///
    /// 指定返回的最大对象数量。
    ///
    /// 设置分隔符时同时计入对象与分组。
    ///
    /// 默认 1000，合法范围 1-1000。
    pub fn set_max_keys(mut self, max_keys: u32) -> Self {
        let max_keys = cmp::min(1000, cmp::max(1, max_keys));
        self.req.insert_query("max-keys", max_keys.to_string());
        self
    }
    /// Include owner information in results.
    ///
    /// 在结果中包含所有者信息。
    pub fn fetch_owner(mut self) -> Self {
        self.req.insert_query("fetch-owner", "true");
        self
    }
    /// Send the request and return results.
    ///
    /// 发送请求并返回结果。
    pub async fn send(self) -> Result<ObjectsList, Error> {
        // Build the HTTP request
        let response = self.req.send_to_oss()?.await?;
        // Parse the response
        let status_code = response.status();
        match status_code {
            code if code.is_success() => {
                let response_bytes =
                    body_to_bytes(response.into_body()).await.map_err(|_| Error::OssInvalidResponse(None))?;
                let object_list: ObjectsList = serde_xml_rs::from_reader(&*response_bytes)
                    .map_err(|_| Error::OssInvalidResponse(Some(response_bytes)))?;
                Ok(object_list)
            }
            _ => return Err(normal_error(response).await),
        }
    }
}
