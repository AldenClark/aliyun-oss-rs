use crate::{
    Error,
    error::normal_error,
    request::{Oss, OssRequest},
};
use bytes::Bytes;
use http::Method;
use http_body_util::Full;

/// Extend the retention period of an existing WORM configuration.
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/zh/oss/developer-reference/extendbucketworm) for details.
///
/// 延长已存在的 WORM 保留周期。
///
/// 详情参见 [阿里云文档](https://help.aliyun.com/zh/oss/developer-reference/extendbucketworm)。
pub struct ExtendBucketWorm {
    req: OssRequest,
    retention_days: Option<u32>,
}

impl ExtendBucketWorm {
    pub(super) fn new(oss: Oss, worm_id: impl Into<String>) -> Self {
        let mut req = OssRequest::new(oss, Method::POST);
        req.insert_query("wormId", worm_id.into());
        req.insert_query("comp", "extend");
        ExtendBucketWorm {
            req,
            retention_days: None,
        }
    }

    /// Set the new retention period in days.
    ///
    /// 设置新的保留天数。
    pub fn set_retention_days(mut self, days: u32) -> Self {
        self.retention_days = Some(days);
        self
    }

    /// Send the request.
    ///
    /// 发送请求。
    pub async fn send(mut self) -> Result<(), Error> {
        let days = self.retention_days.ok_or(Error::MissingRequestBody)?;
        let body = format!(
            "<ExtendWormConfiguration><RetentionPeriodInDays>{}</RetentionPeriodInDays></ExtendWormConfiguration>",
            days
        );
        self.req.set_body(Full::new(Bytes::from(body)));
        let response = self.req.send_to_oss()?.await?;
        match response.status() {
            code if code.is_success() => Ok(()),
            _ => Err(normal_error(response).await),
        }
    }
}
