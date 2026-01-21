use crate::{
    Error,
    common::body_to_bytes_sync,
    error::normal_error_sync,
    request_sync::{Oss, OssRequest},
};
use http::Method;
use serde_derive::Deserialize;

use super::put_bucket_cors_sync::CorsRule;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(rename = "CORSConfiguration")]
struct CorsConfiguration {
    #[serde(rename = "CORSRule", default)]
    rules: Vec<CorsRule>,
}

/// Retrieve the CORS rules of a bucket.
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/zh/oss/developer-reference/getbucketcors) for details.
///
/// 获取 Bucket 的 CORS 规则。
///
/// 详情参见 [阿里云文档](https://help.aliyun.com/zh/oss/developer-reference/getbucketcors)。
pub struct GetBucketCorsSync {
    req: OssRequest,
}

impl GetBucketCorsSync {
    pub(super) fn new(oss: Oss) -> Self {
        let mut req = OssRequest::new(oss, Method::GET);
        req.insert_query("cors", "");
        GetBucketCorsSync { req }
    }

    /// Send the request and return CORS rules.
    ///
    /// 发送请求并返回 CORS 规则。
    pub fn send(self) -> Result<Vec<CorsRule>, Error> {
        let response = self.req.send_to_oss()?;
        let status_code = response.status();
        match status_code {
            code if code.is_success() => {
                let response_bytes =
                    body_to_bytes_sync(response.into_body()).map_err(|_| Error::OssInvalidResponse(None))?;
                let cors: CorsConfiguration = serde_xml_rs::from_reader(&*response_bytes)
                    .map_err(|_| Error::OssInvalidResponse(Some(response_bytes)))?;
                Ok(cors.rules)
            }
            _ => Err(normal_error_sync(response)),
        }
    }
}
