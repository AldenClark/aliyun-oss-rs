use crate::{
    Error,
    common::body_to_bytes,
    error::normal_error,
    request::{Oss, OssRequest},
};
use http::Method;

/// Retrieve the lifecycle configuration of a bucket.
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/zh/oss/developer-reference/getbucketlifecycle) for details.
pub struct GetBucketLifecycle {
    req: OssRequest,
}

impl GetBucketLifecycle {
    pub(super) fn new(oss: Oss) -> Self {
        let mut req = OssRequest::new(oss, Method::GET);
        req.insert_query("lifecycle", "");
        GetBucketLifecycle { req }
    }

    /// Send the request and return the lifecycle XML document.
    pub async fn send(self) -> Result<String, Error> {
        let response = self.req.send_to_oss()?.await?;
        match response.status() {
            code if code.is_success() => {
                let bytes = body_to_bytes(response.into_body()).await?;
                Ok(String::from_utf8_lossy(bytes.as_ref()).into_owned())
            }
            _ => Err(normal_error(response).await),
        }
    }
}
