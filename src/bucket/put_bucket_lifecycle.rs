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

    /// Provide the complete lifecycle configuration document.
    ///
    /// The content must follow the OSS lifecycle XML schema.
    pub fn set_configuration(mut self, xml: impl ToString) -> Self {
        self.body = Some(xml.to_string());
        self
    }

    /// Send the request to OSS.
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
