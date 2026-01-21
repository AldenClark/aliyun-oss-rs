use crate::{
    error::{Error, normal_error_sync},
    request_sync::{Oss, OssRequest},
};
use http::{Method, header};
use std::fs::File;
use std::io::{BufReader, Read};
use ureq::SendBody;

/// Upload a part in a multipart upload (sync).
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/document_detail/31993.html) for details.
///
/// 上传分片（同步）。
///
/// 详情参见 [阿里云文档](https://help.aliyun.com/document_detail/31993.html)。
pub struct UploadPartSync {
    req: OssRequest,
    callback: Option<Box<dyn Fn(u64, u64) + Send + Sync + 'static>>,
}

struct ProgressReader<R> {
    inner: R,
    uploaded: u64,
    total: u64,
    callback: Option<Box<dyn Fn(u64, u64) + Send + Sync + 'static>>,
}

impl<R: Read> ProgressReader<R> {
    fn new(
        inner: R,
        total: u64,
        callback: Option<Box<dyn Fn(u64, u64) + Send + Sync + 'static>>,
    ) -> Self {
        ProgressReader {
            inner,
            uploaded: 0,
            total,
            callback,
        }
    }
}

impl<R: Read> Read for ProgressReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let size = self.inner.read(buf)?;
        if size > 0 {
            self.uploaded += size as u64;
            if let Some(callback) = &self.callback {
                callback(self.uploaded, self.total);
            }
        }
        Ok(size)
    }
}

impl UploadPartSync {
    pub(super) fn new(oss: Oss, part_number: u32, upload_id: impl Into<String>) -> Self {
        let mut req = OssRequest::new(oss, Method::PUT);
        req.insert_query("partNumber", part_number.to_string());
        req.insert_query("uploadId", upload_id.into());
        UploadPartSync {
            req,
            callback: None,
        }
    }
    /// Set an upload progress callback, only effective for `send_file()`.
    /// ```
    /// let callback = Box::new(|uploaded_size: u64, total_size: u64| {
    /// let percentage = if total_size == 0 {
    /// 100.0
    /// } else {
    /// (uploaded_size as f64) / (total_size as f64) * 100.00
    /// };
    /// println!("{:.2}%", percentage);
    /// });
    /// ```
    ///
    /// 设置上传进度回调，仅对 `send_file()` 生效。
    /// ```
    /// let callback = Box::new(|uploaded_size: u64, total_size: u64| {
    /// let percentage = if total_size == 0 {
    /// 100.0
    /// } else {
    /// (uploaded_size as f64) / (total_size as f64) * 100.00
    /// };
    /// println!("{:.2}%", percentage);
    /// });
    /// ```
    pub fn set_callback(mut self, callback: Box<dyn Fn(u64, u64) + Send + Sync + 'static>) -> Self {
        self.callback = Some(callback);
        self
    }
    /// Upload a local file as a part and return the ETag.
    ///
    /// 上传本地文件分片并返回 ETag。
    pub fn send_file(mut self, file: impl Into<String>) -> Result<String, Error> {
        let file = file.into();
        let file = File::open(&file)?;
        let file_size = file.metadata()?.len();
        if file_size >= 5_368_709_120 || file_size < 102_400 {
            return Err(Error::InvalidFileSize);
        }
        self.req
            .insert_header(header::CONTENT_LENGTH.as_str(), file_size.to_string());
        let reader = BufReader::with_capacity(131072, file);
        let reader: Box<dyn Read> = match self.callback {
            Some(callback) => Box::new(ProgressReader::new(reader, file_size, Some(callback))),
            None => Box::new(reader),
        };
        let body = SendBody::from_owned_reader(reader);
        let response = self.req.send_to_oss_with_body(body)?;
        let status_code = response.status();
        match status_code {
            code if code.is_success() => {
                let e_tag = response
                    .headers()
                    .get("ETag")
                    .map(|v| String::from_utf8(v.as_bytes().to_vec()).ok())
                    .flatten()
                    .unwrap_or_else(|| String::new());
                Ok(e_tag)
            }
            _ => Err(normal_error_sync(response)),
        }
    }
    /// Upload in-memory data as a part and return the ETag.
    ///
    /// 上传内存分片并返回 ETag。
    pub fn send_content(mut self, content: Vec<u8>) -> Result<String, Error> {
        let content_size = content.len() as u64;
        if content_size >= 5_000_000_000 {
            return Err(Error::InvalidFileSize);
        }
        self.req
            .insert_header(header::CONTENT_LENGTH.as_str(), content_size.to_string());
        self.req.set_body(content);
        let response = self.req.send_to_oss()?;
        let status_code = response.status();
        match status_code {
            code if code.is_success() => {
                let e_tag = response
                    .headers()
                    .get("ETag")
                    .map(|v| String::from_utf8(v.as_bytes().to_vec()).ok())
                    .flatten()
                    .unwrap_or_else(|| String::new());
                Ok(e_tag)
            }
            _ => Err(normal_error_sync(response)),
        }
    }
}
