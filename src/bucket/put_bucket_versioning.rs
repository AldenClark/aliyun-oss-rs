use crate::{
    Error,
    error::normal_error,
    request::{Oss, OssRequest},
};
use bytes::Bytes;
use http::Method;
use http_body_util::Full;

use super::{VersioningConfiguration, VersioningStatus};

/// Configure bucket versioning.
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/zh/oss/developer-reference/putbucketversioning) for details.
///
/// 配置 Bucket 版本控制。
///
/// 详情见 [阿里云文档](https://help.aliyun.com/zh/oss/developer-reference/putbucketversioning)。
pub struct PutBucketVersioning {
    req: OssRequest,
    config: VersioningConfiguration,
}

impl PutBucketVersioning {
    pub(super) fn new(oss: Oss) -> Self {
        let mut req = OssRequest::new(oss, Method::PUT);
        req.insert_query("versioning", "");
        PutBucketVersioning { req, config: VersioningConfiguration::default() }
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
    pub async fn send(mut self) -> Result<(), Error> {
        let body = serde_xml_rs::to_string(&self.config).map_err(|_| Error::InvalidCharacter)?;
        self.req.set_body(Full::new(Bytes::from(body)));
        let response = self.req.send_to_oss()?.await?;
        match response.status() {
            code if code.is_success() => Ok(()),
            _ => Err(normal_error(response).await),
        }
    }
}
