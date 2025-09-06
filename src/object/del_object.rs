use crate::{
    error::normal_error,
    request::{Oss, OssRequest},
    Error,
};
use http::Method;

/// Delete the specified object
///
/// When deleting, OSS does not check whether the object exists; a valid request always succeeds
///
/// If versioning is enabled, the response is meaningful. Please refer to the documentation for delete markers and version IDs
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/document_detail/31982.html) for details
pub struct DelObject {
    req: OssRequest,
}
impl DelObject {
    pub(super) fn new(oss: Oss) -> Self {
        DelObject {
            req: OssRequest::new(oss, Method::DELETE),
        }
    }
    /// Send the request
    ///
    /// The return value is meaningful only when versioning is enabled
    ///
    /// - Return value 0: x-oss-delete-marker flag
    /// - Return value 1: Version ID. If no version ID is specified when deleting, this is the version ID of the new delete marker; otherwise, it is the specified version ID
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
