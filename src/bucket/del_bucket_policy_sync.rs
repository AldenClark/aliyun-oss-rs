use crate::{
    Error,
    error::normal_error_sync,
    request_sync::{Oss, OssRequest},
};
use http::Method;

/// Delete the bucket policy.
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/zh/oss/developer-reference/deletebucketpolicy) for details.
///
/// 删除 Bucket 策略。
///
/// 详情参见 [阿里云文档](https://help.aliyun.com/zh/oss/developer-reference/deletebucketpolicy)。
pub struct DelBucketPolicySync {
    req: OssRequest,
}

impl DelBucketPolicySync {
    pub(super) fn new(oss: Oss) -> Self {
        let mut req = OssRequest::new(oss, Method::DELETE);
        req.insert_query("policy", "");
        DelBucketPolicySync { req }
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
