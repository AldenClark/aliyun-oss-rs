use crate::{
    Error,
    error::normal_error,
    request::{Oss, OssRequest},
};
use http::Method;

/// Delete a bucket inventory task configuration.
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/zh/oss/developer-reference/deletebucketinventory) for details.
pub struct DelBucketInventory {
    req: OssRequest,
}

impl DelBucketInventory {
    pub(super) fn new(oss: Oss, inventory_id: impl ToString) -> Self {
        let mut req = OssRequest::new(oss, Method::DELETE);
        req.insert_query("inventory", "");
        req.insert_query("inventoryId", inventory_id.to_string());
        DelBucketInventory { req }
    }

    /// Send the request.
    pub async fn send(self) -> Result<(), Error> {
        let response = self.req.send_to_oss()?.await?;
        match response.status() {
            code if code.is_success() => Ok(()),
            _ => Err(normal_error(response).await),
        }
    }
}
