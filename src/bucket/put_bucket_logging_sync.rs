use crate::{
    error::{Error, normal_error_sync},
    request_sync::{Oss, OssRequest},
};
use http::Method;

/// Enable or update the bucket logging configuration (synchronous)
pub struct PutBucketLoggingSync {
    req: OssRequest,
}
impl PutBucketLoggingSync {
    pub(crate) fn new(
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
        req.set_body(body.into_bytes());
        PutBucketLoggingSync { req }
    }
    /// Send the request
    pub fn send(self) -> Result<(), Error> {
        let response = self.req.send_to_oss()?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            Err(normal_error_sync(response))
        }
    }
}
