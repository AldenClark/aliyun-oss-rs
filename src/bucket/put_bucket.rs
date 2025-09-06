use crate::{
    common::{Acl, DataRedundancyType, StorageClass},
    error::normal_error,
    request::{Oss, OssRequest},
    Error,
};
use http::Method;
use bytes::Bytes;
use http_body_util::Full;
use serde_derive::Serialize;
use serde_xml_rs::to_string;

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
struct CreateBucketConfiguration {
    storage_class: Option<StorageClass>,
    data_redundancy_type: Option<DataRedundancyType>,
}

/// Call the PutBucket interface to create a bucket
///
/// An Alibaba Cloud account can create up to 100 buckets in a single region
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/document_detail/31959.html) for details
pub struct PutBucket {
    req: OssRequest,
    storage_class: Option<StorageClass>,
    data_redundancy_type: Option<DataRedundancyType>,
}
impl PutBucket {
    pub(super) fn new(oss: Oss) -> Self {
        PutBucket {
            req: OssRequest::new(oss, Method::PUT),
            storage_class: None,
            data_redundancy_type: None,
        }
    }
    /// Set the bucket access permissions
    pub fn set_acl(mut self, acl: Acl) -> Self {
        self.req.insert_header("x-oss-acl", acl);
        self
    }
    /// Specify the resource group ID
    ///
    /// If this header is included with a resource group ID, the created bucket belongs to that group. When the ID is rg-default-id, the bucket belongs to the default group.
    ///
    /// If the header is omitted, the bucket belongs to the default resource group.
    pub fn set_group_id(mut self, group_id: impl ToString) -> Self {
        self.req.insert_header("x-oss-resource-group-id", group_id);
        self
    }
    /// Set the bucket storage class
    pub fn set_storage_class(mut self, storage_class: StorageClass) -> Self {
        let body_str = format!(
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?><CreateBucketConfiguration>{}{}</CreateBucketConfiguration>",
            storage_class.to_string(),
            self.data_redundancy_type.map_or(String::new(),|v|format!("<DataRedundancyType>{}</DataRedundancyType>",v.to_string()))
        );
        self.storage_class = Some(storage_class);
        self.req.set_body(Full::new(Bytes::from(body_str)));
        self
    }
    /// Set the bucket data redundancy type
    pub fn set_redundancy_type(mut self, redundancy_type: DataRedundancyType) -> Self {
        let body_str = format!(
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?><CreateBucketConfiguration>{}{}</CreateBucketConfiguration>",
            self.storage_class.map(|v|format!("<StorageClass>{}</StorageClass>",v.to_string())).unwrap_or_else(||String::new()),
            redundancy_type.to_string()
        );
        self.req.set_body(Full::new(Bytes::from(body_str)));
        self.data_redundancy_type = Some(redundancy_type);
        self
    }
    /// Send the request
    pub async fn send(self) -> Result<(), Error> {
        let mut body = String::new();
        if self.data_redundancy_type.is_some() || self.storage_class.is_some() {
            let bucket_config = CreateBucketConfiguration {
                storage_class: self.storage_class,
                data_redundancy_type: self.data_redundancy_type,
            };
            if let Ok(body_str) = to_string(&bucket_config) {
                body.push_str(&body_str)
            };
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
