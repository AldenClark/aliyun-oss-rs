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

/// Initialize a multipart upload part
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/document_detail/31993.html) for details
pub struct UploadPart {
    req: OssRequest,
    callback: Option<Box<dyn Fn(u64, u64) + Send + Sync + 'static>>,
}
impl UploadPart {
    pub(super) fn new(oss: Oss, part_number: u32, upload_id: impl ToString) -> Self {
        let mut req = OssRequest::new(oss, Method::PUT);
        req.insert_query("partNumber", part_number);
        req.insert_query("uploadId", upload_id);
        UploadPart {
            req,
            callback: None,
        }
    }
    /// Set a callback for upload progress; this only applies to `send_file()`
    /// ```
    /// let callback = Box::new(|uploaded_size: u64, total_size: u64| {
    ///     let percentage = if total_size == 0 {
    ///         100.0
    ///     } else {
    ///         (uploaded_size as f64) / (total_size as f64) * 100.00
    ///     };
    ///     println!("{:.2}%", percentage);
    /// });
    /// ```
    pub fn set_callback(mut self, callback: Box<dyn Fn(u64, u64) + Send + Sync + 'static>) -> Self {
        self.callback = Some(callback);
        self
    }
    /// Upload a file from disk to OSS
    ///
    /// Returns the ETag
    pub async fn send_file(mut self, file: impl ToString) -> Result<String, Error> {
        // Open the file
        let file = File::open(file.to_string()).await?;
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
    /// Upload in-memory data to OSS
    ///
    /// Returns the ETag
    pub async fn send_content(mut self, content: Vec<u8>) -> Result<String, Error> {
        // Read size
        let content_size = content.len() as u64;
        if content_size >= 5_000_000_000 {
            return Err(Error::InvalidFileSize);
        }
        self.req.insert_header(header::CONTENT_LENGTH, content_size);
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
