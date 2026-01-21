use crate::{
    Error,
    error::normal_error_sync,
    request_sync::{Oss, OssRequest},
};
use http::Method;

/// Abort an in-progress WORM retention configuration.
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/zh/oss/developer-reference/abortbucketworm) for details.
///
/// 取消进行中的 WORM 保留策略。
///
/// 详情参见 [阿里云文档](https://help.aliyun.com/zh/oss/developer-reference/abortbucketworm)。
pub struct AbortBucketWormSync {
    req: OssRequest,
}

impl AbortBucketWormSync {
    pub(super) fn new(oss: Oss, worm_id: impl Into<String>) -> Self {
        let mut req = OssRequest::new(oss, Method::DELETE);
        req.insert_query("wormId", worm_id.into());
        AbortBucketWormSync { req }
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
