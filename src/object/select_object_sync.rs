use crate::{
    Error,
    common::body_to_bytes_sync,
    error::normal_error_sync,
    request_sync::{Oss, OssRequest},
};
use bytes::Bytes;
use http::Method;

/// Execute an SQL-like query against an object stored in OSS (sync).
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/zh/oss/developer-reference/selectobject) for details.
///
/// 对 OSS 中的对象执行类 SQL 查询（同步）。
///
/// 详情参见 [阿里云文档](https://help.aliyun.com/zh/oss/developer-reference/selectobject)。
pub struct SelectObjectSync {
    req: OssRequest,
    request_xml: Option<String>,
}

impl SelectObjectSync {
    pub(super) fn new(oss: Oss) -> Self {
        let mut req = OssRequest::new(oss, Method::POST);
        req.insert_query("select", "");
        req.insert_query("select-type", "2");
        SelectObjectSync { req, request_xml: None }
    }

    /// Provide the select request XML document.
    ///
    /// Refer to the official documentation for the `<SelectRequest>` schema.
    ///
    /// 提供查询请求 XML 文档。
    ///
    /// `<SelectRequest>` 结构请参考官方文档。
    pub fn set_request(mut self, xml: impl Into<String>) -> Self {
        self.request_xml = Some(xml.into());
        self
    }

    /// Enable raw output mode (sets `x-oss-select-output-raw`).
    ///
    /// 启用原始输出模式（设置 `x-oss-select-output-raw`）。
    pub fn enable_output_raw(mut self, enable: bool) -> Self {
        if enable {
            self.req.insert_header("x-oss-select-output-raw", "true");
        }
        self
    }

    /// Send the request and collect the response into memory.
    ///
    /// 发送请求并将响应聚合到内存。
    pub fn send(self) -> Result<Bytes, Error> {
        let response = self.send_internal()?;
        Ok(body_to_bytes_sync(response.into_body())?)
    }

    /// Send the request and return a blocking reader.
    ///
    /// 发送请求并返回阻塞读取器。
    pub fn send_to_reader(self) -> Result<ureq::BodyReader<'static>, Error> {
        let response = self.send_internal()?;
        Ok(response.into_body().into_reader())
    }

    fn send_internal(mut self) -> Result<http::Response<ureq::Body>, Error> {
        let body = self.request_xml.ok_or(Error::MissingRequestBody)?;
        self.req.set_body(body.into_bytes());
        let response = self.req.send_to_oss()?;
        let status_code = response.status();
        if status_code.is_success() { Ok(response) } else { Err(normal_error_sync(response)) }
    }
}
