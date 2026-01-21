use crate::{
    Error,
    common::body_to_bytes_sync,
    error::normal_error_sync,
    request_sync::{Oss, OssRequest},
};
use http::Method;

use super::WebsiteConfiguration;

/// Retrieve the static website configuration of a bucket.
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/zh/oss/developer-reference/getbucketwebsite) for details.
///
/// 获取 Bucket 静态网站配置。
///
/// 详情参见 [阿里云文档](https://help.aliyun.com/zh/oss/developer-reference/getbucketwebsite)。
pub struct GetBucketWebsiteSync {
    req: OssRequest,
}

impl GetBucketWebsiteSync {
    pub(super) fn new(oss: Oss) -> Self {
        let mut req = OssRequest::new(oss, Method::GET);
        req.insert_query("website", "");
        GetBucketWebsiteSync { req }
    }

    /// Send the request and return the parsed configuration.
    ///
    /// 发送请求并返回解析后的配置。
    pub fn send(self) -> Result<WebsiteConfiguration, Error> {
        let response = self.req.send_to_oss()?;
        match response.status() {
            code if code.is_success() => {
                let bytes = body_to_bytes_sync(response.into_body())?;
                let config: WebsiteConfiguration =
                    serde_xml_rs::from_reader(bytes.as_ref()).map_err(|_| Error::OssInvalidResponse(Some(bytes)))?;
                Ok(config)
            }
            _ => Err(normal_error_sync(response)),
        }
    }
}
