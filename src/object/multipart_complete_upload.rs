use crate::{
    error::{Error, normal_error},
    request::{Oss, OssRequest},
};
use bytes::Bytes;
use http::Method;
use http_body_util::Full;

/// Complete the multipart upload
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/document_detail/31995.html) for details
pub struct CompleteUpload<'a> {
    req: OssRequest,
    parts: Vec<(&'a str, &'a str)>,
}
impl<'a> CompleteUpload<'a> {
    pub(super) fn new(oss: Oss, upload_id: impl ToString) -> Self {
        let mut req = OssRequest::new(oss, Method::POST);
        req.insert_query("uploadId", upload_id);
        CompleteUpload {
            req,
            parts: Vec::new(),
        }
    }
    /// Add part information
    ///
    /// Data structure: (PartNumber, ETag)
    pub fn add_parts(mut self, parts: Vec<(&'a str, &'a str)>) -> Self {
        self.parts.extend(parts);
        self
    }
    /// Complete the multipart upload
    ///
    pub async fn send(mut self) -> Result<(), Error> {
        // Build body
        let body = format!(
            "<CompleteMultipartUpload>{}</CompleteMultipartUpload>",
            self.parts
                .iter()
                .map(|(part_num, e_tag)| format!(
                    "<Part><PartNumber>{}</PartNumber><ETag>{}</ETag></Part>",
                    part_num, e_tag
                ))
                .collect::<Vec<_>>()
                .join("")
        );
        let body_len = body.len();
        self.req.set_body(Full::new(Bytes::from(body)));
        self.req.insert_header("Content-Length", body_len);
        // Upload file
        let response = self.req.send_to_oss()?.await?;
        // Parse the response
        let status_code = response.status();
        match status_code {
            code if code.is_success() => Ok(()),
            _ => Err(normal_error(response).await),
        }
    }
}
