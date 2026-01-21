use crate::{
    common::format_gmt,
    error::{Error, normal_error},
    request::{Oss, OssRequest},
};
use http::Method;
use time::OffsetDateTime;

/// Copy source object content to a multipart upload part.
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/document_detail/31994.html) for details.
///
/// 将源对象内容复制到分片上传的某个分片。
///
/// 详情参见 [阿里云文档](https://help.aliyun.com/document_detail/31994.html)。
pub struct CopyToPart {
    req: OssRequest,
}
impl CopyToPart {
    pub(super) fn new(
        oss: Oss,
        part_number: u32,
        upload_id: impl Into<String>,
        copy_source: impl Into<String>,
    ) -> Self {
        let mut req = OssRequest::new(oss, Method::PUT);
        req.insert_query("partNumber", part_number.to_string());
        req.insert_query("uploadId", upload_id.into());
        req.insert_header("x-oss-copy-source", copy_source.into());
        CopyToPart { req }
    }
    /// Set the source byte range to copy.
    ///
    /// By default, the entire object is copied; byte indexing starts at 0.
    ///
    /// 设置复制的源对象字节范围。
    ///
    /// 默认复制整个对象，字节从 0 开始。
    pub fn set_source_range(mut self, start: usize, end: Option<usize>) -> Self {
        self.req.insert_header(
            "x-oss-copy-source-range",
            format!("bytes={}-{}", start, end.map(|v| v.to_string()).unwrap_or_else(|| String::new())),
        );
        self
    }
    /// Copy only if the source is modified after the given time.
    ///
    /// 仅当源对象在指定时间之后被修改时才复制。
    pub fn set_if_modified_since(mut self, if_modified_since: OffsetDateTime) -> Self {
        self.req.insert_header("x-oss-copy-source-if-modified-since", format_gmt(if_modified_since));
        self
    }
    /// Copy only if the source is not modified after the given time.
    ///
    /// 仅当源对象在指定时间之后未被修改时才复制。
    pub fn set_if_unmodified_since(mut self, if_unmodified_since: OffsetDateTime) -> Self {
        self.req.insert_header("x-oss-copy-source-if-unmodified-since", format_gmt(if_unmodified_since));
        self
    }
    /// Copy only if the source ETag matches the given value.
    ///
    /// ETag helps detect data changes and verify integrity.
    ///
    /// 仅当源对象 ETag 与给定值一致时才复制。
    ///
    /// ETag 可用于检测数据变更和校验完整性。
    pub fn set_if_match(mut self, if_match: impl Into<String>) -> Self {
        self.req.insert_header("x-oss-copy-source-if-match", if_match.into());
        self
    }
    /// Copy only if the source ETag does not match the given value.
    ///
    /// ETag helps detect data changes and verify integrity.
    ///
    /// 仅当源对象 ETag 与给定值不一致时才复制。
    ///
    /// ETag 可用于检测数据变更和校验完整性。
    pub fn set_if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
        self.req.insert_header("x-oss-copy-source-if-none-match", if_none_match.into());
        self
    }
    /// Send the copy request and return the ETag.
    ///
    /// 发送复制请求并返回 ETag。
    pub async fn send(self) -> Result<String, Error> {
        // Upload file
        let response = self.req.send_to_oss()?.await?;
        // Parse the response
        let status_code = response.status();
        match status_code {
            code if code.is_success() => {
                let e_tag = response
                    .headers()
                    .get("ETag")
                    .map(|v| String::from_utf8(v.as_bytes().to_vec()).ok())
                    .flatten()
                    .unwrap_or_else(|| String::new());
                Ok(e_tag)
            }
            _ => Err(normal_error(response).await),
        }
    }
}
