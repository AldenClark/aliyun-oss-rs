use crate::{
    Error,
    common::RestoreTier,
    error::normal_error_sync,
    request_sync::{Oss, OssRequest},
};
use http::Method;

/// Restore an archived object.
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/document_detail/52930.html) for details.
///
/// 解冻归档对象。
///
/// 详情参见 [阿里云文档](https://help.aliyun.com/document_detail/52930.html)。
pub struct RestoreObjectSync {
    req: OssRequest,
    days: Option<u32>,
    tier: Option<RestoreTier>,
}
impl RestoreObjectSync {
    pub(super) fn new(oss: Oss) -> Self {
        let mut req = OssRequest::new(oss, Method::POST);
        req.insert_query("restore", "");
        RestoreObjectSync {
            req,
            days: None,
            tier: None,
        }
    }
    /// Set the number of days to keep the restored copy.
    ///
    /// 设置解冻副本保留天数。
    pub fn set_days(mut self, days: u32) -> Self {
        self.days = Some(days);
        self
    }
    /// Set the restore priority.
    ///
    /// 设置解冻优先级。
    pub fn set_tier(mut self, tier: RestoreTier) -> Self {
        self.tier = Some(tier);
        self
    }
    /// Send the request.
    ///
    /// 发送请求。
    pub fn send(mut self) -> Result<(), Error> {
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
            self.req.set_body(body_str.into_bytes());
        }
        // Build the HTTP request
        let response = self.req.send_to_oss()?;
        // Parse the response
        let status_code = response.status();
        match status_code {
            code if code.is_success() => Ok(()),
            _ => Err(normal_error_sync(response)),
        }
    }
}
