use crate::common::body_to_bytes_sync;
use crate::{
    common::{
        Acl, CacheControl, ContentDisposition, StorageClass, invalid_metadata_key, url_encode,
    },
    error::{Error, normal_error_sync},
    request_sync::{Oss, OssRequest},
};
use http::{Method, header};
use serde_derive::Deserialize;
use std::collections::HashMap;

// Returned content
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct InitiateMultipartUploadResult {
    upload_id: String,
}

/// Initiate a multipart upload (sync).
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/document_detail/31992.html) for details.
///
/// 初始化分片上传（同步）。
///
/// 详情参见 [阿里云文档](https://help.aliyun.com/document_detail/31992.html)。
pub struct InitUploadSync {
    req: OssRequest,
    tags: HashMap<String, String>,
}
impl InitUploadSync {
    pub(super) fn new(oss: Oss) -> Self {
        let mut req = OssRequest::new(oss, Method::POST);
        req.insert_query("uploads", "");
        InitUploadSync {
            req,
            tags: HashMap::new(),
        }
    }
    /// Set the object's MIME type.
    ///
    /// If not set, fallback is `application/octet-stream`.
    ///
    /// 设置对象的 MIME 类型。
    ///
    /// 未设置时默认使用 `application/octet-stream`。
    pub fn set_mime(mut self, mime: impl Into<String>) -> Self {
        self.req
            .insert_header(header::CONTENT_TYPE.as_str(), mime.into());
        self
    }
    /// Set object ACL.
    ///
    /// 设置对象 ACL。
    pub fn set_acl(mut self, acl: Acl) -> Self {
        self.req
            .insert_header("x-oss-object-acl", acl.to_string());
        self
    }
    /// Set object storage class.
    ///
    /// 设置对象存储类型。
    pub fn set_storage_class(mut self, storage_class: StorageClass) -> Self {
        self.req.insert_header(
            "x-oss-storage-class",
            storage_class.to_string(),
        );
        self
    }
    /// Set cache-control behavior when the object is downloaded.
    ///
    /// 设置对象下载时的缓存策略。
    pub fn set_cache_control(mut self, cache_control: CacheControl) -> Self {
        self.req
            .insert_header(header::CACHE_CONTROL.as_str(), cache_control.to_string());
        self
    }
    /// Set content disposition for downloads.
    ///
    /// 设置下载时的内容呈现方式。
    pub fn set_content_disposition(mut self, content_disposition: ContentDisposition) -> Self {
        self.req.insert_header(
            header::CONTENT_DISPOSITION.as_str(),
            content_disposition.to_string(),
        );
        self
    }
    /// Disallow overwriting objects with the same key.
    ///
    /// 禁止覆盖同名对象。
    pub fn forbid_overwrite(mut self) -> Self {
        self.req.insert_header("x-oss-forbid-overwrite", "true");
        self
    }
    /// Set custom object metadata.
    ///
    /// Metadata keys may only contain letters, numbers, and hyphens.
    ///
    /// 设置对象自定义元数据。
    ///
    /// 元数据键仅允许字母、数字和连字符。
    pub fn set_meta(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        let key = key.into();
        if !invalid_metadata_key(&key) {
            self.req
                .insert_header(format!("x-oss-meta-{}", key), value.into());
        }
        self
    }
    /// Add a tag key/value pair.
    ///
    /// 追加对象标签键值对。
    pub fn set_tagging(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.tags.insert(key.into(), value.into());
        self
    }
    /// Send the request and return the upload ID.
    ///
    /// 发送请求并返回上传 ID。
    pub fn send(mut self) -> Result<String, Error> {
        // Insert tags
        let tags = self
            .tags
            .into_iter()
            .map(|(key, value)| {
                if value.is_empty() {
                    url_encode(&key.to_string())
                } else {
                    format!(
                        "{}={}",
                        url_encode(&key.to_string()),
                        url_encode(&value.to_string())
                    )
                }
            })
            .collect::<Vec<_>>()
            .join("&");
        if !tags.is_empty() {
            self.req.insert_header("x-oss-tagging", tags);
        }
        // Upload file
        let response = self.req.send_to_oss()?;
        // Parse the response
        let status_code = response.status();
        match status_code {
            code if code.is_success() => {
                let response_bytes = body_to_bytes_sync(response.into_body())
                    .map_err(|_| Error::OssInvalidResponse(None))?;
                let result: InitiateMultipartUploadResult =
                    serde_xml_rs::from_reader(&*response_bytes)
                        .map_err(|_| Error::OssInvalidResponse(Some(response_bytes)))?;
                Ok(result.upload_id)
            }
            _ => Err(normal_error_sync(response)),
        }
    }
}
