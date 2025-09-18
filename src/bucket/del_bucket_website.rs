use crate::{
    Error,
    error::normal_error,
    request::{Oss, OssRequest},
};
use http::Method;

/// Delete the static website configuration of a bucket.
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/zh/oss/developer-reference/deletebucketwebsite) for details.
pub struct DelBucketWebsite {
    req: OssRequest,
}

impl DelBucketWebsite {
    pub(super) fn new(oss: Oss) -> Self {
        let mut req = OssRequest::new(oss, Method::DELETE);
        req.insert_query("website", "");
        DelBucketWebsite { req }
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
