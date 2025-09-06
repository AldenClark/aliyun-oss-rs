use crate::{
    common::StorageClass,
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
pub struct ListPartsResult {
    pub storage_class: StorageClass,
    pub part_number_marker: u32,
    pub next_part_number_marker: u32,
    pub is_truncated: bool,
    pub part: Option<Vec<Part>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Part {
    pub part_number: u32,
    pub last_modified: String,
    pub e_tag: String,
    pub hash_crc64ecma: u64,
    pub size: u64,
}

/// List all successfully uploaded parts for the specified Upload ID
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/document_detail/31998.html) for details
pub struct ListParts {
    req: OssRequest,
}

impl ListParts {
    pub(super) fn new(oss: Oss, upload_id: impl ToString) -> Self {
        let mut req = OssRequest::new(oss, Method::GET);
        req.insert_query("uploadId", upload_id);
        ListParts { req }
    }
    /// Limit the maximum number of parts returned
    ///
    /// Default: 1000, range 1-1000; values outside the range use the default
    pub fn set_max_parts(mut self, max_keys: u32) -> Self {
        let max_keys = cmp::min(1000, cmp::max(1, max_keys));
        self.req.insert_query("max-uploads", max_keys);
        self
    }
    /// Specify the starting position of the list
    ///
    pub fn set_part_number_marker(mut self, part_number_marker: u32) -> Self {
        self.req
            .insert_query("part-number-marker", part_number_marker);
        self
    }
    /// Send the request
    ///
    pub async fn send(self) -> Result<ListPartsResult, Error> {
        // Upload file
        let response = self.req.send_to_oss()?.await?;
        // Parse the response
        let status_code = response.status();
        match status_code {
            code if code.is_success() => {
                let response_bytes = body_to_bytes(response.into_body())
                    .await
                    .map_err(|_| Error::OssInvalidResponse(None))?;
                let result: ListPartsResult = serde_xml_rs::from_reader(&*response_bytes)
                    .map_err(|_| Error::OssInvalidResponse(Some(response_bytes)))?;
                Ok(result)
            }
            _ => Err(normal_error(response).await),
        }
    }
}
