use crate::{
    Error,
    error::normal_error_sync,
    request_sync::{Oss, OssRequest},
};
use http::Method;

/// Start a WORM retention configuration for the bucket.
///
/// Returns the WORM ID on success.
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/zh/oss/developer-reference/initiatebucketworm) for details.
///
/// 启用 Bucket 的 WORM 保留策略。
///
/// 成功时返回 WORM ID。
///
/// 详情参见 [阿里云文档](https://help.aliyun.com/zh/oss/developer-reference/initiatebucketworm)。
pub struct InitiateBucketWormSync {
    req: OssRequest,
    retention_days: Option<u32>,
}

impl InitiateBucketWormSync {
    pub(super) fn new(oss: Oss) -> Self {
        let mut req = OssRequest::new(oss, Method::POST);
        req.insert_query("worm", "");
        req.insert_query("comp", "initiate");
        InitiateBucketWormSync { req, retention_days: None }
    }

    /// Set the retention period in days (1-36500).
    ///
    /// 设置保留天数（1-36500）。
    pub fn set_retention_days(mut self, days: u32) -> Self {
        self.retention_days = Some(days);
        self
    }

    /// Send the request and return the WORM ID.
    ///
    /// 发送请求并返回 WORM ID。
    pub fn send(mut self) -> Result<String, Error> {
        let days = self.retention_days.ok_or(Error::MissingRequestBody)?;
        let body = format!(
            "<InitiateWormConfiguration><RetentionPeriodInDays>{}</RetentionPeriodInDays></InitiateWormConfiguration>",
            days
        );
        self.req.set_body(body.into_bytes());
        let response = self.req.send_to_oss()?;
        match response.status() {
            code if code.is_success() => {
                let worm_id = response
                    .headers()
                    .get("x-oss-worm-id")
                    .and_then(|value| value.to_str().ok())
                    .ok_or(Error::OssInvalidResponse(None))?;
                Ok(worm_id.to_string())
            }
            _ => Err(normal_error_sync(response)),
        }
    }
}
