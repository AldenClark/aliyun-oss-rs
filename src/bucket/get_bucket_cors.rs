use crate::{
    Error,
    common::body_to_bytes,
    error::normal_error,
    request::{Oss, OssRequest},
};
use http::Method;

use super::{CorsConfiguration, CorsRule};

/// Retrieve the CORS rules of a bucket
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/zh/oss/developer-reference/getbucketcors) for details
pub struct GetBucketCors {
    req: OssRequest,
}

impl GetBucketCors {
    pub(super) fn new(oss: Oss) -> Self {
        let mut req = OssRequest::new(oss, Method::GET);
        req.insert_query("cors", "");
        GetBucketCors { req }
    }

    /// Send the request
    pub async fn send(self) -> Result<Vec<CorsRule>, Error> {
        let response = self.req.send_to_oss()?.await?;
        let status_code = response.status();
        match status_code {
            code if code.is_success() => {
                let response_bytes = body_to_bytes(response.into_body())
                    .await
                    .map_err(|_| Error::OssInvalidResponse(None))?;
                let cors: CorsConfiguration = serde_xml_rs::from_reader(&*response_bytes)
                    .map_err(|_| Error::OssInvalidResponse(Some(response_bytes)))?;
                Ok(cors.rules)
            }
            _ => Err(normal_error(response).await),
        }
    }
}
