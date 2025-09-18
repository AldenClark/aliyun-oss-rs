use crate::common::body_to_bytes;
use crate::{
    Error,
    error::normal_error,
    request::{Oss, OssRequest},
};
use http::Method;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LoggingEnabled {
    pub target_bucket: String,
    pub target_prefix: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct BucketLoggingStatus {
    #[serde(rename = "LoggingEnabled")]
    pub logging_enabled: Option<LoggingEnabled>,
}

/// Retrieve the bucket logging configuration
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/zh/oss/developer-reference/getbucketlogging) for details
pub struct GetBucketLogging {
    req: OssRequest,
}
impl GetBucketLogging {
    pub(super) fn new(oss: Oss) -> Self {
        let mut req = OssRequest::new(oss, Method::GET);
        req.insert_query("logging", "");
        GetBucketLogging { req }
    }
    /// Send the request
    pub async fn send(self) -> Result<Option<LoggingEnabled>, Error> {
        let response = self.req.send_to_oss()?.await?;
        let status_code = response.status();
        match status_code {
            code if code.is_success() => {
                let response_bytes = body_to_bytes(response.into_body())
                    .await
                    .map_err(|_| Error::OssInvalidResponse(None))?;
                let status: BucketLoggingStatus = serde_xml_rs::from_reader(&*response_bytes)
                    .map_err(|_| Error::OssInvalidResponse(Some(response_bytes)))?;
                Ok(status.logging_enabled)
            }
            _ => Err(normal_error(response).await),
        }
    }
}
