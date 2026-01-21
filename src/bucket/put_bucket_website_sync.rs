use crate::{
    Error,
    error::normal_error_sync,
    request_sync::{Oss, OssRequest},
};
use http::Method;

use super::{ErrorDocument, IndexDocument, WebsiteConfiguration};

/// Configure bucket static website hosting.
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/zh/oss/developer-reference/putbucketwebsite) for details.
///
/// 配置 Bucket 静态网站托管。
///
/// 详情参见 [阿里云文档](https://help.aliyun.com/zh/oss/developer-reference/putbucketwebsite)。
pub struct PutBucketWebsiteSync {
    req: OssRequest,
    config: WebsiteConfiguration,
}

impl PutBucketWebsiteSync {
    pub(super) fn new(oss: Oss) -> Self {
        let mut req = OssRequest::new(oss, Method::PUT);
        req.insert_query("website", "");
        PutBucketWebsiteSync { req, config: WebsiteConfiguration::default() }
    }

    /// Set the index document suffix (e.g., `index.html`).
    ///
    /// 设置索引文档后缀（如 `index.html`）。
    pub fn set_index_document(mut self, suffix: impl Into<String>) -> Self {
        self.config.index_document = Some(IndexDocument { suffix: suffix.into() });
        self
    }

    /// Set the error document key (e.g., `error.html`).
    ///
    /// 设置错误文档 Key（如 `error.html`）。
    pub fn set_error_document(mut self, key: impl Into<String>) -> Self {
        self.config.error_document = Some(ErrorDocument { key: key.into() });
        self
    }

    /// Replace the entire configuration object.
    ///
    /// 替换完整配置对象。
    pub fn set_configuration(mut self, config: WebsiteConfiguration) -> Self {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_website_serialization() {
        let config = WebsiteConfiguration {
            index_document: Some(IndexDocument { suffix: "index.html".into() }),
            error_document: Some(ErrorDocument { key: "error.html".into() }),
            routing_rules: None,
        };
        let xml = serde_xml_rs::to_string(&config).unwrap();
        assert!(xml.contains("<WebsiteConfiguration>"));
        assert!(xml.contains("<Suffix>index.html</Suffix>"));
        assert!(xml.contains("<Key>error.html</Key>"));
    }
}
