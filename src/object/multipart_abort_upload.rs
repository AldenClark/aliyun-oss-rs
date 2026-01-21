use crate::{
    error::{Error, normal_error},
    request::{Oss, OssRequest},
};
use http::Method;

/// Abort a multipart upload.
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/document_detail/31996.html) for details.
///
/// 取消分片上传。
///
/// 详情参见 [阿里云文档](https://help.aliyun.com/document_detail/31996.html)。
pub struct AbortUpload {
    req: OssRequest,
}

impl AbortUpload {
    pub(super) fn new(oss: Oss, upload_id: impl Into<String>) -> Self {
        let mut req = OssRequest::new(oss, Method::DELETE);
        req.insert_query("uploadId", upload_id.into());
        AbortUpload { req }
    }
    /// Send the abort request.
    ///
    /// 发送取消请求。
    pub async fn send(self) -> Result<(), Error> {
        // Upload file
        let response = self.req.send_to_oss()?.await?;
        // Parse the response
        let status_code = response.status();
        match status_code {
            code if code.is_success() => Ok(()),
            _ => Err(normal_error(response).await),
        }
    }
}
