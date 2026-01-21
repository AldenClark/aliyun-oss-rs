use crate::{
    Error,
    error::normal_error_sync,
    request_sync::{Oss, OssRequest},
};
use http::Method;

use super::{RequestPaymentConfiguration, RequestPayer};

/// Configure requester pays for a bucket.
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/zh/oss/developer-reference/putbucketrequestpayment) for details.
///
/// 配置 Bucket 请求者付费。
///
/// 详情见 [阿里云文档](https://help.aliyun.com/zh/oss/developer-reference/putbucketrequestpayment)。
pub struct PutBucketRequestPaymentSync {
    req: OssRequest,
    config: RequestPaymentConfiguration,
}

impl PutBucketRequestPaymentSync {
    pub(super) fn new(oss: Oss) -> Self {
        let mut req = OssRequest::new(oss, Method::PUT);
        req.insert_query("requestPayment", "");
        PutBucketRequestPaymentSync {
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
