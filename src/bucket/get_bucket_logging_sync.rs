use crate::{
    error::{Error, normal_error_sync},
    request_sync::{Oss, OssRequest},
};
use bytes::Bytes;
use http::Method;
use serde_derive::Deserialize;
use std::io::Read;

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

/// Retrieve the bucket logging configuration (sync).
///
/// 获取 Bucket 日志配置（同步）。
pub struct GetBucketLoggingSync {
    req: OssRequest,
}
impl GetBucketLoggingSync {
    pub(crate) fn new(oss: Oss) -> Self {
        let mut req = OssRequest::new(oss, Method::GET);
        req.insert_query("logging", "");
        GetBucketLoggingSync { req }
    }
    /// Send the request and return logging configuration if any.
    ///
    /// 发送请求并返回日志配置（如有）。
    pub fn send(self) -> Result<Option<LoggingEnabled>, Error> {
        let response = self.req.send_to_oss()?;
        let status = response.status();
        if status.is_success() {
            let mut reader = response.into_body().into_reader();
            let mut buf = Vec::new();
            reader.read_to_end(&mut buf)?;
            let status: BucketLoggingStatus =
                serde_xml_rs::from_reader(&*buf).map_err(|_| Error::OssInvalidResponse(Some(Bytes::from(buf))))?;
            Ok(status.logging_enabled)
        } else {
            Err(normal_error_sync(response))
        }
    }
}
