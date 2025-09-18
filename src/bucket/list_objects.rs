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
pub struct ObjectsList {
    // Continuation token for subsequent requests
    pub next_continuation_token: Option<String>,
    // File list
    pub contents: Option<Vec<ObjectInfo>>,
    // Group list
    pub common_prefixes: Option<Vec<CommonPrefixes>>,
}

/// Object information
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ObjectInfo {
    /// Object path
    pub key: String,
    /// Last modified time of the object
    pub last_modified: String,
    /// The ETag is generated for each object to identify its content. It can be used to check if the object has changed, but it is not recommended to use it as an MD5 checksum for data integrity.
    pub e_tag: String,
    #[serde(rename = "Type")]
    pub type_field: String,
    /// Size of the object in bytes
    pub size: u64,
    /// Storage class of the object
    pub storage_class: StorageClass,
    /// Restore status of the object
    pub restore_info: Option<String>,
    /// Owner information of the bucket
    pub owner: Option<Owner>,
}

/// Group list
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CommonPrefixes {
    /// Prefix
    pub prefix: String,
}

/// List information of all files in the bucket
///
/// By default, retrieves the first 1000 files
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/document_detail/187544.html) for details
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
    /// Character used to group object names. All object names containing the specified prefix are grouped between the first occurrences of the delimiter (i.e., CommonPrefixes)
    pub fn set_delimiter(mut self, delimiter: impl ToString) -> Self {
        self.req.insert_query("delimiter", delimiter);
        self
    }
    /// Specify where to start listing objects alphabetically after start-after.
    ///
    /// start-after is used for pagination and must be less than 1024 bytes.
    ///
    /// When performing conditional queries, even if start-after does not exist, listing starts from the next object in alphabetical order.
    pub fn set_start_after(mut self, start_after: impl ToString) -> Self {
        self.req.insert_query("start-after", start_after);
        self
    }
    /// Specify the token from which the listing operation should begin.
    ///
    /// This token can be obtained from NextContinuationToken in the ListObjects result.
    pub fn set_continuation_token(mut self, continuation_token: impl ToString) -> Self {
        self.req
            .insert_query("continuation-token", continuation_token);
        self
    }
    /// Restrict the returned object keys to those with the given prefix.
    pub fn set_prefix(mut self, prefix: impl ToString) -> Self {
        self.req.insert_query("prefix", prefix.to_string());
        self
    }
    /// Specify the maximum number of files to return.
    ///
    /// When a delimiter is set, this counts both files and groups
    ///
    /// Default: 1000, range 1-1000; values outside the range use the default
    pub fn set_max_keys(mut self, max_keys: u32) -> Self {
        let max_keys = cmp::min(1000, cmp::max(1, max_keys));
        self.req.insert_query("max-keys", max_keys);
        self
    }
    /// Specify whether to include owner information in the result.
    pub fn fetch_owner(mut self) -> Self {
        self.req.insert_query("fetch-owner", "true");
        self
    }
    /// Send the request
    ///
    pub async fn send(self) -> Result<ObjectsList, Error> {
        // Build the HTTP request
        let response = self.req.send_to_oss()?.await?;
        // Parse the response
        let status_code = response.status();
        match status_code {
            code if code.is_success() => {
                let response_bytes = body_to_bytes(response.into_body())
                    .await
                    .map_err(|_| Error::OssInvalidResponse(None))?;
                let object_list: ObjectsList = serde_xml_rs::from_reader(&*response_bytes)
                    .map_err(|_| Error::OssInvalidResponse(Some(response_bytes)))?;
                Ok(object_list)
            }
            _ => return Err(normal_error(response).await),
        }
    }
}
