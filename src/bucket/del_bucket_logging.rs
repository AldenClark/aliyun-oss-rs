use crate::{
    Error,
    error::normal_error,
    request::{Oss, OssRequest},
};
use http::Method;

/// Delete the bucket logging configuration.
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/zh/oss/developer-reference/deletebucketlogging) for details.
///
/// 删除 Bucket 日志配置。
///
/// 详情参见 [阿里云文档](https://help.aliyun.com/zh/oss/developer-reference/deletebucketlogging)。
pub struct DelBucketLogging {
    req: OssRequest,
}
impl DelBucketLogging {
    pub(super) fn new(oss: Oss) -> Self {
        let mut req = OssRequest::new(oss, Method::DELETE);
        req.insert_query("logging", "");
        DelBucketLogging { req }
    }
    /// Send the request.
    ///
    /// 发送请求。
    pub async fn send(self) -> Result<(), Error> {
        let response = self.req.send_to_oss()?.await?;
        let status_code = response.status();
        match status_code {
            code if code.is_success() => Ok(()),
            _ => Err(normal_error(response).await),
        }
    }
}
