use crate::{
    Error,
    error::normal_error_sync,
    request_sync::{Oss, OssRequest},
};
use http::Method;

use super::{VersioningConfiguration, VersioningStatus};

/// Configure bucket versioning.
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/zh/oss/developer-reference/putbucketversioning) for details.
///
/// 配置 Bucket 版本控制。
///
/// 详情见 [阿里云文档](https://help.aliyun.com/zh/oss/developer-reference/putbucketversioning)。
pub struct PutBucketVersioningSync {
    req: OssRequest,
    config: VersioningConfiguration,
}

impl PutBucketVersioningSync {
    pub(super) fn new(oss: Oss) -> Self {
        let mut req = OssRequest::new(oss, Method::PUT);
        req.insert_query("versioning", "");
        PutBucketVersioningSync { req, config: VersioningConfiguration::default() }
    }

    /// Set the versioning status.
    ///
    /// 设置版本控制状态。
    pub fn set_status(mut self, status: VersioningStatus) -> Self {
        self.config.status = status;
        self
    }

    /// Replace the entire configuration document.
    ///
    /// 替换完整配置文档。
    pub fn set_configuration(mut self, config: VersioningConfiguration) -> Self {
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
