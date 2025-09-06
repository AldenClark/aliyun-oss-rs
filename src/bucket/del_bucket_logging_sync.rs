use crate::{
    error::{normal_error_sync, Error},
    request_sync::{Oss, OssRequest},
};
use http::Method;

/// Delete the bucket logging configuration (synchronous)
pub struct DelBucketLoggingSync {
    req: OssRequest,
}
impl DelBucketLoggingSync {
    pub(crate) fn new(oss: Oss) -> Self {
        let mut req = OssRequest::new(oss, Method::DELETE);
        req.insert_query("logging", "");
        DelBucketLoggingSync { req }
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
