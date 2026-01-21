use crate::{
    error::{Error, normal_error},
    request::{Oss, OssRequest},
};
use bytes::Bytes;
use futures_util::StreamExt;
use http::{Method, header};
use http_body::Frame;
use http_body_util::{Full, StreamBody};
use tokio::{fs::File, io::BufReader};
use tokio_util::io::ReaderStream;

/// Upload a part in a multipart upload.
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/document_detail/31993.html) for details.
///
/// 上传分片。
///
/// 详情参见 [阿里云文档](https://help.aliyun.com/document_detail/31993.html)。
pub struct UploadPart {
    req: OssRequest,
    callback: Option<Box<dyn Fn(u64, u64) + Send + Sync + 'static>>,
}
impl UploadPart {
    pub(super) fn new(oss: Oss, part_number: u32, upload_id: impl Into<String>) -> Self {
        let mut req = OssRequest::new(oss, Method::PUT);
        req.insert_query("partNumber", part_number.to_string());
        req.insert_query("uploadId", upload_id.into());
        UploadPart { req, callback: None }
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
    pub async fn send_file(mut self, file: impl Into<String>) -> Result<String, Error> {
        let file = file.into();
        // Open the file
        let file = File::open(&file).await?;
        // Read the file size
        let file_size = file.metadata().await?.len();
        if file_size >= 5_368_709_120 || file_size < 102_400 {
            return Err(Error::InvalidFileSize);
        }
        // Initialize the data stream for reading file content
        let buf = BufReader::with_capacity(131072, file);
        let stream = ReaderStream::with_capacity(buf, 16384);
        // Initialize the uploaded content size
        let mut uploaded_size = 0;
        // Initialize upload request
        let body = StreamBody::new(stream.map(move |result| match result {
            Ok(chunk) => {
                if let Some(callback) = &self.callback {
                    let upload_size = chunk.len() as u64;
                    uploaded_size += upload_size;
                    callback(uploaded_size, file_size);
                }
                Ok(Frame::data(chunk))
            }
            Err(err) => Err(err),
        }));
        self.req.set_body(body);
        // Upload file
        let response = self.req.send_to_oss()?.await?;
        // Parse the response
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
            _ => Err(normal_error(response).await),
        }
    }
    /// Upload in-memory data as a part and return the ETag.
    ///
    /// 上传内存分片并返回 ETag。
    pub async fn send_content(mut self, content: Vec<u8>) -> Result<String, Error> {
        // Read size
        let content_size = content.len() as u64;
        if content_size >= 5_000_000_000 {
            return Err(Error::InvalidFileSize);
        }
        self.req.insert_header(header::CONTENT_LENGTH.as_str(), content_size.to_string());
        // Insert body
        self.req.set_body(Full::new(Bytes::from(content)));
        // Upload file
        let response = self.req.send_to_oss()?.await?;
        // Parse the response
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
            _ => Err(normal_error(response).await),
        }
    }
}
