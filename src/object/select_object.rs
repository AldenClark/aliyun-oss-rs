use crate::{
    Error,
    common::body_to_bytes,
    error::normal_error,
    request::{Oss, OssRequest},
};
use bytes::Bytes;
use futures_util::{Stream, StreamExt};
use http::Method;
use http_body_util::{BodyExt, Full};
use hyper::{Response, body::Incoming};
use std::pin::Pin;

/// Execute an SQL-like query against an object stored in OSS.
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/zh/oss/developer-reference/selectobject) for details.
pub struct SelectObject {
    req: OssRequest,
    request_xml: Option<String>,
}

impl SelectObject {
    pub(super) fn new(oss: Oss) -> Self {
        let mut req = OssRequest::new(oss, Method::POST);
        req.insert_query("select", "");
        req.insert_query("select-type", "2");
        SelectObject {
            req,
            request_xml: None,
        }
    }

    /// Provide the select request XML document.
    ///
    /// Refer to the official documentation for the schema of the `<SelectRequest>` payload.
    pub fn set_request(mut self, xml: impl ToString) -> Self {
        self.request_xml = Some(xml.to_string());
        self
    }

    /// Enable raw output mode (sets the `x-oss-select-output-raw` header).
    pub fn enable_output_raw(mut self, enable: bool) -> Self {
        if enable {
            self.req.insert_header("x-oss-select-output-raw", "true");
        }
        self
    }

    /// Send the request and collect the response into memory.
    pub async fn send(self) -> Result<Bytes, Error> {
        let response = self.send_internal().await?;
        Ok(body_to_bytes(response.into_body()).await?)
    }

    /// Send the request and return the response stream for manual consumption.
    pub async fn send_to_stream(
        self,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<bytes::Bytes, Error>> + Send>>, Error> {
        let response = self.send_internal().await?;
        let status_code = response.status();
        match status_code {
            code if code.is_success() => {
                let stream = response
                    .into_body()
                    .into_data_stream()
                    .map(|item| match item {
                        Ok(bytes) => Ok(bytes),
                        Err(e) => Err(e.into()),
                    });
                Ok(Box::pin(stream))
            }
            _ => Err(normal_error(response).await),
        }
    }

    async fn send_internal(mut self) -> Result<Response<Incoming>, Error> {
        let body = self.request_xml.ok_or(Error::MissingRequestBody)?;
        self.req.set_body(Full::new(Bytes::from(body)));
        let response = self.req.send_to_oss()?.await?;
        let status_code = response.status();
        if status_code.is_success() {
            Ok(response)
        } else {
            Err(normal_error(response).await)
        }
    }
}
