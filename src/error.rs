use bytes::Bytes;
use serde_derive::Deserialize;
use thiserror::Error;
use http::StatusCode;
#[cfg(feature = "async")]
use hyper::{body::Incoming, Response};
#[cfg(feature = "async")]
use hyper_util::client::legacy::Error as HyperClientError;
#[cfg(feature = "async")]
use crate::common::body_to_bytes;
#[cfg(feature = "sync")]
use std::io::Read;
#[cfg(feature = "sync")]
use ureq::{self, Body};

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    IoError(#[from] std::io::Error),
    #[error("不支持网络路径")]
    PathNotSupported,
    #[error("文件大小不符合要求")]
    InvalidFileSize,
    #[error("{0}")]
    HttpError(#[from] http::Error),
    #[cfg(feature = "async")]
    #[error("{0}")]
    HyperError(#[from] hyper::Error),
    #[cfg(feature = "async")]
    #[error("{0}")]
    HyperClientError(#[from] HyperClientError),
    #[cfg(feature = "sync")]
    #[error("{0}")]
    RequestError(#[from] ureq::Error),
    #[error("OSS返回了成功，但消息体结构解析失败，请尝试自行解析")]
    OssInvalidResponse(Option<Bytes>),
    #[error("{0} \n {1:#?}")]
    OssError(StatusCode, OssError),
    #[error("OSS返回了错误，HTTP状态码：{0}，错误内容请自行解析")]
    OssInvalidError(StatusCode, Bytes),
    #[error("使用了不符合要求的字符")]
    InvalidCharacter,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "Error")]
pub struct OssError {
    #[serde(rename = "Code")]
    pub code: String,
    #[serde(rename = "Message")]
    pub message: String,
    #[serde(rename = "RequestId")]
    pub request_id: String,
    #[serde(rename = "EC")]
    pub ec: String,
}

#[cfg(feature = "async")]
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
