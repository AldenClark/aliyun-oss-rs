use crate::{
    common::{
        Acl, CacheControl, ContentDisposition, StorageClass, invalid_metadata_key, url_encode,
    },
    error::{Error, normal_error},
    request::{Oss, OssRequest},
};
use bytes::Bytes;
use futures_util::StreamExt;
use http::{Method, header};
use http_body::Frame;
use http_body_util::{Full, StreamBody};
use std::collections::HashMap;
use tokio::{fs::File, io::BufReader};
use tokio_util::io::ReaderStream;

/// Upload a file
///
/// The object size cannot exceed 5GB
///
/// By default, if an object with the same name exists and you have access, the new object overwrites it
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/document_detail/31978.html) for details
pub struct PutObject {
    req: OssRequest,
    mime: Option<String>,
    tags: HashMap<String, String>,
    callback: Option<Box<dyn Fn(u64, u64) + Send + Sync + 'static>>,
}
impl PutObject {
    pub(super) fn new(oss: Oss) -> Self {
        PutObject {
            req: OssRequest::new(oss, Method::PUT),
            mime: None,
            tags: HashMap::new(),
            callback: None,
        }
    }
    /// Set the file's MIME type
    ///
    /// If no MIME type is set, the request will attempt to obtain it from the content, local path, or remote path; if still not found, the default MIME type (application/octet-stream) is used
    pub fn set_mime(mut self, mime: impl ToString) -> Self {
        self.mime = Some(mime.to_string());
        self
    }
    /// Set the file's access permissions
    pub fn set_acl(mut self, acl: Acl) -> Self {
        self.req.insert_header("x-oss-object-acl", acl);
        self
    }
    /// Set the file's storage class
    pub fn set_storage_class(mut self, storage_class: StorageClass) -> Self {
        self.req.insert_header("x-oss-storage-class", storage_class);
        self
    }
    /// Cache behavior of the webpage when the file is downloaded
    pub fn set_cache_control(mut self, cache_control: CacheControl) -> Self {
        self.req.insert_header(header::CACHE_CONTROL, cache_control);
        self
    }
    /// Set how the file is presented
    pub fn set_content_disposition(mut self, content_disposition: ContentDisposition) -> Self {
        self.req
            .insert_header(header::CONTENT_DISPOSITION, content_disposition);
        self
    }
    /// Disallow overwriting files with the same name
    pub fn forbid_overwrite(mut self) -> Self {
        self.req.insert_header("x-oss-forbid-overwrite", "true");
        self
    }
    /// Set additional metadata
    ///
    /// Keys may only contain letters, numbers, and hyphens; metadata with other characters will be discarded
    pub fn set_meta(mut self, key: impl ToString, value: impl ToString) -> Self {
        let key = key.to_string();
        if !invalid_metadata_key(&key) {
            self.req
                .insert_header(format!("x-oss-meta-{}", key.to_string()), value);
        }
        self
    }
    /// Set tag information
    pub fn set_tagging(mut self, key: impl ToString, value: impl ToString) -> Self {
        self.tags.insert(key.to_string(), value.to_string());
        self
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
    pub async fn send_file(mut self, file: impl ToString) -> Result<(), Error> {
        // Determine file MIME type
        let file_type = match self.mime {
            Some(mime) => mime,
            None => match infer::get_from_path(&file.to_string())? {
                Some(ext) => ext.mime_type().to_owned(),
                None => mime_guess::from_path(
                    &self
                        .req
                        .oss
                        .object
                        .clone()
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| String::new()),
                )
                .first()
                .map(|v| v.to_string())
                .unwrap_or_else(|| "application/octet-stream".to_owned())
                .to_string(),
            },
        };
        self.req.insert_header(header::CONTENT_TYPE, file_type);
        // Insert tags
        let tags = self
            .tags
            .into_iter()
            .map(|(key, value)| {
                if value.is_empty() {
                    url_encode(&key.to_string())
                } else {
                    format!(
                        "{}={}",
                        url_encode(&key.to_string()),
                        url_encode(&value.to_string())
                    )
                }
            })
            .collect::<Vec<_>>()
            .join("&");
        if !tags.is_empty() {
            self.req.insert_header("x-oss-tagging", tags);
        }
        // Open the file
        let file = File::open(file.to_string()).await?;
        // Read the file size
        let file_size = file.metadata().await?.len();
        if file_size >= 5_368_709_120 {
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
            code if code.is_success() => Ok(()),
            _ => Err(normal_error(response).await),
        }
    }
    /// Upload in-memory data to OSS
    pub async fn send_content(mut self, content: Vec<u8>) -> Result<(), Error> {
        // Determine file MIME type
        let content_type = match self.mime {
            Some(mime) => mime,
            None => match infer::get(&content) {
                Some(ext) => ext.mime_type().to_string(),
                None => mime_guess::from_path(
                    self.req
                        .oss
                        .object
                        .clone()
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| String::new().into()),
                )
                .first()
                .map(|v| v.to_string())
                .unwrap_or_else(|| "application/octet-stream".to_owned())
                .to_string(),
            },
        };
        self.req.insert_header(header::CONTENT_TYPE, content_type);
        // Insert tags
        let tags = self
            .tags
            .into_iter()
            .map(|(key, value)| {
                if value.is_empty() {
                    url_encode(&key.to_string())
                } else {
                    format!(
                        "{}={}",
                        url_encode(&key.to_string()),
                        url_encode(&value.to_string())
                    )
                }
            })
            .collect::<Vec<_>>()
            .join("&");
        if !tags.is_empty() {
            self.req.insert_header("x-oss-tagging", tags);
        }
        // Read size
        let content_size = content.len() as u64;
        if content_size >= 5_368_709_120 {
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
            code if code.is_success() => Ok(()),
            _ => Err(normal_error(response).await),
        }
    }
}
