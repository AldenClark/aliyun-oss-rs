use crate::{
    Error,
    error::normal_error,
    request::{Oss, OssRequest},
};
use bytes::Bytes;
use http::Method;
use http_body_util::Full;

/// Set tags for an object.
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/document_detail/114855.html) for details.
///
/// 设置对象标签。
///
/// 详情参见 [阿里云文档](https://help.aliyun.com/document_detail/114855.html)。
pub struct PutObjectTagging {
    req: OssRequest,
    tags: Vec<(String, String)>,
}
impl PutObjectTagging {
    pub(super) fn new(oss: Oss, tags: Vec<(impl Into<String>, impl Into<String>)>) -> Self {
        let mut req = OssRequest::new(oss, Method::PUT);
        req.insert_query("tagging", "");
        PutObjectTagging { req, tags: tags.into_iter().map(|(key, value)| (key.into(), value.into())).collect() }
    }
    /// Add tags.
    ///
    /// 追加标签。
    pub fn add_tags(mut self, tags: Vec<(impl Into<String>, impl Into<String>)>) -> Self {
        self.tags.extend(tags.into_iter().map(|(key, value)| (key.into(), value.into())));
        self
    }
    /// Send the request.
    ///
    /// 发送请求。
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
        self.req.insert_header("Content-Length", body.len().to_string());
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
