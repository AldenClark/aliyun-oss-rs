use crate::{
    Error,
    error::normal_error,
    request::{Oss, OssRequest},
};
use bytes::Bytes;
use http::Method;
use http_body_util::Full;

/// Start a WORM retention configuration for the bucket.
///
/// Returns the WORM ID on success.
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/zh/oss/developer-reference/initiatebucketworm) for details.
pub struct InitiateBucketWorm {
    req: OssRequest,
    retention_days: Option<u32>,
}

impl InitiateBucketWorm {
    pub(super) fn new(oss: Oss) -> Self {
        let mut req = OssRequest::new(oss, Method::POST);
        req.insert_query("worm", "");
        req.insert_query("comp", "initiate");
        InitiateBucketWorm {
            req,
            retention_days: None,
        }
    }

    /// Set the retention period in days (1-36500).
    pub fn set_retention_days(mut self, days: u32) -> Self {
        self.retention_days = Some(days);
        self
    }

    /// Send the request and return the generated WORM ID.
    pub async fn send(mut self) -> Result<String, Error> {
        let days = self.retention_days.ok_or(Error::MissingRequestBody)?;
        let body = format!(
            "<InitiateWormConfiguration><RetentionPeriodInDays>{}</RetentionPeriodInDays></InitiateWormConfiguration>",
            days
        );
        self.req.set_body(Full::new(Bytes::from(body)));
        let response = self.req.send_to_oss()?.await?;
        match response.status() {
            code if code.is_success() => {
                let worm_id = response
                    .headers()
                    .get("x-oss-worm-id")
                    .and_then(|value| value.to_str().ok())
                    .ok_or(Error::OssInvalidResponse(None))?;
                Ok(worm_id.to_string())
            }
            _ => Err(normal_error(response).await),
        }
    }
}
