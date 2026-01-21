use crate::common::body_to_bytes_sync;
use crate::{
    Error,
    common::{Owner, StorageClass},
    error::normal_error_sync,
    request_sync::{Oss, OssRequest},
};
use http::Method;
use serde_derive::Deserialize;
use std::cmp;

use super::list_objects_sync::CommonPrefixes;

/// List object versions in a bucket.
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/zh/oss/developer-reference/listobjectversions) for details.
///
/// 列举 Bucket 内对象版本。
///
/// 详情见 [阿里云文档](https://help.aliyun.com/zh/oss/developer-reference/listobjectversions)。
pub struct ListObjectVersionsSync {
    req: OssRequest,
}

/// Response payload for ListObjectVersionsSync.
///
/// ListObjectVersionsSync 的响应结果。
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListObjectVersionsResult {
    pub name: String,
    pub prefix: Option<String>,
    pub key_marker: Option<String>,
    pub version_id_marker: Option<String>,
    pub next_key_marker: Option<String>,
    pub next_version_id_marker: Option<String>,
    pub max_keys: Option<u32>,
    pub delimiter: Option<String>,
    pub is_truncated: bool,
    #[serde(rename = "Version", default)]
    pub versions: Vec<ObjectVersion>,
    #[serde(rename = "DeleteMarker", default)]
    pub delete_markers: Vec<DeleteMarker>,
    #[serde(rename = "CommonPrefixes", default)]
    pub common_prefixes: Vec<CommonPrefixes>,
}

/// Object version entry.
///
/// 对象版本条目。
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ObjectVersion {
    pub key: String,
    pub version_id: String,
    pub is_latest: bool,
    pub last_modified: String,
    pub e_tag: String,
    #[serde(rename = "Type")]
    pub type_field: Option<String>,
    pub size: Option<u64>,
    pub storage_class: Option<StorageClass>,
    pub owner: Option<Owner>,
}

/// Delete marker entry.
///
/// 删除标记条目。
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteMarker {
    pub key: String,
    pub version_id: String,
    pub is_latest: bool,
    pub last_modified: String,
    pub owner: Option<Owner>,
}

impl ListObjectVersionsSync {
    pub(super) fn new(oss: Oss) -> Self {
        let mut req = OssRequest::new(oss, Method::GET);
        req.insert_query("versions", "");
        req.insert_query("max-keys", "1000");
        ListObjectVersionsSync { req }
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

    /// Set the key marker for pagination.
    ///
    /// 设置分页的 key marker。
    pub fn set_key_marker(mut self, key_marker: impl Into<String>) -> Self {
        self.req.insert_query("key-marker", key_marker.into());
        self
    }

    /// Set the version ID marker for pagination.
    ///
    /// 设置分页的 version ID marker。
    pub fn set_version_id_marker(mut self, version_id_marker: impl Into<String>) -> Self {
        self.req
            .insert_query("version-id-marker", version_id_marker.into());
        self
    }

    /// Set the maximum number of entries to return.
    ///
    /// 设置返回条目上限。
    pub fn set_max_keys(mut self, max_keys: u32) -> Self {
        let max_keys = cmp::min(1000, cmp::max(1, max_keys));
        self.req
            .insert_query("max-keys", max_keys.to_string());
        self
    }

    /// Send the request.
    ///
    /// 发送请求。
    pub fn send(self) -> Result<ListObjectVersionsResult, Error> {
        let response = self.req.send_to_oss()?;
        match response.status() {
            code if code.is_success() => {
                let response_bytes = body_to_bytes_sync(response.into_body())
                    
                    .map_err(|_| Error::OssInvalidResponse(None))?;
                let result: ListObjectVersionsResult =
                    serde_xml_rs::from_reader(response_bytes.as_ref())
                        .map_err(|_| Error::OssInvalidResponse(Some(response_bytes)))?;
                Ok(result)
            }
            _ => Err(normal_error_sync(response)),
        }
    }
}
