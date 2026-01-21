use crate::{
    Error,
    common::Acl,
    error::normal_error_sync,
    request_sync::{Oss, OssRequest},
};
use http::Method;

/// Set the object's ACL.
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/document_detail/31986.html) for details.
///
/// 设置对象 ACL。
///
/// 详情参见 [阿里云文档](https://help.aliyun.com/document_detail/31986.html)。
pub struct PutObjectAclSync {
    req: OssRequest,
}
impl PutObjectAclSync {
    pub(super) fn new(oss: Oss, acl: Acl) -> Self {
        let mut req = OssRequest::new(oss, Method::PUT);
        req.insert_query("acl", "");
        req.insert_header("x-oss-object-acl", acl.to_string());
        PutObjectAclSync { req }
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
