use crate::{
    Error,
    error::normal_error,
    request::{Oss, OssRequest},
};
use http::Method;

/// Delete the bucket policy.
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/zh/oss/developer-reference/deletebucketpolicy) for details.
pub struct DelBucketPolicy {
    req: OssRequest,
}

impl DelBucketPolicy {
    pub(super) fn new(oss: Oss) -> Self {
        let mut req = OssRequest::new(oss, Method::DELETE);
        req.insert_query("policy", "");
        DelBucketPolicy { req }
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
