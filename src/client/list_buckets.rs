use crate::common::body_to_bytes;
use crate::{
    Error,
    common::StorageClass,
    error::normal_error,
    request::{Oss, OssRequest},
};
use http::Method;
use serde_derive::Deserialize;

// Return value
/// Basic bucket information
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BucketBase {
    /// Bucket name
    pub name: String,
    /// Region
    pub region: String,
    /// Region identifier in OSS
    pub location: String,
    /// Public endpoint
    pub extranet_endpoint: String,
    /// Internal endpoint
    pub intranet_endpoint: String,
    /// Storage class
    pub storage_class: StorageClass,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct Buckets {
    pub bucket: Option<Vec<BucketBase>>,
}

// Result set of bucket list query
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct ListAllMyBucketsResult {
    /// If a single query does not list all buckets, next_marker can be used for the next query
    pub next_marker: Option<String>,
    /// Bucket list
    pub buckets: Buckets,
}

/// Result set of bucket list query
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListAllMyBuckets {
    /// If a single query does not list all buckets, next_marker can be used for the next query
    pub next_marker: Option<String>,
    /// Bucket list
    pub buckets: Option<Vec<BucketBase>>,
}

/// List buckets
///
/// Filters can be set via the `set_` methods. See the [Alibaba Cloud documentation](https://help.aliyun.com/document_detail/31957.html) for details
///
/// ```ignore
/// let client = OssClient::new("AccessKey ID","AccessKey Secret","oss-cn-beijing.aliyuncs.com");
/// let buckets = client.list_buckets().set_prefix("rust").send().await;
/// println!("{:#?}", buckets);
/// ```
///
pub struct ListBuckets {
    req: OssRequest,
}

impl ListBuckets {
    pub(super) fn new(oss: Oss) -> Self {
        ListBuckets {
            req: OssRequest::new(oss, Method::GET),
        }
    }

    /// Limit the returned bucket names to those starting with prefix. Without setting, no prefix filtering is applied.
    ///
    /// Prefix requirements:
    /// - Cannot be empty and must not exceed 63 bytes
    /// - May contain only lowercase letters, numbers, and hyphens, and cannot start with a hyphen
    ///
    pub fn set_prefix(mut self, prefix: impl ToString) -> Self {
        self.req.insert_query("prefix", prefix);
        self
    }
    /// Start returning results from the first key alphabetically after marker. If not set, return from the beginning.
    pub fn set_marker(mut self, marker: impl ToString) -> Self {
        self.req.insert_query("marker", marker);
        self
    }
    /// Limit the maximum number of buckets returned. Range: 1-1000, default: 100
    pub fn set_max_keys(mut self, max_keys: u32) -> Self {
        self.req.insert_query("max-keys", max_keys);
        self
    }
    /// Specify the resource group ID
    pub fn set_group_id(mut self, group_id: impl ToString) -> Self {
        self.req.insert_header("x-oss-resource-group-id", group_id);
        self
    }
    /// Specify the endpoint from which to initiate the query; this does not limit the region of buckets
    ///
    /// Defaults to oss.aliyuncs.com. If inaccessible, set an endpoint you can reach
    pub fn set_endpoint(mut self, endpoint: impl ToString) -> Self {
        self.req.set_endpoint(endpoint);
        self
    }
    /// Send the request
    pub async fn send(self) -> Result<ListAllMyBuckets, Error> {
        // Build the HTTP request
        let response = self.req.send_to_oss()?.await?;
        // Parse the response
        let status_code = response.status();
        match status_code {
            code if code.is_success() => {
                let response_bytes = body_to_bytes(response.into_body())
                    .await
                    .map_err(|_| Error::OssInvalidResponse(None))?;
                let result: ListAllMyBucketsResult = serde_xml_rs::from_reader(&*response_bytes)
                    .map_err(|_| Error::OssInvalidResponse(Some(response_bytes)))?;
                Ok(ListAllMyBuckets {
                    next_marker: result.next_marker,
                    buckets: result.buckets.bucket,
                })
            }
            _ => Err(normal_error(response).await),
        }
    }
}
