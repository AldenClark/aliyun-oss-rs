use crate::{
    Error,
    common::{Acl, StorageClass},
    error::normal_error,
    request::{Oss, OssRequest},
};
use http::Method;

/// Create a symlink
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/document_detail/45126.html) for details
pub struct PutSymlink {
    req: OssRequest,
}
impl PutSymlink {
    pub(super) fn new(oss: Oss, symlink_target: impl ToString) -> Self {
        let mut req = OssRequest::new(oss, Method::PUT);
        req.insert_query("symlink", "");
        req.insert_header("x-oss-symlink-target", symlink_target);
        PutSymlink { req }
    }
    /// Set the file's access permissions
    pub fn set_acl(mut self, acl: Acl) -> Self {
        self.req.insert_header("x-oss-object-acl", acl);
        self
    }
    /// Set the file's storage class
    pub fn set_storage_class(mut self, storage_class: StorageClass) -> Self {
        self.req.insert_header("x-oss-storage-class", storage_class);
        self
    }
    /// Disallow overwriting files with the same name
    pub fn forbid_overwrite(mut self) -> Self {
        self.req.insert_header("x-oss-forbid-overwrite", "true");
        self
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
