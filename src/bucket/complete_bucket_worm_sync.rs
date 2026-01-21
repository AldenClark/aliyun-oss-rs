use crate::{
    Error,
    error::normal_error_sync,
    request_sync::{Oss, OssRequest},
};
use http::Method;

/// Complete a WORM retention configuration after locking objects.
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/zh/oss/developer-reference/completebucketworm) for details.
///
/// 在锁定对象后完成 WORM 保留策略。
///
/// 详情参见 [阿里云文档](https://help.aliyun.com/zh/oss/developer-reference/completebucketworm)。
pub struct CompleteBucketWormSync {
    req: OssRequest,
}

impl CompleteBucketWormSync {
    pub(super) fn new(oss: Oss, worm_id: impl Into<String>) -> Self {
        let mut req = OssRequest::new(oss, Method::POST);
        req.insert_query("wormId", worm_id.into());
        req.insert_query("comp", "complete");
        CompleteBucketWormSync { req }
    }

    /// Send the request.
    ///
    /// 发送请求。
    pub fn send(self) -> Result<(), Error> {
        let response = self.req.send_to_oss()?;
        match response.status() {
            code if code.is_success() => Ok(()),
            _ => Err(normal_error_sync(response)),
        }
    }
}
