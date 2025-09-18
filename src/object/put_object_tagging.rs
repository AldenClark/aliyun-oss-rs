use crate::{
    Error,
    error::normal_error,
    request::{Oss, OssRequest},
};
use bytes::Bytes;
use http::Method;
use http_body_util::Full;

/// Set object tags
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/document_detail/114855.html) for details
pub struct PutObjectTagging {
    req: OssRequest,
    tags: Vec<(String, String)>,
}
impl PutObjectTagging {
    pub(super) fn new(oss: Oss, tags: Vec<(impl ToString, impl ToString)>) -> Self {
        let mut req = OssRequest::new(oss, Method::PUT);
        req.insert_query("tagging", "");
        PutObjectTagging {
            req,
            tags: tags
                .iter()
                .map(|(key, value)| (key.to_string(), value.to_string()))
                .collect(),
        }
    }
    /// Add tags
    pub fn add_tags(mut self, tags: Vec<(impl ToString, impl ToString)>) -> Self {
        self.tags.extend(
            tags.iter()
                .map(|(key, value)| (key.to_string(), value.to_string())),
        );
        self
    }
    /// Send the request
    ///
    pub async fn send(mut self) -> Result<(), Error> {
        // Build body
        let tag_str = self
            .tags
            .iter()
            .map(|(key, value)| {
                if value.is_empty() {
                    format!("<Tag><Key>{}</Key></Tag>", key)
                } else {
                    format!("<Tag><Key>{}</Key><Value>{}</Value></Tag>", key, value)
                }
            })
            .collect::<Vec<_>>()
            .join("");
        let body = format!("<Tagging><TagSet>{}</TagSet></Tagging>", tag_str);
        self.req.insert_header("Content-Length", body.len());
        self.req.set_body(Full::new(Bytes::from(body)));
        // Build the HTTP request
        let response = self.req.send_to_oss()?.await?;
        // Parse the response
        let status_code = response.status();
        match status_code {
            code if code.is_success() => Ok(()),
            _ => Err(normal_error(response).await),
        }
    }
}
