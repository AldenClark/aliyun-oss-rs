use crate::common::format_gmt;
use crate::{
    Error,
    request::{Oss, OssRequest},
};
use base64::{Engine, engine::general_purpose};
use bytes::Bytes;
use http::Method;
use std::collections::HashMap;
use time::OffsetDateTime;

/// Retrieve the object's metadata
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/document_detail/31984.html) for details
pub struct HeadObject {
    req: OssRequest,
}
impl HeadObject {
    pub(super) fn new(oss: Oss) -> Self {
        HeadObject {
            req: OssRequest::new(oss, Method::HEAD),
        }
    }
    /// If the provided time is earlier than the actual modification time, the request succeeds
    ///
    pub fn set_if_modified_since(mut self, if_modified_since: OffsetDateTime) -> Self {
        self.req
            .insert_header("If-Modified-Since", format_gmt(if_modified_since));
        self
    }
    /// Require the specified time to be equal to or later than the object's last modification time
    ///
    pub fn set_if_unmodified_since(mut self, if_unmodified_since: OffsetDateTime) -> Self {
        self.req
            .insert_header("If-Unmodified-Since", format_gmt(if_unmodified_since));
        self
    }
    /// Require the object's ETag to match the provided ETag
    ///
    /// The ETag verifies whether the data has changed and can be used to check data integrity
    pub fn set_if_match(mut self, if_match: impl ToString) -> Self {
        self.req.insert_header("If-Match", if_match);
        self
    }
    /// Require the object's ETag to differ from the provided ETag
    ///
    pub fn set_if_none_match(mut self, if_none_match: impl ToString) -> Self {
        self.req.insert_header("If-None-Match", if_none_match);
        self
    }
    /// Send the request
    ///
    pub async fn send(self) -> Result<HashMap<String, String>, Error> {
        // Build the HTTP request
        let mut response = self.req.send_to_oss()?.await?;
        // Parse the response
        let status_code = response.status();
        match status_code {
            code if code.is_success() => {
                let headers = response.headers_mut();
                headers.remove("server");
                headers.remove("date");
                headers.remove("content-type");
                headers.remove("content-length");
                headers.remove("connection");
                headers.remove("x-oss-request-id");
                headers.remove("accept-ranges");
                let result = headers
                    .into_iter()
                    .map(|(key, value)| {
                        let key = key.to_string();
                        let mut value = String::from_utf8(value.as_bytes().to_vec())
                            .unwrap_or_else(|_| String::new());
                        if &key == "etag" {
                            value = value.trim_matches('"').to_owned();
                        }
                        (key, value)
                    })
                    .collect::<HashMap<String, String>>();
                Ok(result)
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
