use crate::{
    Error,
    error::normal_error,
    request::{Oss, OssRequest},
};
use http::Method;

/// Delete all lifecycle rules configured on the bucket.
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/zh/oss/developer-reference/deletebucketlifecycle) for details.
pub struct DelBucketLifecycle {
    req: OssRequest,
}

impl DelBucketLifecycle {
    pub(super) fn new(oss: Oss) -> Self {
        let mut req = OssRequest::new(oss, Method::DELETE);
        req.insert_query("lifecycle", "");
        DelBucketLifecycle { req }
    }

    /// Send the request.
    pub async fn send(self) -> Result<(), Error> {
        let response = self.req.send_to_oss()?.await?;
        match response.status() {
            code if code.is_success() => Ok(()),
            _ => Err(normal_error(response).await),
        }
    }
}
