use crate::{
    error::{Error, normal_error_sync},
    request_sync::{Oss, OssRequest},
};
use http::Method;

/// Enable or update the bucket logging configuration (sync).
///
/// 启用或更新 Bucket 日志配置（同步）。
pub struct PutBucketLoggingSync {
    req: OssRequest,
}
impl PutBucketLoggingSync {
    pub(crate) fn new(
        oss: Oss,
        target_bucket: impl Into<String>,
        target_prefix: impl Into<String>,
    ) -> Self {
        let mut req = OssRequest::new(oss, Method::PUT);
        req.insert_query("logging", "");
        let body = format!(
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?><BucketLoggingStatus><LoggingEnabled><TargetBucket>{}</TargetBucket><TargetPrefix>{}</TargetPrefix></LoggingEnabled></BucketLoggingStatus>",
            target_bucket.into(),
            target_prefix.into()
        );
        req.set_body(body.into_bytes());
        PutBucketLoggingSync { req }
    }
    /// Send the request.
    ///
    /// 发送请求。
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
