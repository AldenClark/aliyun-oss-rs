use crate::{
    Error,
    request_sync::{Oss, OssRequest},
};
use base64::{Engine, engine::general_purpose};
use bytes::Bytes;
use http::Method;
use serde_derive::Deserialize;

// Returned content
/// Object metadata returned by `GetObjectMetaSync`.
///
/// `GetObjectMetaSync` 返回的对象元数据。
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ObjectMeta {
    /// Content length in bytes.
    ///
    /// 内容长度（字节）。
    pub content_length: String,
    /// Entity tag of the object.
    ///
    /// 对象的 ETag。
    pub e_tag: String,
    /// Last access time, if available.
    ///
    /// 最近访问时间（如果有）。
    pub last_access_time: Option<String>,
    /// Last modified time.
    ///
    /// 最近修改时间。
    pub last_modified: String,
}

/// Retrieve object metadata.
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/document_detail/31985.html) for details.
///
/// 获取对象元数据。
///
/// 详情参见 [阿里云文档](https://help.aliyun.com/document_detail/31985.html)。
pub struct GetObjectMetaSync {
    req: OssRequest,
}
impl GetObjectMetaSync {
    pub(super) fn new(oss: Oss) -> Self {
        let mut req = OssRequest::new(oss, Method::HEAD);
        req.insert_query("objectMeta", "");
        GetObjectMetaSync { req }
    }
    /// Send the request and return metadata.
    ///
    /// 发送请求并返回元数据。
    pub fn send(self) -> Result<ObjectMeta, Error> {
        // Build the HTTP request
        let response = self.req.send_to_oss()?;
        // Parse the response
        let status_code = response.status();
        match status_code {
            code if code.is_success() => {
                let headers = response.headers();
                let content_length = headers
                    .get("Content-Length")
                    .and_then(|header| header.to_str().ok().map(|s| s.to_owned()))
                    .unwrap_or_else(|| String::new());
                let e_tag = headers
                    .get("ETag")
                    .and_then(|header| header.to_str().ok().map(|s| s.trim_matches('"').to_owned()))
                    .unwrap_or_else(|| String::new());
                let last_access_time =
                    headers.get("x-oss-last-access-time").and_then(|header| header.to_str().ok().map(|s| s.to_owned()));
                let last_modified = headers
                    .get("Last-Modified")
                    .and_then(|header| header.to_str().ok().map(|s| s.to_owned()))
                    .unwrap_or_else(|| String::new());
                Ok(ObjectMeta { content_length, e_tag, last_access_time, last_modified })
            }
            _ => {
                let x_oss_error = response
                    .headers()
                    .get("x-oss-err")
                    .and_then(|header| general_purpose::STANDARD.decode(header).ok().map(|v| Bytes::from(v)));
                match x_oss_error {
                    None => Err(Error::OssInvalidError(status_code, Bytes::new())),
                    Some(response_bytes) => {
                        let oss_error = serde_xml_rs::from_reader(&*response_bytes);
                        match oss_error {
                            Ok(oss_error) => Err(Error::OssError(status_code, oss_error)),
                            Err(_) => Err(Error::OssInvalidError(status_code, response_bytes)),
                        }
                    }
                }
            }
        }
    }
}
