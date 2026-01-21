use crate::{
    Error,
    error::normal_error,
    request::{Oss, OssRequest},
};
use bytes::Bytes;
use http::Method;
use http_body_util::Full;

/// Configure a bucket inventory task.
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/zh/oss/developer-reference/putbucketinventory) for details.
///
/// 配置 Bucket 清单任务。
///
/// 详情参见 [阿里云文档](https://help.aliyun.com/zh/oss/developer-reference/putbucketinventory)。
pub struct PutBucketInventory {
    req: OssRequest,
    body: Option<String>,
}

impl PutBucketInventory {
    pub(super) fn new(oss: Oss, inventory_id: impl Into<String>) -> Self {
        let mut req = OssRequest::new(oss, Method::PUT);
        req.insert_query("inventory", "");
        req.insert_query("inventoryId", inventory_id.into());
        PutBucketInventory { req, body: None }
    }

    /// Provide the inventory configuration XML.
    ///
    /// 提供清单配置 XML。
    pub fn set_configuration(mut self, xml: impl Into<String>) -> Self {
        self.body = Some(xml.into());
        self
    }

    /// Send the request.
    ///
    /// 发送请求。
    pub async fn send(mut self) -> Result<(), Error> {
        let body = self.body.ok_or(Error::MissingRequestBody)?;
        self.req.set_body(Full::new(Bytes::from(body)));
        let response = self.req.send_to_oss()?.await?;
        match response.status() {
            code if code.is_success() => Ok(()),
            _ => Err(normal_error(response).await),
        }
    }
}
