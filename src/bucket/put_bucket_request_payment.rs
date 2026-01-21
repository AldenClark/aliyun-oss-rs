use crate::{
    Error,
    error::normal_error,
    request::{Oss, OssRequest},
};
use bytes::Bytes;
use http::Method;
use http_body_util::Full;

use super::{RequestPaymentConfiguration, RequestPayer};

/// Configure requester pays for a bucket.
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/zh/oss/developer-reference/putbucketrequestpayment) for details.
///
/// 配置 Bucket 请求者付费。
///
/// 详情见 [阿里云文档](https://help.aliyun.com/zh/oss/developer-reference/putbucketrequestpayment)。
pub struct PutBucketRequestPayment {
    req: OssRequest,
    config: RequestPaymentConfiguration,
}

impl PutBucketRequestPayment {
    pub(super) fn new(oss: Oss) -> Self {
        let mut req = OssRequest::new(oss, Method::PUT);
        req.insert_query("requestPayment", "");
        PutBucketRequestPayment {
            req,
            config: RequestPaymentConfiguration {
                payer: RequestPayer::default(),
            },
        }
    }

    /// Set who pays for the requests.
    ///
    /// 设置请求付费方。
    pub fn set_payer(mut self, payer: RequestPayer) -> Self {
        self.config.payer = payer;
        self
    }

    /// Replace the entire configuration document.
    ///
    /// 替换完整配置文档。
    pub fn set_configuration(mut self, config: RequestPaymentConfiguration) -> Self {
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
