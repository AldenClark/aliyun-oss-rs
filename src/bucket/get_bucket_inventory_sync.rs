use crate::{
    Error,
    common::body_to_bytes_sync,
    error::normal_error_sync,
    request_sync::{Oss, OssRequest},
};
use http::Method;

/// Retrieve a specific inventory task configuration.
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/zh/oss/developer-reference/getbucketinventory) for details.
///
/// 获取指定的 Bucket 清单任务配置。
///
/// 详情参见 [阿里云文档](https://help.aliyun.com/zh/oss/developer-reference/getbucketinventory)。
pub struct GetBucketInventorySync {
    req: OssRequest,
}

impl GetBucketInventorySync {
    pub(super) fn new(oss: Oss, inventory_id: impl Into<String>) -> Self {
        let mut req = OssRequest::new(oss, Method::GET);
        req.insert_query("inventory", "");
        req.insert_query("inventoryId", inventory_id.into());
        GetBucketInventorySync { req }
    }

    /// Send the request and return the inventory XML.
    ///
    /// 发送请求并返回清单 XML。
    pub fn send(self) -> Result<String, Error> {
        let response = self.req.send_to_oss()?;
        match response.status() {
            code if code.is_success() => {
                let bytes = body_to_bytes_sync(response.into_body())?;
                Ok(String::from_utf8_lossy(bytes.as_ref()).into_owned())
            }
            _ => Err(normal_error_sync(response)),
        }
    }
}
