use crate::{
    Error,
    error::normal_error_sync,
    request_sync::{Oss, OssRequest},
};
use http::Method;

/// Remove all tags from an object.
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/document_detail/114879.html) for details.
///
/// 删除对象的所有标签。
///
/// 详情参见 [阿里云文档](https://help.aliyun.com/document_detail/114879.html)。
pub struct DelObjectTaggingSync {
    req: OssRequest,
}
impl DelObjectTaggingSync {
    pub(super) fn new(oss: Oss) -> Self {
        let mut req = OssRequest::new(oss, Method::DELETE);
        req.insert_query("tagging", "");
        DelObjectTaggingSync { req }
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
