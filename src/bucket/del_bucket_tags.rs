use crate::{
    Error,
    error::normal_error,
    request::{Oss, OssRequest},
};
use http::Method;

/// Delete bucket tags.
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/zh/oss/developer-reference/deletebuckettags) for details.
///
/// 删除 Bucket 标签。
///
/// 详情见 [阿里云文档](https://help.aliyun.com/zh/oss/developer-reference/deletebuckettags)。
pub struct DelBucketTags {
    req: OssRequest,
}

impl DelBucketTags {
    pub(super) fn new(oss: Oss) -> Self {
        let mut req = OssRequest::new(oss, Method::DELETE);
        req.insert_query("tagging", "");
        DelBucketTags { req }
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
