use crate::{
    Error,
    common::body_to_bytes,
    error::normal_error,
    request::{Oss, OssRequest},
};
use http::Method;

/// Retrieve the lifecycle configuration of a bucket.
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/zh/oss/developer-reference/getbucketlifecycle) for details.
///
/// 获取 Bucket 生命周期配置。
///
/// 详情参见 [阿里云文档](https://help.aliyun.com/zh/oss/developer-reference/getbucketlifecycle)。
pub struct GetBucketLifecycle {
    req: OssRequest,
}

impl GetBucketLifecycle {
    pub(super) fn new(oss: Oss) -> Self {
        let mut req = OssRequest::new(oss, Method::GET);
        req.insert_query("lifecycle", "");
        GetBucketLifecycle { req }
    }

    /// Send the request and return the lifecycle XML.
    ///
    /// 发送请求并返回生命周期 XML。
    pub async fn send(self) -> Result<String, Error> {
        let response = self.req.send_to_oss()?.await?;
        match response.status() {
            code if code.is_success() => {
                let bytes = body_to_bytes(response.into_body()).await?;
                Ok(String::from_utf8_lossy(bytes.as_ref()).into_owned())
            }
            _ => Err(normal_error(response).await),
        }
    }
}
