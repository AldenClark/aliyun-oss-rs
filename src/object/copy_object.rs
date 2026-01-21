use crate::{
    common::{Acl, StorageClass, format_gmt, invalid_metadata_key, url_encode},
    error::{Error, normal_error},
    request::{Oss, OssRequest},
};
use http::Method;
use std::collections::HashMap;
use time::OffsetDateTime;

/// Copy an object.
///
/// Same-bucket copy is limited to 5GB; cross-bucket copy is limited to 1GB.
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/document_detail/31979.html) for details and restrictions.
///
/// 复制对象。
///
/// 同桶复制上限 5GB，跨桶复制上限 1GB。
///
/// 详细限制请参阅 [阿里云文档](https://help.aliyun.com/document_detail/31979.html)。
pub struct CopyObject {
    req: OssRequest,
    tags: HashMap<String, String>,
}

impl CopyObject {
    pub(super) fn new(oss: Oss, copy_source: impl Into<String>) -> Self {
        let mut req = OssRequest::new(oss, Method::PUT);
        req.insert_header("x-oss-copy-source", copy_source.into());
        CopyObject {
            req,
            tags: HashMap::new(),
        }
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
    /// Proceed only if the source is modified after the given time.
    ///
    /// 仅当源对象在指定时间之后被修改时才复制。
    pub fn set_if_modified_since(mut self, if_modified_since: OffsetDateTime) -> Self {
        self.req.insert_header(
            "x-oss-copy-source-if-modified-since",
            format_gmt(if_modified_since),
        );
        self
    }
    /// Proceed only if the source is not modified after the given time.
    ///
    /// 仅当源对象在指定时间之后未被修改时才复制。
    pub fn set_if_unmodified_since(mut self, if_unmodified_since: OffsetDateTime) -> Self {
        self.req.insert_header(
            "x-oss-copy-source-if-unmodified-since",
            format_gmt(if_unmodified_since),
        );
        self
    }
    /// Copy only if the source ETag matches the given value.
    ///
    /// ETag helps detect data changes and verify integrity.
    ///
    /// 仅当源对象 ETag 与给定值一致时才复制。
    ///
    /// ETag 可用于检测数据变更和校验完整性。
    pub fn set_if_match(mut self, if_match: impl Into<String>) -> Self {
        self.req
            .insert_header("x-oss-copy-source-if-match", if_match.into());
        self
    }
    /// Copy only if the source ETag does not match the given value.
    ///
    /// ETag helps detect data changes and verify integrity.
    ///
    /// 仅当源对象 ETag 与给定值不一致时才复制。
    ///
    /// ETag 可用于检测数据变更和校验完整性。
    pub fn set_if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
        self.req
            .insert_header("x-oss-copy-source-if-none-match", if_none_match.into());
        self
    }
    /// Disallow overwriting objects with the same key.
    ///
    /// 禁止覆盖同名对象。
    pub fn forbid_overwrite(mut self) -> Self {
        self.req.insert_header("x-oss-forbid-overwrite", "true");
        self
    }
    /// Add a tag key/value pair.
    ///
    /// 追加对象标签键值对。
    pub fn set_tagging(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.tags.insert(key.into(), value.into());
        self
    }
    /// Use metadata from this request, ignoring source metadata.
    ///
    /// 使用本请求的元数据，忽略源对象元数据。
    pub fn set_metadata_directive(mut self) -> Self {
        self.req
            .insert_header("x-oss-metadata-directive", "REPLACE");
        self
    }
    /// Use tags from this request, ignoring source tags.
    ///
    /// 使用本请求的标签，忽略源对象标签。
    pub fn set_tagging_directive(mut self) -> Self {
        self.req.insert_header("x-oss-tagging-directive", "Replace");
        self
    }

    /// Send the copy request.
    ///
    /// 发送复制请求。
    pub async fn send(mut self) -> Result<(), Error> {
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
        // Build the HTTP request
        let response = self.req.send_to_oss()?.await?;
        // Parse the response
        let status_code = response.status();
        match status_code {
            code if code.is_success() => Ok(()),
            _ => Err(normal_error(response).await),
        }
    }
}
