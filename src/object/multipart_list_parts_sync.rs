use crate::common::body_to_bytes_sync;
use crate::{
    common::StorageClass,
    error::{Error, normal_error_sync},
    request_sync::{Oss, OssRequest},
};
use http::Method;
use serde_derive::Deserialize;
use std::cmp;

// Returned content
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
/// Result of listing parts for a multipart upload.
///
/// 列举分片上传结果。
pub struct ListPartsResult {
    /// Storage class of the object.
    ///
    /// 对象存储类型。
    pub storage_class: StorageClass,
    /// Current part number marker.
    ///
    /// 当前分片标记。
    pub part_number_marker: u32,
    /// Next part number marker.
    ///
    /// 下一个分片标记。
    pub next_part_number_marker: u32,
    /// Whether the result is truncated.
    ///
    /// 是否被截断。
    pub is_truncated: bool,
    /// Parts returned in this response.
    ///
    /// 返回的分片列表。
    pub part: Option<Vec<Part>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
/// Information about a single uploaded part.
///
/// 单个已上传分片的信息。
pub struct Part {
    /// Part number.
    ///
    /// 分片编号。
    pub part_number: u32,
    /// Last modified time.
    ///
    /// 最近修改时间。
    pub last_modified: String,
    /// ETag of the part.
    ///
    /// 分片 ETag。
    pub e_tag: String,
    /// CRC64 hash value.
    ///
    /// CRC64 哈希值。
    pub hash_crc64ecma: u64,
    /// Part size in bytes.
    ///
    /// 分片大小（字节）。
    pub size: u64,
}

/// List uploaded parts for the given upload ID (sync).
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/document_detail/31998.html) for details.
///
/// 列举指定上传 ID 的已上传分片（同步）。
///
/// 详情参见 [阿里云文档](https://help.aliyun.com/document_detail/31998.html)。
pub struct ListPartsSync {
    req: OssRequest,
}

impl ListPartsSync {
    pub(super) fn new(oss: Oss, upload_id: impl Into<String>) -> Self {
        let mut req = OssRequest::new(oss, Method::GET);
        req.insert_query("uploadId", upload_id.into());
        ListPartsSync { req }
    }
    /// Limit the maximum number of parts returned.
    ///
    /// Default is 1000, valid range is 1-1000.
    ///
    /// 限制返回的最大分片数量。
    ///
    /// 默认 1000，合法范围 1-1000。
    pub fn set_max_parts(mut self, max_keys: u32) -> Self {
        let max_keys = cmp::min(1000, cmp::max(1, max_keys));
        self.req.insert_query("max-uploads", max_keys.to_string());
        self
    }
    /// Specify the starting part number marker.
    ///
    /// 指定起始分片标记。
    pub fn set_part_number_marker(mut self, part_number_marker: u32) -> Self {
        self.req.insert_query("part-number-marker", part_number_marker.to_string());
        self
    }
    /// Send the request and return the parts list.
    ///
    /// 发送请求并返回分片列表。
    pub fn send(self) -> Result<ListPartsResult, Error> {
        // Upload file
        let response = self.req.send_to_oss()?;
        // Parse the response
        let status_code = response.status();
        match status_code {
            code if code.is_success() => {
                let response_bytes =
                    body_to_bytes_sync(response.into_body()).map_err(|_| Error::OssInvalidResponse(None))?;
                let result: ListPartsResult = serde_xml_rs::from_reader(&*response_bytes)
                    .map_err(|_| Error::OssInvalidResponse(Some(response_bytes)))?;
                Ok(result)
            }
            _ => Err(normal_error_sync(response)),
        }
    }
}
