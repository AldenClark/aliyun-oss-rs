use crate::{
    Error,
    error::normal_error,
    request::{Oss, OssRequest},
};
use http::Method;

/// Delete all lifecycle rules configured on the bucket.
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/zh/oss/developer-reference/deletebucketlifecycle) for details.
///
/// 删除 Bucket 的所有生命周期规则。
///
/// 详情参见 [阿里云文档](https://help.aliyun.com/zh/oss/developer-reference/deletebucketlifecycle)。
pub struct DelBucketLifecycle {
    req: OssRequest,
}

impl DelBucketLifecycle {
    pub(super) fn new(oss: Oss) -> Self {
        let mut req = OssRequest::new(oss, Method::DELETE);
        req.insert_query("lifecycle", "");
        DelBucketLifecycle { req }
    }

    /// Send the request.
    ///
    /// 发送请求。
    pub async fn send(self) -> Result<(), Error> {
        let response = self.req.send_to_oss()?.await?;
        match response.status() {
            code if code.is_success() => Ok(()),
            _ => Err(normal_error(response).await),
        }
    }
}
