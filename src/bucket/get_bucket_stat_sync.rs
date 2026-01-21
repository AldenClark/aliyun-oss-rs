use crate::common::body_to_bytes_sync;
use crate::{
    Error,
    error::normal_error_sync,
    request_sync::{Oss, OssRequest},
};
use http::Method;
use serde_derive::Deserialize;

// Returned content
/// Bucket storage statistics.
///
/// Bucket 存储统计信息。
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BucketStat {
    /// Total storage size in bytes.
    ///
    /// 总存储量（字节）。
    pub storage: u64,
    /// Total object count.
    ///
    /// 对象总数。
    pub object_count: u64,
    /// Multipart uploads initiated but not completed or aborted.
    ///
    /// 未完成或未取消的分片上传数量。
    pub multipart_upload_count: u64,
    /// Live channel count.
    ///
    /// Live Channel 数量。
    pub live_channel_count: u64,
    /// Timestamp in seconds when stats were generated.
    ///
    /// 统计生成时间戳（秒）。
    pub last_modified_time: u64,
    /// Standard storage size in bytes.
    ///
    /// 标准存储量（字节）。
    pub standard_storage: u64,
    /// Standard storage object count.
    ///
    /// 标准存储对象数量。
    pub standard_object_count: u64,
    /// Billable Infrequent Access storage in bytes.
    ///
    /// 低频存储计费量（字节）。
    pub infrequent_access_storage: u64,
    /// Actual Infrequent Access storage in bytes.
    ///
    /// 低频存储实际量（字节）。
    pub infrequent_access_real_storage: u64,
    /// Infrequent Access object count.
    ///
    /// 低频存储对象数量。
    pub infrequent_access_object_count: u64,
    /// Billable Archive storage in bytes.
    ///
    /// 归档存储计费量（字节）。
    pub archive_storage: u64,
    /// Actual Archive storage in bytes.
    ///
    /// 归档存储实际量（字节）。
    pub archive_real_storage: u64,
    /// Archive object count.
    ///
    /// 归档存储对象数量。
    pub archive_object_count: u64,
    /// Billable Cold Archive storage in bytes.
    ///
    /// 冷归档存储计费量（字节）。
    pub cold_archive_storage: u64,
    /// Actual Cold Archive storage in bytes.
    ///
    /// 冷归档存储实际量（字节）。
    pub cold_archive_real_storage: u64,
    /// Cold Archive object count.
    ///
    /// 冷归档对象数量。
    pub cold_archive_object_count: u64,
    /// Used reserved capacity in bytes.
    ///
    /// 预留容量使用量（字节）。
    pub reserved_capacity_storage: u64,
    /// Reserved capacity object count.
    ///
    /// 预留容量对象数量。
    pub reserved_capacity_object_count: u64,
    /// Billable Deep Cold Archive storage in bytes.
    ///
    /// 深冷归档计费量（字节）。
    pub deep_cold_archive_storage: u64,
    /// Actual Deep Cold Archive storage in bytes.
    ///
    /// 深冷归档实际量（字节）。
    pub deep_cold_archive_real_storage: u64,
    /// Deep Cold Archive object count.
    ///
    /// 深冷归档对象数量。
    pub deep_cold_archive_object_count: u64,
}

/// Retrieve bucket storage size and object counts.
///
/// Data is not real-time and may lag by more than one hour.
///
/// The returned timestamp may not be the latest; later calls can return a smaller value.
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/document_detail/426056.html) for details.
///
/// 获取 Bucket 存储量与对象数量。
///
/// 数据非实时，可能延迟超过 1 小时。
///
/// 返回时间不保证最新；后续调用可能返回更小的时间戳。
///
/// 详情参见 [阿里云文档](https://help.aliyun.com/document_detail/426056.html)。
pub struct GetBucketStatSync {
    req: OssRequest,
}
impl GetBucketStatSync {
    pub(super) fn new(oss: Oss) -> Self {
        let mut req = OssRequest::new(oss, Method::GET);
        req.insert_query("stat", "");
        GetBucketStatSync { req }
    }

    /// Send the request and return statistics.
    ///
    /// 发送请求并返回统计信息。
    pub fn send(self) -> Result<BucketStat, Error> {
        // Build the HTTP request
        let response = self.req.send_to_oss()?;
        // Parse the response
        let status_code = response.status();
        match status_code {
            code if code.is_success() => {
                let response_bytes = body_to_bytes_sync(response.into_body())
                    
                    .map_err(|_| Error::OssInvalidResponse(None))?;
                let bucket_stat: BucketStat = serde_xml_rs::from_reader(&*response_bytes)
                    .map_err(|_| Error::OssInvalidResponse(Some(response_bytes)))?;
                Ok(bucket_stat)
            }
            _ => Err(normal_error_sync(response)),
        }
    }
}
