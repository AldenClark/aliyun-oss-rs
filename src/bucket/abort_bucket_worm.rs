use crate::{
    Error,
    error::normal_error,
    request::{Oss, OssRequest},
};
use http::Method;

/// Abort an in-progress WORM retention configuration.
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/zh/oss/developer-reference/abortbucketworm) for details.
pub struct AbortBucketWorm {
    req: OssRequest,
}

impl AbortBucketWorm {
    pub(super) fn new(oss: Oss, worm_id: impl ToString) -> Self {
        let mut req = OssRequest::new(oss, Method::DELETE);
        req.insert_query("wormId", worm_id.to_string());
        AbortBucketWorm { req }
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
