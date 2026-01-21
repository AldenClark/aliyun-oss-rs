use crate::{
    Error,
    error::normal_error,
    request::{Oss, OssRequest},
};
use bytes::Bytes;
use http::Method;
use http_body_util::Full;

/// Configure lifecycle rules for a bucket.
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/zh/oss/developer-reference/putbucketlifecycle) for details.
///
/// 配置 Bucket 生命周期规则。
///
/// 详情参见 [阿里云文档](https://help.aliyun.com/zh/oss/developer-reference/putbucketlifecycle)。
pub struct PutBucketLifecycle {
    req: OssRequest,
    body: Option<String>,
}

impl PutBucketLifecycle {
    pub(super) fn new(oss: Oss) -> Self {
        let mut req = OssRequest::new(oss, Method::PUT);
        req.insert_query("lifecycle", "");
        PutBucketLifecycle { req, body: None }
    }

    /// Provide the complete lifecycle configuration XML.
    ///
    /// The content must follow the OSS lifecycle XML schema.
    ///
    /// 提供完整的生命周期配置 XML。
    ///
    /// 内容需符合 OSS 生命周期 XML 规范。
    pub fn set_configuration(mut self, xml: impl Into<String>) -> Self {
        self.body = Some(xml.into());
        self
    }

    /// Send the request.
    ///
    /// 发送请求。
    pub async fn send(mut self) -> Result<(), Error> {
        let body = self.body.ok_or(Error::MissingRequestBody)?;
        self.req.set_body(Full::new(Bytes::from(body)));
        let response = self.req.send_to_oss()?.await?;
        match response.status() {
            code if code.is_success() => Ok(()),
            _ => Err(normal_error(response).await),
        }
    }
}
