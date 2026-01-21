use crate::{
    Error,
    common::body_to_bytes,
    error::normal_error,
    request::{Oss, OssRequest},
};
use http::Method;

/// List all inventory task configurations for a bucket.
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/zh/oss/developer-reference/listbucketinventory) for details.
///
/// 列举 Bucket 的清单任务配置。
///
/// 详情参见 [阿里云文档](https://help.aliyun.com/zh/oss/developer-reference/listbucketinventory)。
pub struct ListBucketInventory {
    req: OssRequest,
}

impl ListBucketInventory {
    pub(super) fn new(oss: Oss) -> Self {
        let mut req = OssRequest::new(oss, Method::GET);
        req.insert_query("inventory", "");
        req.insert_query("comp", "list");
        ListBucketInventory { req }
    }

    /// Send the request and return the XML response.
    ///
    /// 发送请求并返回 XML 响应。
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
