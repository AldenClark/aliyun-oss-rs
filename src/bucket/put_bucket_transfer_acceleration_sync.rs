use crate::{
    Error,
    error::normal_error_sync,
    request_sync::{Oss, OssRequest},
};
use http::Method;

use super::TransferAccelerationConfiguration;

/// Configure transfer acceleration for the bucket.
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/zh/oss/developer-reference/putbuckettransferacceleration) for details.
///
/// 配置 Bucket 传输加速。
///
/// 详情见 [阿里云文档](https://help.aliyun.com/zh/oss/developer-reference/putbuckettransferacceleration)。
pub struct PutBucketTransferAccelerationSync {
    req: OssRequest,
    config: TransferAccelerationConfiguration,
}

impl PutBucketTransferAccelerationSync {
    pub(super) fn new(oss: Oss) -> Self {
        let mut req = OssRequest::new(oss, Method::PUT);
        req.insert_query("transferAcceleration", "");
        PutBucketTransferAccelerationSync { req, config: TransferAccelerationConfiguration::default() }
    }

    /// Enable or disable transfer acceleration.
    ///
    /// 启用或禁用传输加速。
    pub fn set_enabled(mut self, enabled: bool) -> Self {
        self.config.enabled = enabled;
        self
    }

    /// Replace the entire configuration document.
    ///
    /// 替换完整配置文档。
    pub fn set_configuration(mut self, config: TransferAccelerationConfiguration) -> Self {
        self.config = config;
        self
    }

    /// Send the request.
    ///
    /// 发送请求。
    pub fn send(mut self) -> Result<(), Error> {
        let body = serde_xml_rs::to_string(&self.config).map_err(|_| Error::InvalidCharacter)?;
        self.req.set_body(body.into_bytes());
        let response = self.req.send_to_oss()?;
        match response.status() {
            code if code.is_success() => Ok(()),
            _ => Err(normal_error_sync(response)),
        }
    }
}
