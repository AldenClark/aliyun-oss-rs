use crate::{
    Error,
    common::body_to_bytes,
    error::normal_error,
    request::{Oss, OssRequest},
};
use http::Method;

use super::BucketWormConfiguration;

/// Retrieve the WORM configuration of a bucket.
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/zh/oss/developer-reference/getbucketworm) for details.
pub struct GetBucketWorm {
    req: OssRequest,
}

impl GetBucketWorm {
    pub(super) fn new(oss: Oss) -> Self {
        let mut req = OssRequest::new(oss, Method::GET);
        req.insert_query("worm", "");
        GetBucketWorm { req }
    }

    /// Send the request and return the parsed configuration.
    pub async fn send(self) -> Result<BucketWormConfiguration, Error> {
        let response = self.req.send_to_oss()?.await?;
        match response.status() {
            code if code.is_success() => {
                let bytes = body_to_bytes(response.into_body()).await?;
                let config: BucketWormConfiguration = serde_xml_rs::from_reader(bytes.as_ref())
                    .map_err(|_| Error::OssInvalidResponse(Some(bytes)))?;
                Ok(config)
            }
            _ => Err(normal_error(response).await),
        }
    }
}
