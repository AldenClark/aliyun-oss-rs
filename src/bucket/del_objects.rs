use crate::{
    error::normal_error,
    request::{Oss, OssRequest},
    Error,
};
use base64::{engine::general_purpose, Engine};
use http::Method;
use bytes::Bytes;
use http_body_util::Full;
use md5::{Digest, Md5};
use std::collections::HashSet;

/// Delete multiple files
///
/// When deleting, OSS does not check whether the file exists; a valid request always succeeds
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/document_detail/31983.html) for details
pub struct DelObjects {
    req: OssRequest,
    objects: HashSet<String>,
}
impl DelObjects {
    pub(super) fn new(oss: Oss, files: Vec<impl ToString>) -> Self {
        let mut req = OssRequest::new(oss, Method::POST);
        req.insert_query("delete", "");
        let len = files.len();
        if len == 0 {
            DelObjects {
                req,
                objects: HashSet::new(),
            }
        } else {
            let mut objects = HashSet::with_capacity(len);
            for object in files {
                objects.insert(object.to_string());
            }
            DelObjects { req, objects }
        }
    }
    /// Add files to delete
    ///
    pub fn add_files(mut self, files: Vec<impl ToString>) -> Self {
        let len = files.len();
        if len == 0 {
            self
        } else {
            self.objects.reserve(len);
            for object in files {
                self.objects.insert(object.to_string());
            }
            self
        }
    }
    /// Send the request
    ///
    pub async fn send(mut self) -> Result<(), Error> {
        // Generate body
        let body = format!(
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?><Delete><Quiet>true</Quiet>{}</Delete>",
            self.objects
                .iter()
                .map(|v| format!("<Object><Key>{}</Key></Object>", v))
                .collect::<Vec<_>>()
                .join("")
        );
        // Calculate body length
        let body_len = body.len();
        // Calculate body MD5
        let mut hasher = Md5::new();
        hasher.update(&body);
        let result = hasher.finalize();
        let body_md5 = general_purpose::STANDARD.encode(&result);
        // Insert body content
        self.req.set_body(Full::new(Bytes::from(body)));
        // Insert header content
        self.req.insert_header("Content-Length", body_len);
        self.req.insert_header("Content-MD5", body_md5);
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
