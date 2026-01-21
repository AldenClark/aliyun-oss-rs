use crate::common::body_to_bytes;
use crate::{
    error::{Error, normal_error},
    request::{Oss, OssRequest},
};
use http::Method;
use serde_derive::Deserialize;
use std::cmp;

// Returned content
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
/// Result of listing multipart uploads.
///
/// 列举分片上传结果。
pub struct ListMultipartUploadsResult {
    /// Whether the result is truncated.
    ///
    /// 是否被截断。
    pub is_truncated: bool,
    /// Next key marker.
    ///
    /// 下一次列举的 key-marker。
    pub next_key_marker: String,
    /// Next upload ID marker.
    ///
    /// 下一次列举的 upload-id-marker。
    pub next_upload_id_marker: String,
    /// Upload entries.
    ///
    /// 上传条目。
    pub upload: Option<Vec<Upload>>,
    /// Common prefixes.
    ///
    /// 共同前缀。
    pub common_prefixes: Option<Vec<CommonPrefixes>>,
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
/// Multipart upload entry.
///
/// 分片上传条目。
pub struct Upload {
    /// Object key.
    ///
    /// 对象 Key。
    pub key: String,
    /// Upload ID.
    ///
    /// 上传 ID。
    pub upload_id: String,
    /// Storage class.
    ///
    /// 存储类型。
    pub storage_class: String,
    /// Initiated time.
    ///
    /// 发起时间。
    pub initiated: String,
}

/// List multipart uploads that are initiated but not completed or aborted.
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/document_detail/31997.html) for details.
///
/// 列举已发起但未完成或未取消的分片上传。
///
/// 详情参见 [阿里云文档](https://help.aliyun.com/document_detail/31997.html)。
pub struct ListUploads {
    req: OssRequest,
}

impl ListUploads {
    pub(super) fn new(oss: Oss) -> Self {
        let mut req = OssRequest::new(oss, Method::GET);
        req.insert_query("uploads", "");
        ListUploads { req }
    }
    /// Group object keys by delimiter (populate `CommonPrefixes`).
    ///
    /// 使用分隔符对对象 Key 分组（填充 `CommonPrefixes`）。
    pub fn set_delimiter(mut self, delimiter: impl Into<String>) -> Self {
        self.req.insert_query("delimiter", delimiter.into());
        self
    }
    /// Restrict results to keys with the given prefix.
    ///
    /// 限定返回指定前缀的对象。
    pub fn set_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.req.insert_query("prefix", prefix.into());
        self
    }
    /// Set the key marker.
    ///
    /// 设置 key-marker。
    pub fn set_key_marker(mut self, key_marker: impl Into<String>) -> Self {
        self.req.insert_query("key-marker", key_marker.into());
        self
    }
    /// Set the upload ID marker.
    ///
    /// 设置 upload-id-marker。
    pub fn set_upload_id_marker(mut self, upload_id_marker: impl Into<String>) -> Self {
        self.req
            .insert_query("upload-id-marker", upload_id_marker.into());
        self
    }
    /// Limit the maximum number of uploads returned.
    ///
    /// When a delimiter is set, this counts both objects and groups.
    ///
    /// Default is 1000, valid range is 1-1000.
    ///
    /// 限制返回的分片上传数量。
    ///
    /// 设置分隔符时同时计入对象与分组。
    ///
    /// 默认 1000，合法范围 1-1000。
    pub fn set_max_uploads(mut self, max_keys: u32) -> Self {
        let max_keys = cmp::min(1000, cmp::max(1, max_keys));
        self.req
            .insert_query("max-uploads", max_keys.to_string());
        self
    }
    /// Send the request and return results.
    ///
    /// 发送请求并返回结果。
    pub async fn send(self) -> Result<ListMultipartUploadsResult, Error> {
        // Upload file
        let response = self.req.send_to_oss()?.await?;
        // Parse the response
        let status_code = response.status();
        match status_code {
            code if code.is_success() => {
                let response_bytes = body_to_bytes(response.into_body())
                    .await
                    .map_err(|_| Error::OssInvalidResponse(None))?;
                let result: ListMultipartUploadsResult =
                    serde_xml_rs::from_reader(&*response_bytes)
                        .map_err(|_| Error::OssInvalidResponse(Some(response_bytes)))?;
                Ok(result)
            }
            _ => Err(normal_error(response).await),
        }
    }
}
