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

use super::list_objects::CommonPrefixes;

/// List objects in a bucket using the legacy ListObjects API.
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/zh/oss/developer-reference/listobjects) for details.
///
/// 使用旧版 ListObjects API 列举对象。
///
/// 详情见 [阿里云文档](https://help.aliyun.com/zh/oss/developer-reference/listobjects)。
pub struct ListObjectsV1 {
    req: OssRequest,
}

/// Response payload for ListObjects (v1).
///
/// ListObjects（v1）的响应结果。
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListObjectsV1Result {
    pub name: String,
    pub prefix: Option<String>,
    pub marker: Option<String>,
    pub next_marker: Option<String>,
    pub max_keys: Option<u32>,
    pub delimiter: Option<String>,
    pub is_truncated: bool,
    #[serde(rename = "Contents", default)]
    pub contents: Vec<ObjectInfo>,
    #[serde(rename = "CommonPrefixes", default)]
    pub common_prefixes: Vec<CommonPrefixes>,
}

/// Object entry.
///
/// 对象条目。
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ObjectInfo {
    pub key: String,
    pub last_modified: String,
    pub e_tag: String,
    #[serde(rename = "Type")]
    pub type_field: String,
    pub size: u64,
    pub storage_class: StorageClass,
    pub owner: Option<Owner>,
}

impl ListObjectsV1 {
    pub(super) fn new(oss: Oss) -> Self {
        let mut req = OssRequest::new(oss, Method::GET);
        req.insert_query("max-keys", "1000");
        ListObjectsV1 { req }
    }

    /// Restrict returned keys to the given prefix.
    ///
    /// 仅返回指定前缀的对象。
    pub fn set_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.req.insert_query("prefix", prefix.into());
        self
    }

    /// Group objects by delimiter.
    ///
    /// 通过分隔符分组对象。
    pub fn set_delimiter(mut self, delimiter: impl Into<String>) -> Self {
        self.req.insert_query("delimiter", delimiter.into());
        self
    }

    /// Set the marker for pagination.
    ///
    /// 设置分页 marker。
    pub fn set_marker(mut self, marker: impl Into<String>) -> Self {
        self.req.insert_query("marker", marker.into());
        self
    }

    /// Set the maximum number of entries to return.
    ///
    /// 设置返回条目上限。
    pub fn set_max_keys(mut self, max_keys: u32) -> Self {
        let max_keys = cmp::min(1000, cmp::max(1, max_keys));
        self.req.insert_query("max-keys", max_keys.to_string());
        self
    }

    /// Send the request.
    ///
    /// 发送请求。
    pub async fn send(self) -> Result<ListObjectsV1Result, Error> {
        let response = self.req.send_to_oss()?.await?;
        match response.status() {
            code if code.is_success() => {
                let response_bytes =
                    body_to_bytes(response.into_body()).await.map_err(|_| Error::OssInvalidResponse(None))?;
                let result: ListObjectsV1Result = serde_xml_rs::from_reader(response_bytes.as_ref())
                    .map_err(|_| Error::OssInvalidResponse(Some(response_bytes)))?;
                Ok(result)
            }
            _ => Err(normal_error(response).await),
        }
    }
}
