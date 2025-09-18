use crate::{
    Error,
    request::{Oss, OssRequest},
};
use base64::{Engine, engine::general_purpose};
use bytes::Bytes;
use http::Method;
use serde_derive::Deserialize;

// Returned content
/// Object meta information
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ObjectMeta {
    /// File size in bytes
    pub content_length: String,
    /// Identifies the file content
    pub e_tag: String,
    /// Last access time
    pub last_access_time: Option<String>,
    /// Last modified time
    pub last_modified: String,
}

/// Retrieve the object's meta information
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/document_detail/31985.html) for details
pub struct GetObjectMeta {
    req: OssRequest,
}
impl GetObjectMeta {
    pub(super) fn new(oss: Oss) -> Self {
        let mut req = OssRequest::new(oss, Method::HEAD);
        req.insert_query("objectMeta", "");
        GetObjectMeta { req }
    }
    /// Send the request
    ///
    pub async fn send(self) -> Result<ObjectMeta, Error> {
        // Build the HTTP request
        let response = self.req.send_to_oss()?.await?;
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
                let last_access_time = headers
                    .get("x-oss-last-access-time")
                    .and_then(|header| header.to_str().ok().map(|s| s.to_owned()));
                let last_modified = headers
                    .get("Last-Modified")
                    .and_then(|header| header.to_str().ok().map(|s| s.to_owned()))
                    .unwrap_or_else(|| String::new());
                Ok(ObjectMeta {
                    content_length,
                    e_tag,
                    last_access_time,
                    last_modified,
                })
            }
            _ => {
                let x_oss_error = response.headers().get("x-oss-err").and_then(|header| {
                    general_purpose::STANDARD
                        .decode(header)
                        .ok()
                        .map(|v| Bytes::from(v))
                });
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
