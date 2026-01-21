use crate::common::body_to_bytes;
use crate::{
    Error,
    error::normal_error,
    request::{Oss, OssRequest},
};
use http::Method;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
/// Logging configuration information.
///
/// 日志配置信息。
pub struct LoggingEnabled {
    /// Target bucket storing logs.
    ///
    /// 存储日志的目标 Bucket。
    pub target_bucket: String,
    /// Log object key prefix.
    ///
    /// 日志对象前缀。
    pub target_prefix: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct BucketLoggingStatus {
    #[serde(rename = "LoggingEnabled")]
    pub logging_enabled: Option<LoggingEnabled>,
}

/// Retrieve the bucket logging configuration.
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/zh/oss/developer-reference/getbucketlogging) for details.
///
/// 获取 Bucket 日志配置。
///
/// 详情参见 [阿里云文档](https://help.aliyun.com/zh/oss/developer-reference/getbucketlogging)。
pub struct GetBucketLogging {
    req: OssRequest,
}
impl GetBucketLogging {
    pub(super) fn new(oss: Oss) -> Self {
        let mut req = OssRequest::new(oss, Method::GET);
        req.insert_query("logging", "");
        GetBucketLogging { req }
    }
    /// Send the request and return logging configuration if any.
    ///
    /// 发送请求并返回日志配置（如有）。
    pub async fn send(self) -> Result<Option<LoggingEnabled>, Error> {
        let response = self.req.send_to_oss()?.await?;
        let status_code = response.status();
        match status_code {
            code if code.is_success() => {
                let response_bytes = body_to_bytes(response.into_body())
                    .await
                    .map_err(|_| Error::OssInvalidResponse(None))?;
                let status: BucketLoggingStatus = serde_xml_rs::from_reader(&*response_bytes)
                    .map_err(|_| Error::OssInvalidResponse(Some(response_bytes)))?;
                Ok(status.logging_enabled)
            }
            _ => Err(normal_error(response).await),
        }
    }
}
