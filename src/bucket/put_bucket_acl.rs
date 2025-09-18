use crate::{
    Error,
    common::Acl,
    error::normal_error,
    request::{Oss, OssRequest},
};
use http::Method;

/// Set bucket access permissions
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/document_detail/31965.html) for details
pub struct PutBucketAcl {
    req: OssRequest,
}
impl PutBucketAcl {
    pub(crate) fn new(oss: Oss) -> Self {
        let mut req = OssRequest::new(oss, Method::PUT);
        req.insert_query("acl", "");
        PutBucketAcl { req }
    }
    /// Set the access control
    pub fn set_acl(mut self, acl: Acl) -> Self {
        self.req.insert_header("x-oss-acl", acl);
        self
    }
    /// Send the request
    pub async fn send(self) -> Result<(), Error> {
        let response = self.req.send_to_oss()?.await?;
        let status_code = response.status();
        match status_code {
            code if code.is_success() => Ok(()),
            _ => Err(normal_error(response).await),
        }
    }
}
