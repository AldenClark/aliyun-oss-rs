use crate::{
    common::{
        invalid_metadata_key, url_encode, Acl, CacheControl, ContentDisposition, StorageClass,
    },
    error::{normal_error, Error},
    request::{Oss, OssRequest},
};
use http::{header, Method};
use crate::common::body_to_bytes;
use serde_derive::Deserialize;
use std::collections::HashMap;

// Returned content
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct InitiateMultipartUploadResult {
    upload_id: String,
}

/// Initiate a multipart upload
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/document_detail/31992.html) for details
pub struct InitUpload {
    req: OssRequest,
    tags: HashMap<String, String>,
}
impl InitUpload {
    pub(super) fn new(oss: Oss) -> Self {
        let mut req = OssRequest::new(oss, Method::POST);
        req.insert_query("uploads", "");
        InitUpload {
            req,
            tags: HashMap::new(),
        }
    }
    /// Set the file's MIME type
    ///
    /// If no MIME type is set, the default (application/octet-stream) is used
    pub fn set_mime(mut self, mime: impl ToString) -> Self {
        self.req.insert_header(header::CONTENT_TYPE, mime);
        self
    }
    /// Set the file's access permissions
    pub fn set_acl(mut self, acl: Acl) -> Self {
        self.req.insert_header("x-oss-object-acl", acl);
        self
    }
    /// Set the file's storage class
    pub fn set_storage_class(mut self, storage_class: StorageClass) -> Self {
        self.req.insert_header("x-oss-storage-class", storage_class);
        self
    }
    /// Cache behavior of the webpage when the file is downloaded
    pub fn set_cache_control(mut self, cache_control: CacheControl) -> Self {
        self.req.insert_header(header::CACHE_CONTROL, cache_control);
        self
    }
    /// Set how the file is presented
    pub fn set_content_disposition(mut self, content_disposition: ContentDisposition) -> Self {
        self.req
            .insert_header(header::CONTENT_DISPOSITION, content_disposition);
        self
    }
    /// Disallow overwriting files with the same name
    pub fn forbid_overwrite(mut self) -> Self {
        self.req.insert_header("x-oss-forbid-overwrite", "true");
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
    /// Set tag information
    pub fn set_tagging(mut self, key: impl ToString, value: impl ToString) -> Self {
        self.tags.insert(key.to_string(), value.to_string());
        self
    }
    /// Send the request
    pub async fn send(mut self) -> Result<String, Error> {
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
        let response = self.req.send_to_oss()?.await?;
        // Parse the response
        let status_code = response.status();
        match status_code {
            code if code.is_success() => {
                let response_bytes = body_to_bytes(response.into_body())
                    .await
                    .map_err(|_| Error::OssInvalidResponse(None))?;
                let result: InitiateMultipartUploadResult =
                    serde_xml_rs::from_reader(&*response_bytes)
                        .map_err(|_| Error::OssInvalidResponse(Some(response_bytes)))?;
                Ok(result.upload_id)
            }
            _ => Err(normal_error(response).await),
        }
    }
}
