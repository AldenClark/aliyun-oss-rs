use crate::{
    Error,
    error::normal_error,
    request::{Oss, OssRequest},
};
use bytes::Bytes;
use http::Method;
use http_body_util::Full;

/// Enable or update the bucket logging configuration
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/zh/oss/developer-reference/putbucketlogging) for details
pub struct PutBucketLogging {
    req: OssRequest,
}
impl PutBucketLogging {
    pub(super) fn new(
        oss: Oss,
        target_bucket: impl ToString,
        target_prefix: impl ToString,
    ) -> Self {
        let mut req = OssRequest::new(oss, Method::PUT);
        req.insert_query("logging", "");
        let body = format!(
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?><BucketLoggingStatus><LoggingEnabled><TargetBucket>{}</TargetBucket><TargetPrefix>{}</TargetPrefix></LoggingEnabled></BucketLoggingStatus>",
            target_bucket.to_string(),
            target_prefix.to_string()
        );
        req.set_body(Full::new(Bytes::from(body)));
        PutBucketLogging { req }
    }
    /// Send the request
    pub async fn send(self) -> Result<(), Error> {
        let response = self.req.send_to_oss()?.await?;
        let status_code = response.status();
        match status_code {
            code if code.is_success() => Ok(()),
            _ => Err(normal_error(response).await),
        }
    }
}
