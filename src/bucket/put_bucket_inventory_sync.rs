use crate::{
    Error,
    error::normal_error_sync,
    request_sync::{Oss, OssRequest},
};
use http::Method;

/// Configure a bucket inventory task.
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/zh/oss/developer-reference/putbucketinventory) for details.
///
/// 配置 Bucket 清单任务。
///
/// 详情参见 [阿里云文档](https://help.aliyun.com/zh/oss/developer-reference/putbucketinventory)。
pub struct PutBucketInventorySync {
    req: OssRequest,
    body: Option<String>,
}

impl PutBucketInventorySync {
    pub(super) fn new(oss: Oss, inventory_id: impl Into<String>) -> Self {
        let mut req = OssRequest::new(oss, Method::PUT);
        req.insert_query("inventory", "");
        req.insert_query("inventoryId", inventory_id.into());
        PutBucketInventorySync { req, body: None }
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
    pub fn send(mut self) -> Result<(), Error> {
        let body = self.body.ok_or(Error::MissingRequestBody)?;
        self.req.set_body(body.into_bytes());
        let response = self.req.send_to_oss()?;
        match response.status() {
            code if code.is_success() => Ok(()),
            _ => Err(normal_error_sync(response)),
        }
    }
}
