use crate::{
    common::RestoreTier,
    error::normal_error,
    request::{Oss, OssRequest},
    Error,
};
use http::Method;
use bytes::Bytes;
use http_body_util::Full;

/// Restore an archived object
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/document_detail/52930.html) for details
pub struct RestoreObject {
    req: OssRequest,
    days: Option<u32>,
    tier: Option<RestoreTier>,
}
impl RestoreObject {
    pub(super) fn new(oss: Oss) -> Self {
        let mut req = OssRequest::new(oss, Method::POST);
        req.insert_query("restore", "");
        RestoreObject {
            req,
            days: None,
            tier: None,
        }
    }
    /// Set the number of days to keep the restored copy
    ///
    pub fn set_days(mut self, days: u32) -> Self {
        self.days = Some(days);
        self
    }
    /// Set the restore priority
    ///
    pub fn set_tier(mut self, tier: RestoreTier) -> Self {
        self.tier = Some(tier);
        self
    }
    /// Send the request
    ///
    pub async fn send(mut self) -> Result<(), Error> {
        // Build the body
        let days_str = self
            .days
            .map(|v| format!("<Days>{}</Days>", v))
            .unwrap_or_else(|| String::new());
        let tier_str = self
            .tier
            .map(|v| format!("<JobParameters><Tier>{}</Tier></JobParameters>", v))
            .unwrap_or_else(|| String::new());
        if !days_str.is_empty() || !tier_str.is_empty() {
            let body_str = format!("<RestoreRequest>{}{}</RestoreRequest>", days_str, tier_str);
            self.req.set_body(Full::new(Bytes::from(body_str)));
        }
        // Build the HTTP request
        let response = self.req.send_to_oss()?.await?;
        // Parse the response
        let status_code = response.status();
        match status_code {
            code if code.is_success() => Ok(()),
            _ => Err(normal_error(response).await),
        }
    }
}
