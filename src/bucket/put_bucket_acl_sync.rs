use crate::{
    common::Acl,
    error::{Error, normal_error_sync},
    request_sync::{Oss, OssRequest},
};
use http::Method;

/// Set bucket access permissions (synchronous)
pub struct PutBucketAclSync {
    req: OssRequest,
}
impl PutBucketAclSync {
    pub(crate) fn new(oss: Oss) -> Self {
        let req = OssRequest::new(oss, Method::PUT);
        PutBucketAclSync { req }
    }
    /// Set the access control
    pub fn acl(mut self, acl: Acl) -> Self {
        self.req.insert_query("acl", "");
        self.req.insert_header("x-oss-acl", acl.to_string());
        self
    }
    /// Send the request
    pub fn send(self) -> Result<(), Error> {
        let response = self.req.send_to_oss()?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            Err(normal_error_sync(response))
        }
    }
}
