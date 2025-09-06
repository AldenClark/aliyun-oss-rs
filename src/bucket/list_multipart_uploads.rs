use crate::{
    error::{normal_error, Error},
    request::{Oss, OssRequest},
};
use http::Method;
use crate::common::body_to_bytes;
use serde_derive::Deserialize;
use std::cmp;

// Returned content
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListMultipartUploadsResult {
    pub is_truncated: bool,
    pub next_key_marker: String,
    pub next_upload_id_marker: String,
    pub upload: Option<Vec<Upload>>,
    pub common_prefixes: Option<Vec<CommonPrefixes>>,
}

/// Group list
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CommonPrefixes {
    /// Prefix
    pub prefix: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Upload {
    pub key: String,
    pub upload_id: String,
    pub storage_class: String,
    pub initiated: String,
}

/// List all ongoing Multipart Upload events that have been initiated but not yet completed or aborted
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/document_detail/31997.html) for details
pub struct ListUploads {
    req: OssRequest,
}

impl ListUploads {
    pub(super) fn new(oss: Oss) -> Self {
        let mut req = OssRequest::new(oss, Method::GET);
        req.insert_query("uploads", "");
        ListUploads { req }
    }
    /// Character used to group object names. All object names containing the specified prefix are grouped between the first occurrences of the delimiter (i.e., CommonPrefixes)
    pub fn set_delimiter(mut self, delimiter: impl ToString) -> Self {
        self.req.insert_query("delimiter", delimiter);
        self
    }
    /// Restrict the returned object keys to those with the given prefix.
    pub fn set_prefix(mut self, prefix: impl ToString) -> Self {
        self.req.insert_query("prefix", prefix.to_string());
        self
    }
    /// Set the key-marker
    pub fn set_key_marker(mut self, key_marker: impl ToString) -> Self {
        self.req.insert_query("key-marker", key_marker.to_string());
        self
    }
    /// Set the upload-id-marker
    pub fn set_upload_id_marker(mut self, upload_id_marker: impl ToString) -> Self {
        self.req
            .insert_query("upload-id-marker", upload_id_marker.to_string());
        self
    }
    /// Limit the maximum number of Multipart Upload events returned
    ///
    /// When a delimiter is set, this counts both files and groups
    ///
    /// Default: 1000, range 1-1000; values outside the range use the default
    pub fn set_max_uploads(mut self, max_keys: u32) -> Self {
        let max_keys = cmp::min(1000, cmp::max(1, max_keys));
        self.req.insert_query("max-uploads", max_keys);
        self
    }
    /// Send the request
    ///
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
