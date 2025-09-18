use crate::{
    Error,
    common::body_to_bytes,
    error::normal_error,
    request::{Oss, OssRequest},
};
use http::Method;

/// Retrieve a specific inventory task configuration.
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/zh/oss/developer-reference/getbucketinventory) for details.
pub struct GetBucketInventory {
    req: OssRequest,
}

impl GetBucketInventory {
    pub(super) fn new(oss: Oss, inventory_id: impl ToString) -> Self {
        let mut req = OssRequest::new(oss, Method::GET);
        req.insert_query("inventory", "");
        req.insert_query("inventoryId", inventory_id.to_string());
        GetBucketInventory { req }
    }

    /// Send the request and return the inventory XML.
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
