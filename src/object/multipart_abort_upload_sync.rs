use crate::{
    error::{Error, normal_error_sync},
    request_sync::{Oss, OssRequest},
};
use http::Method;

/// Abort a multipart upload (sync).
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/document_detail/31996.html) for details.
///
/// 取消分片上传（同步）。
///
/// 详情参见 [阿里云文档](https://help.aliyun.com/document_detail/31996.html)。
pub struct AbortUploadSync {
    req: OssRequest,
}

impl AbortUploadSync {
    pub(super) fn new(oss: Oss, upload_id: impl Into<String>) -> Self {
        let mut req = OssRequest::new(oss, Method::DELETE);
        req.insert_query("uploadId", upload_id.into());
        AbortUploadSync { req }
    }
    /// Send the abort request.
    ///
    /// 发送取消请求。
    pub fn send(self) -> Result<(), Error> {
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
