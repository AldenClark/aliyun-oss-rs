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
pub struct PutBucketInventory {
    req: OssRequest,
    body: Option<String>,
}

impl PutBucketInventory {
    pub(super) fn new(oss: Oss, inventory_id: impl ToString) -> Self {
        let mut req = OssRequest::new(oss, Method::PUT);
        req.insert_query("inventory", "");
        req.insert_query("inventoryId", inventory_id.to_string());
        PutBucketInventory { req, body: None }
    }

    /// Provide the inventory configuration XML.
    pub fn set_configuration(mut self, xml: impl ToString) -> Self {
        self.body = Some(xml.to_string());
        self
    }

    /// Send the request.
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
