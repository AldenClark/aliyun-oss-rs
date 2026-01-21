use crate::{
    error::{Error, normal_error_sync},
    request_sync::{Oss, OssRequest},
};
use http::Method;

/// Complete a multipart upload (sync).
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/document_detail/31995.html) for details.
///
/// 完成分片上传（同步）。
///
/// 详情参见 [阿里云文档](https://help.aliyun.com/document_detail/31995.html)。
pub struct CompleteUploadSync<'a> {
    req: OssRequest,
    parts: Vec<(&'a str, &'a str)>,
}
impl<'a> CompleteUploadSync<'a> {
    pub(super) fn new(oss: Oss, upload_id: impl Into<String>) -> Self {
        let mut req = OssRequest::new(oss, Method::POST);
        req.insert_query("uploadId", upload_id.into());
        CompleteUploadSync { req, parts: Vec::new() }
    }
    /// Add part information in `(PartNumber, ETag)` pairs.
    ///
    /// 添加分片信息，格式为 `(PartNumber, ETag)`。
    pub fn add_parts(mut self, parts: Vec<(&'a str, &'a str)>) -> Self {
        self.parts.extend(parts);
        self
    }
    /// Send the complete request.
    ///
    /// 发送完成请求。
    pub fn send(mut self) -> Result<(), Error> {
        // Build body
        let body = format!(
            "<CompleteMultipartUpload>{}</CompleteMultipartUpload>",
            self.parts
                .iter()
                .map(|(part_num, e_tag)| format!(
                    "<Part><PartNumber>{}</PartNumber><ETag>{}</ETag></Part>",
                    part_num, e_tag
                ))
                .collect::<Vec<_>>()
                .join("")
        );
        let body_len = body.len();
        self.req.set_body(body.into_bytes());
        self.req.insert_header("Content-Length", body_len.to_string());
        // Upload file
        let response = self.req.send_to_oss()?;
        // Parse the response
        let status_code = response.status();
        match status_code {
            code if code.is_success() => Ok(()),
            _ => Err(normal_error_sync(response)),
        }
    }
}
