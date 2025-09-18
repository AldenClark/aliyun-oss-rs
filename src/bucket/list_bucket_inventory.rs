use crate::{
    Error,
    common::body_to_bytes,
    error::normal_error,
    request::{Oss, OssRequest},
};
use http::Method;

/// List all inventory task configurations defined on the bucket.
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/zh/oss/developer-reference/listbucketinventory) for details.
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
