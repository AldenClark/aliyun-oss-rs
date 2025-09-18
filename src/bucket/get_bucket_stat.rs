use crate::common::body_to_bytes;
use crate::{
    Error,
    error::normal_error,
    request::{Oss, OssRequest},
};
use http::Method;
use serde_derive::Deserialize;

// Returned content
/// Bucket capacity information
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BucketStat {
    /// Total storage size in bytes
    pub storage: u64,
    /// Total number of files
    pub object_count: u64,
    /// Number of multipart uploads that have been initiated but not completed or aborted
    pub multipart_upload_count: u64,
    /// Number of Live Channels
    pub live_channel_count: u64,
    /// Time when the storage information was obtained, as a timestamp in seconds
    pub last_modified_time: u64,
    /// Storage size of Standard storage class in bytes
    pub standard_storage: u64,
    /// Number of files in Standard storage class
    pub standard_object_count: u64,
    /// Billable storage size for Infrequent Access in bytes
    pub infrequent_access_storage: u64,
    /// Actual storage size for Infrequent Access in bytes
    pub infrequent_access_real_storage: u64,
    /// Number of files in Infrequent Access
    pub infrequent_access_object_count: u64,
    /// Billable storage size for Archive in bytes
    pub archive_storage: u64,
    /// Actual storage size for Archive in bytes
    pub archive_real_storage: u64,
    /// Number of files in Archive
    pub archive_object_count: u64,
    /// Billable storage size for Cold Archive in bytes
    pub cold_archive_storage: u64,
    /// Actual storage size for Cold Archive in bytes
    pub cold_archive_real_storage: u64,
    /// Number of files in Cold Archive
    pub cold_archive_object_count: u64,
    /// Used reserved capacity
    pub reserved_capacity_storage: u64,
    /// Number of files using reserved capacity
    pub reserved_capacity_object_count: u64,
    /// Billable storage size for Deep Cold Archive in bytes
    pub deep_cold_archive_storage: u64,
    /// Actual storage size for Deep Cold Archive in bytes
    pub deep_cold_archive_real_storage: u64,
    /// Number of files in Deep Cold Archive
    pub deep_cold_archive_object_count: u64,
}

/// Retrieve the storage size and file count of a bucket
///
/// The data is not real-time and may be delayed by over an hour
///
/// The returned time point is not guaranteed to be the latest; a later call may return a smaller LastModifiedTime than a previous call
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/document_detail/426056.html) for details
pub struct GetBucketStat {
    req: OssRequest,
}
impl GetBucketStat {
    pub(super) fn new(oss: Oss) -> Self {
        let mut req = OssRequest::new(oss, Method::GET);
        req.insert_query("stat", "");
        GetBucketStat { req }
    }

    /// Send the request
    pub async fn send(self) -> Result<BucketStat, Error> {
        // Build the HTTP request
        let response = self.req.send_to_oss()?.await?;
        // Parse the response
        let status_code = response.status();
        match status_code {
            code if code.is_success() => {
                let response_bytes = body_to_bytes(response.into_body())
                    .await
                    .map_err(|_| Error::OssInvalidResponse(None))?;
                let bucket_stat: BucketStat = serde_xml_rs::from_reader(&*response_bytes)
                    .map_err(|_| Error::OssInvalidResponse(Some(response_bytes)))?;
                Ok(bucket_stat)
            }
            _ => Err(normal_error(response).await),
        }
    }
}
