use crate::{
    error::normal_error,
    request::{Oss, OssRequest},
    Error,
};
use http::Method;

/// Remove all tags from an object
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/document_detail/114879.html) for details
pub struct DelObjectTagging {
    req: OssRequest,
}
impl DelObjectTagging {
    pub(super) fn new(oss: Oss) -> Self {
        let mut req = OssRequest::new(oss, Method::DELETE);
        req.insert_query("tagging", "");
        DelObjectTagging { req }
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
