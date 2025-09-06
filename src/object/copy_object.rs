use crate::{
    common::{Acl, StorageClass, format_gmt, invalid_metadata_key, url_encode},
    error::{Error, normal_error},
    request::{Oss, OssRequest},
};
use http::Method;
use std::collections::HashMap;
use time::OffsetDateTime;

/// Copy an object
///
/// Copy within the same bucket is limited to 5GB; copying across buckets is limited to 1GB
///
/// There are many additional restrictions; see the [Alibaba Cloud documentation](https://help.aliyun.com/document_detail/31979.html) for details
pub struct CopyObject {
    req: OssRequest,
    tags: HashMap<String, String>,
}

impl CopyObject {
    pub(super) fn new(oss: Oss, copy_source: impl ToString) -> Self {
        let mut req = OssRequest::new(oss, Method::PUT);
        req.insert_header("x-oss-copy-source", copy_source);
        CopyObject {
            req,
            tags: HashMap::new(),
        }
    }
    /// Set the object's access permissions
    pub fn set_acl(mut self, acl: Acl) -> Self {
        self.req.insert_header("x-oss-object-acl", acl);
        self
    }
    /// Set the object's storage class
    pub fn set_storage_class(mut self, storage_class: StorageClass) -> Self {
        self.req.insert_header("x-oss-storage-class", storage_class);
        self
    }
    /// Set additional metadata
    ///
    /// Keys may only contain letters, numbers, and hyphens; metadata with other characters will be discarded
    pub fn set_meta(mut self, key: impl ToString, value: impl ToString) -> Self {
        let key = key.to_string();
        if !invalid_metadata_key(&key) {
            self.req
                .insert_header(format!("x-oss-meta-{}", key.to_string()), value);
        }
        self
    }
    /// If the specified time is earlier than the file's actual modification time, the copy proceeds.
    ///
    pub fn set_if_modified_since(mut self, if_modified_since: OffsetDateTime) -> Self {
        self.req.insert_header(
            "x-oss-copy-source-if-modified-since",
            format_gmt(if_modified_since),
        );
        self
    }
    /// If the specified time is equal to or later than the file's actual modification time, the copy proceeds.
    ///
    pub fn set_if_unmodified_since(mut self, if_unmodified_since: OffsetDateTime) -> Self {
        self.req.insert_header(
            "x-oss-copy-source-if-unmodified-since",
            format_gmt(if_unmodified_since),
        );
        self
    }
    /// Copy the source object only if its ETag matches the value you provide.
    ///
    /// The ETag is used to verify whether the data has changed; you can use it to check data integrity.
    pub fn set_if_match(mut self, if_match: impl ToString) -> Self {
        self.req
            .insert_header("x-oss-copy-source-if-match", if_match);
        self
    }
    /// Copy the source object only if its ETag does not match the value you provide.
    ///
    /// The ETag is used to verify whether the data has changed; you can use it to check data integrity.
    pub fn set_if_none_match(mut self, if_none_match: impl ToString) -> Self {
        self.req
            .insert_header("x-oss-copy-source-if-none-match", if_none_match);
        self
    }
    /// Disallow overwriting files with the same name
    pub fn forbid_overwrite(mut self) -> Self {
        self.req.insert_header("x-oss-forbid-overwrite", "true");
        self
    }
    /// Set tag information
    pub fn set_tagging(mut self, key: impl ToString, value: impl ToString) -> Self {
        self.tags.insert(key.to_string(), value.to_string());
        self
    }
    /// Use the metadata specified in the request and ignore the source object's metadata
    pub fn set_metadata_directive(mut self) -> Self {
        self.req
            .insert_header("x-oss-metadata-directive", "REPLACE");
        self
    }
    /// Use the tags specified in the request and ignore the source object's tags
    pub fn set_tagging_directive(mut self) -> Self {
        self.req.insert_header("x-oss-tagging-directive", "Replace");
        self
    }

    /// Copy the object
    ///
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
