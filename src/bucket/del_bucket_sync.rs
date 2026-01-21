use crate::{
    Error,
    error::normal_error_sync,
    request_sync::{Oss, OssRequest},
};
use http::Method;

/// Delete a bucket.
///
/// OSS does not allow deleting a non-empty bucket.
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/document_detail/31973.html) for details.
///
/// 删除 Bucket。
///
/// OSS 不允许删除非空 Bucket。
///
/// 详情参见 [阿里云文档](https://help.aliyun.com/document_detail/31973.html)。
pub struct DelBucketSync {
    req: OssRequest,
}
impl DelBucketSync {
    pub(super) fn new(oss: Oss) -> Self {
        DelBucketSync {
            req: OssRequest::new(oss, Method::DELETE),
        }
    }

    /// Send the request.
    ///
    /// 发送请求。
    pub fn send(self) -> Result<(), Error> {
        // Build the HTTP request
        let response = self.req.send_to_oss()?;
        // Parse the response
        let status_code = response.status();
        match status_code {
            code if code.is_success() => Ok(()),
            _ => Err(normal_error_sync(response)),
        }
    }
}
