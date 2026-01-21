use crate::{
    common::Acl,
    error::{Error, normal_error_sync},
    request_sync::{Oss, OssRequest},
};
use http::Method;

/// Set bucket ACL (sync).
///
/// 设置 Bucket ACL（同步）。
pub struct PutBucketAclSync {
    req: OssRequest,
}
impl PutBucketAclSync {
    pub(crate) fn new(oss: Oss) -> Self {
        let req = OssRequest::new(oss, Method::PUT);
        PutBucketAclSync { req }
    }
    /// Set the access control.
    ///
    /// 设置访问控制。
    pub fn acl(mut self, acl: Acl) -> Self {
        self.req.insert_query("acl", "");
        self.req.insert_header("x-oss-acl", acl.to_string());
        self
    }
    /// Send the request.
    ///
    /// 发送请求。
    pub fn send(self) -> Result<(), Error> {
        let response = self.req.send_to_oss()?;
        let status = response.status();
        if status.is_success() { Ok(()) } else { Err(normal_error_sync(response)) }
    }
}
