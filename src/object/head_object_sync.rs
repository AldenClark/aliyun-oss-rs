use crate::common::format_gmt;
use crate::{
    Error,
    request_sync::{Oss, OssRequest},
};
use base64::{Engine, engine::general_purpose};
use bytes::Bytes;
use http::Method;
use std::collections::HashMap;
use time::OffsetDateTime;

/// Retrieve object metadata via HEAD.
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/document_detail/31984.html) for details.
///
/// 通过 HEAD 获取对象元数据。
///
/// 详情参见 [阿里云文档](https://help.aliyun.com/document_detail/31984.html)。
pub struct HeadObjectSync {
    req: OssRequest,
}
impl HeadObjectSync {
    pub(super) fn new(oss: Oss) -> Self {
        HeadObjectSync { req: OssRequest::new(oss, Method::HEAD) }
    }
    /// Succeed only if the object is modified after the given time.
    ///
    /// 仅当对象在指定时间之后被修改时才成功。
    pub fn set_if_modified_since(mut self, if_modified_since: OffsetDateTime) -> Self {
        self.req.insert_header("If-Modified-Since", format_gmt(if_modified_since));
        self
    }
    /// Succeed only if the object is not modified after the given time.
    ///
    /// 仅当对象在指定时间之后未被修改时才成功。
    pub fn set_if_unmodified_since(mut self, if_unmodified_since: OffsetDateTime) -> Self {
        self.req.insert_header("If-Unmodified-Since", format_gmt(if_unmodified_since));
        self
    }
    /// Succeed only if the object ETag matches the given value.
    ///
    /// ETag helps detect data changes and verify integrity.
    ///
    /// 仅当对象 ETag 与给定值一致时才成功。
    ///
    /// ETag 可用于检测数据变更和校验完整性。
    pub fn set_if_match(mut self, if_match: impl Into<String>) -> Self {
        self.req.insert_header("If-Match", if_match.into());
        self
    }
    /// Succeed only if the object ETag differs from the given value.
    ///
    /// 仅当对象 ETag 与给定值不一致时才成功。
    pub fn set_if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
        self.req.insert_header("If-None-Match", if_none_match.into());
        self
    }
    /// Send the request and return filtered response headers.
    ///
    /// 发送请求并返回过滤后的响应头。
    pub fn send(self) -> Result<HashMap<String, String>, Error> {
        // Build the HTTP request
        let mut response = self.req.send_to_oss()?;
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
                        let mut value = String::from_utf8(value.as_bytes().to_vec()).unwrap_or_else(|_| String::new());
                        if &key == "etag" {
                            value = value.trim_matches('"').to_owned();
                        }
                        (key, value)
                    })
                    .collect::<HashMap<String, String>>();
                Ok(result)
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
