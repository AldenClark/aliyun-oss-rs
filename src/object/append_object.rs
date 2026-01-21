use crate::{
    common::{Acl, CacheControl, ContentDisposition, StorageClass, invalid_metadata_key, url_encode},
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

/// Append data to an appendable object.
///
/// Only appendable objects can be appended; objects uploaded via `PutObject` cannot be appended.
///
/// The final object size after appends cannot exceed 5GB.
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/document_detail/31978.html) for details and limitations.
///
/// 追加数据到可追加类型对象。
///
/// 仅可追加类型对象支持追加；使用 `PutObject` 上传的对象不可追加。
///
/// 追加后的对象最终大小不能超过 5GB。
///
/// 详细规则与限制请参阅 [阿里云文档](https://help.aliyun.com/document_detail/31978.html)。
pub struct AppendObject {
    req: OssRequest,
    mime: Option<String>,
    tags: HashMap<String, String>,
    callback: Option<Box<dyn Fn(u64, u64) + Send + Sync + 'static>>,
}

impl AppendObject {
    pub(super) fn new(oss: Oss) -> Self {
        let mut req = OssRequest::new(oss, Method::POST);
        req.insert_query("append", "");
        req.insert_query("position", "0");
        AppendObject { req, mime: None, tags: HashMap::new(), callback: None }
    }
    /// Set the starting position for the append content.
    ///
    /// 设置追加内容的起始位置。
    pub fn set_position(mut self, position: u32) -> Self {
        self.req.insert_query("position", position.to_string());
        self
    }
    /// Set the object's MIME type.
    ///
    /// If not set, the MIME type is inferred; fallback is `application/octet-stream`.
    ///
    /// 设置对象的 MIME 类型。
    ///
    /// 未设置时尝试推断，失败则使用 `application/octet-stream`。
    pub fn set_mime(mut self, mime: impl Into<String>) -> Self {
        self.mime = Some(mime.into());
        self
    }
    /// Set object ACL.
    ///
    /// 设置对象 ACL。
    pub fn set_acl(mut self, acl: Acl) -> Self {
        self.req.insert_header("x-oss-object-acl", acl.to_string());
        self
    }
    /// Set object storage class.
    ///
    /// 设置对象存储类型。
    pub fn set_storage_class(mut self, storage_class: StorageClass) -> Self {
        self.req.insert_header("x-oss-storage-class", storage_class.to_string());
        self
    }
    /// Set cache-control behavior when the object is downloaded.
    ///
    /// 设置对象下载时的缓存策略。
    pub fn set_cache_control(mut self, cache_control: CacheControl) -> Self {
        self.req.insert_header(header::CACHE_CONTROL.as_str(), cache_control.to_string());
        self
    }
    /// Set content disposition for downloads.
    ///
    /// 设置下载时的内容呈现方式。
    pub fn set_content_disposition(mut self, content_disposition: ContentDisposition) -> Self {
        self.req.insert_header(header::CONTENT_DISPOSITION.as_str(), content_disposition.to_string());
        self
    }
    /// Set custom object metadata.
    ///
    /// Metadata keys may only contain letters, numbers, and hyphens.
    ///
    /// 设置对象自定义元数据。
    ///
    /// 元数据键仅允许字母、数字和连字符。
    pub fn set_meta(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        let key = key.into();
        if !invalid_metadata_key(&key) {
            self.req.insert_header(format!("x-oss-meta-{}", key), value.into());
        }
        self
    }
    /// Add a tag key/value pair.
    ///
    /// 追加对象标签键值对。
    pub fn set_tagging(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.tags.insert(key.into(), value.into());
        self
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
    /// Upload a local file to OSS.
    ///
    /// If a progress callback is set, it receives real-time updates.
    ///
    /// 上传本地文件到 OSS。
    ///
    /// 若设置回调，将获得实时进度回调。
    pub async fn send_file(mut self, file: impl Into<String>) -> Result<Option<String>, Error> {
        let file = file.into();
        // Determine file MIME type
        let file_type = match self.mime {
            Some(mime) => mime,
            None => match infer::get_from_path(&file)? {
                Some(ext) => ext.mime_type().to_owned(),
                None => mime_guess::from_path(
                    &self.req.oss.object.clone().map(|v| v.to_string()).unwrap_or_else(|| String::new()),
                )
                .first()
                .map(|v| v.to_string())
                .unwrap_or_else(|| "application/octet-stream".to_owned())
                .to_string(),
            },
        };
        self.req.insert_header(header::CONTENT_TYPE.as_str(), file_type);
        // Insert tags
        let tags = self
            .tags
            .into_iter()
            .map(|(key, value)| {
                if value.is_empty() {
                    url_encode(&key.to_string())
                } else {
                    format!("{}={}", url_encode(&key.to_string()), url_encode(&value.to_string()))
                }
            })
            .collect::<Vec<_>>()
            .join("&");
        if !tags.is_empty() {
            self.req.insert_header("x-oss-tagging", tags);
        }
        // Open the file
        let file = File::open(&file).await?;
        // Read the file size
        let file_size = file.metadata().await?.len();
        if file_size >= 5_368_709_120 {
            return Err(Error::InvalidFileSize);
        }
        self.req.insert_header(header::CONTENT_LENGTH.as_str(), file_size.to_string());
        // Initialize the data stream for reading file content
        let buf = BufReader::with_capacity(131072, file);
        let stream = ReaderStream::with_capacity(buf, 16384);
        // Initialize the uploaded content size
        let mut uploaded_size = 0;
        // Create body object
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
        // Build the HTTP request
        let response = self.req.send_to_oss()?.await?;
        // Parse the response
        let status_code = response.status();
        match status_code {
            code if code.is_success() => {
                let next_position = response
                    .headers()
                    .get("x-oss-next-append-position")
                    .and_then(|header| header.to_str().ok().map(|s| s.to_owned()));
                Ok(next_position)
            }
            _ => Err(normal_error(response).await),
        }
    }
    /// Upload in-memory data to OSS.
    ///
    /// 上传内存数据到 OSS。
    pub async fn send_content(mut self, content: Vec<u8>) -> Result<Option<String>, Error> {
        // Read the file size
        let content_size = content.len();
        if content_size >= 5_368_709_120 {
            return Err(Error::InvalidFileSize);
        }
        self.req.insert_header(header::CONTENT_LENGTH.as_str(), content_size.to_string());
        // Determine file MIME type
        let content_type = match self.mime {
            Some(mime) => mime,
            None => match infer::get(&content) {
                Some(ext) => ext.mime_type().to_string(),
                None => mime_guess::from_path(
                    self.req.oss.object.clone().map(|v| v.to_string()).unwrap_or_else(|| String::new().into()),
                )
                .first()
                .map(|v| v.to_string())
                .unwrap_or_else(|| "application/octet-stream".to_owned())
                .to_string(),
            },
        };
        self.req.insert_header(header::CONTENT_TYPE.as_str(), content_type);
        // Insert tags
        let tags = self
            .tags
            .into_iter()
            .map(|(key, value)| {
                if value.is_empty() {
                    url_encode(&key.to_string())
                } else {
                    format!("{}={}", url_encode(&key.to_string()), url_encode(&value.to_string()))
                }
            })
            .collect::<Vec<_>>()
            .join("&");
        if !tags.is_empty() {
            self.req.insert_header("x-oss-tagging", tags);
        }
        // Insert body
        self.req.set_body(Full::new(Bytes::from(content)));
        // Build the HTTP request
        let response = self.req.send_to_oss()?.await?;
        // Parse the response
        let status_code = response.status();
        match status_code {
            code if code.is_success() => {
                let next_position = response
                    .headers()
                    .get("x-oss-next-append-position")
                    .and_then(|header| header.to_str().ok().map(|s| s.to_owned()));
                Ok(next_position)
            }
            _ => Err(normal_error(response).await),
        }
    }
}
