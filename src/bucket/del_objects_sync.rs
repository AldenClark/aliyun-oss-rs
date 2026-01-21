use crate::{
    Error,
    error::normal_error_sync,
    request_sync::{Oss, OssRequest},
};
use base64::{Engine, engine::general_purpose};
use http::Method;
use md5::{Digest, Md5};
use std::collections::HashSet;

/// Delete multiple objects.
///
/// OSS does not check object existence; a valid request usually succeeds.
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/document_detail/31983.html) for details.
///
/// 批量删除对象。
///
/// OSS 不检查对象是否存在，合法请求通常会成功。
///
/// 详情参见 [阿里云文档](https://help.aliyun.com/document_detail/31983.html)。
pub struct DelObjectsSync {
    req: OssRequest,
    objects: HashSet<String>,
}
impl DelObjectsSync {
    pub(super) fn new(oss: Oss, files: Vec<impl Into<String>>) -> Self {
        let mut req = OssRequest::new(oss, Method::POST);
        req.insert_query("delete", "");
        let len = files.len();
        if len == 0 {
            DelObjectsSync {
                req,
                objects: HashSet::new(),
            }
        } else {
            let mut objects = HashSet::with_capacity(len);
            for object in files {
                objects.insert(object.into());
            }
            DelObjectsSync { req, objects }
        }
    }
    /// Add objects to delete.
    ///
    /// 添加待删除对象。
    pub fn add_files(mut self, files: Vec<impl Into<String>>) -> Self {
        let len = files.len();
        if len == 0 {
            self
        } else {
            self.objects.reserve(len);
            for object in files {
                self.objects.insert(object.into());
            }
            self
        }
    }
    /// Send the request.
    ///
    /// 发送请求。
    pub fn send(mut self) -> Result<(), Error> {
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
        self.req.set_body(body.into_bytes());
        // Insert header content
        self.req
            .insert_header("Content-Length", body_len.to_string());
        self.req.insert_header("Content-MD5", body_md5);
        // Build the HTTP request
        let response = self.req.send_to_oss()?;
        // Parse the response
        let status_code = response.status();
        match status_code {
            code if code.is_success() => Ok(()),
            _ => Err(normal_error_sync(response)),
        }
    }
}
