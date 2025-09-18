use crate::{
    Error,
    common::Acl,
    error::normal_error,
    request::{Oss, OssRequest},
};
use http::Method;

/// Set the object's ACL
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/document_detail/31986.html) for details
pub struct PutObjectAcl {
    req: OssRequest,
}
impl PutObjectAcl {
    pub(super) fn new(oss: Oss, acl: Acl) -> Self {
        let mut req = OssRequest::new(oss, Method::PUT);
        req.insert_query("acl", "");
        req.insert_header("x-oss-object-acl", acl);
        PutObjectAcl { req }
    }
    /// Send the request
    ///
    pub async fn send(self) -> Result<(), Error> {
        // Build the HTTP request
        let response = self.req.send_to_oss()?.await?;
        // Parse the response
        let status_code = response.status();
        match status_code {
            code if code.is_success() => Ok(()),
            _ => Err(normal_error(response).await),
        }
    }
}
