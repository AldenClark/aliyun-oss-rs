use crate::{
    Error,
    error::normal_error_sync,
    request_sync::{Oss, OssRequest},
};
use http::Method;

/// Delete all lifecycle rules configured on the bucket.
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/zh/oss/developer-reference/deletebucketlifecycle) for details.
///
/// 删除 Bucket 的所有生命周期规则。
///
/// 详情参见 [阿里云文档](https://help.aliyun.com/zh/oss/developer-reference/deletebucketlifecycle)。
pub struct DelBucketLifecycleSync {
    req: OssRequest,
}

impl DelBucketLifecycleSync {
    pub(super) fn new(oss: Oss) -> Self {
        let mut req = OssRequest::new(oss, Method::DELETE);
        req.insert_query("lifecycle", "");
        DelBucketLifecycleSync { req }
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
