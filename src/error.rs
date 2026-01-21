#[cfg(feature = "async")]
use crate::common::body_to_bytes;
use bytes::Bytes;
use http::StatusCode;
#[cfg(feature = "async")]
use hyper::{Response, body::Incoming};
#[cfg(feature = "async")]
use hyper_util::client::legacy::Error as HyperClientError;
use serde_derive::Deserialize;
#[cfg(feature = "sync")]
use std::io::Read;
use thiserror::Error;
#[cfg(feature = "sync")]
use ureq::{self, Body};

/// Error type for OSS operations.
///
/// OSS 操作错误类型。
#[derive(Error, Debug)]
pub enum Error {
    /// I/O error.
    ///
    /// I/O 错误。
    #[error("{0}")]
    IoError(#[from] std::io::Error),
    /// Network path is not supported.
    ///
    /// 不支持网络路径。
    #[error("不支持网络路径")]
    PathNotSupported,
    /// Invalid file size.
    ///
    /// 文件大小不符合要求。
    #[error("文件大小不符合要求")]
    InvalidFileSize,
    /// HTTP error while building requests.
    ///
    /// 构建请求时的 HTTP 错误。
    #[error("{0}")]
    HttpError(#[from] http::Error),
    #[cfg(feature = "async")]
    /// Hyper error (async).
    ///
    /// Hyper 错误（异步）。
    #[error("{0}")]
    HyperError(#[from] hyper::Error),
    #[cfg(feature = "async")]
    /// Hyper client error (async).
    ///
    /// Hyper 客户端错误（异步）。
    #[error("{0}")]
    HyperClientError(#[from] HyperClientError),
    #[cfg(feature = "sync")]
    /// Ureq error (sync).
    ///
    /// Ureq 错误（同步）。
    #[error("{0}")]
    RequestError(#[from] ureq::Error),
    /// OSS returned success but response body parsing failed.
    ///
    /// OSS 返回成功但响应体解析失败。
    #[error("OSS返回了成功，但消息体结构解析失败，请尝试自行解析")]
    OssInvalidResponse(Option<Bytes>),
    /// OSS returned an error with a parsed body.
    ///
    /// OSS 返回错误，且错误体已解析。
    #[error("{0} \n {1:#?}")]
    OssError(StatusCode, OssError),
    /// OSS returned an error but body parsing failed.
    ///
    /// OSS 返回错误，但错误体解析失败。
    #[error("OSS返回了错误，HTTP状态码：{0}，错误内容请自行解析")]
    OssInvalidError(StatusCode, Bytes),
    /// Invalid characters in request payload.
    ///
    /// 请求内容包含非法字符。
    #[error("使用了不符合要求的字符")]
    InvalidCharacter,
    /// Request is missing a required body.
    ///
    /// 请求缺少必要的消息体。
    #[error("请求缺少必要的消息体，请检查调用参数")]
    MissingRequestBody,
}

/// Structured OSS error response.
///
/// OSS 错误响应结构。
#[derive(Debug, Deserialize)]
#[serde(rename = "Error")]
pub struct OssError {
    /// Error code.
    ///
    /// 错误码。
    #[serde(rename = "Code")]
    pub code: String,
    /// Error message.
    ///
    /// 错误信息。
    #[serde(rename = "Message")]
    pub message: String,
    /// Request ID.
    ///
    /// 请求 ID。
    #[serde(rename = "RequestId")]
    pub request_id: String,
    /// Extended code (EC).
    ///
    /// 扩展错误码（EC）。
    #[serde(rename = "EC")]
    pub ec: String,
}

#[cfg(feature = "async")]
/// Convert an async OSS response into an `Error`.
///
/// 将异步 OSS 响应转换为 `Error`。
pub async fn normal_error(response: Response<Incoming>) -> Error {
    let status_code = response.status();
    let response_bytes = body_to_bytes(response.into_body()).await;
    match response_bytes {
        Err(e) => Error::HyperError(e),
        Ok(response_bytes) => {
            let oss_error = serde_xml_rs::from_reader(response_bytes.as_ref());
            match oss_error {
                Ok(oss_error) => Error::OssError(status_code, oss_error),
                Err(_) => Error::OssInvalidError(status_code, response_bytes),
            }
        }
    }
}

#[cfg(feature = "sync")]
/// Convert a sync OSS response into an `Error`.
///
/// 将同步 OSS 响应转换为 `Error`。
pub fn normal_error_sync(response: http::Response<Body>) -> Error {
    let status_code = response.status();
    let mut reader = response.into_body().into_reader();
    let mut buf = Vec::new();
    if let Err(e) = reader.read_to_end(&mut buf) {
        return Error::IoError(e);
    }
    let bytes = Bytes::from(buf);
    match serde_xml_rs::from_reader(bytes.as_ref()) {
        Ok(oss_error) => Error::OssError(status_code, oss_error),
        Err(_) => Error::OssInvalidError(status_code, bytes),
    }
}
