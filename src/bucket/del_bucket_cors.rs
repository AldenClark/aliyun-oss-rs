use crate::{
    Error,
    error::normal_error,
    request::{Oss, OssRequest},
};
use http::Method;

/// Delete all CORS rules configured on the bucket.
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/zh/oss/developer-reference/deletebucketcors) for details.
///
/// 删除 Bucket 的所有 CORS 规则。
///
/// 详情参见 [阿里云文档](https://help.aliyun.com/zh/oss/developer-reference/deletebucketcors)。
pub struct DelBucketCors {
    req: OssRequest,
}

impl DelBucketCors {
    pub(super) fn new(oss: Oss) -> Self {
        let mut req = OssRequest::new(oss, Method::DELETE);
        req.insert_query("cors", "");
        DelBucketCors { req }
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
