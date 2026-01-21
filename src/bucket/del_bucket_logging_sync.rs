use crate::{
    error::{Error, normal_error_sync},
    request_sync::{Oss, OssRequest},
};
use http::Method;

/// Delete the bucket logging configuration (sync).
///
/// 删除 Bucket 日志配置（同步）。
pub struct DelBucketLoggingSync {
    req: OssRequest,
}
impl DelBucketLoggingSync {
    pub(crate) fn new(oss: Oss) -> Self {
        let mut req = OssRequest::new(oss, Method::DELETE);
        req.insert_query("logging", "");
        DelBucketLoggingSync { req }
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
