use crate::{
    Error,
    error::normal_error,
    request::{Oss, OssRequest},
};
use http::Method;

/// Delete a bucket
///
/// To prevent accidental deletions, OSS does not allow deleting a non-empty bucket
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/document_detail/31973.html) for details
pub struct DelBucket {
    req: OssRequest,
}
impl DelBucket {
    pub(super) fn new(oss: Oss) -> Self {
        DelBucket {
            req: OssRequest::new(oss, Method::DELETE),
        }
    }

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
