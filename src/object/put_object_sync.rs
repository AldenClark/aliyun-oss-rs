use crate::{
    common::{Acl, CacheControl, ContentDisposition, StorageClass, invalid_metadata_key, url_encode},
    error::{Error, normal_error_sync},
    request_sync::{Oss, OssRequest},
};
use http::{Method, header};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use ureq::SendBody;

/// Upload an object to OSS (sync).
///
/// The object size cannot exceed 5GB.
///
/// By default, if an object with the same name exists and you have access, the new object overwrites it.
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/document_detail/31978.html) for details.
///
/// 上传对象到 OSS（同步）。
///
/// 对象大小不能超过 5GB。
///
/// 默认情况下若同名对象存在且有权限，新对象会覆盖旧对象。
///
/// 详情见 [阿里云文档](https://help.aliyun.com/document_detail/31978.html)。
pub struct PutObjectSync {
    req: OssRequest,
    mime: Option<String>,
    tags: HashMap<String, String>,
    callback: Option<Box<dyn Fn(u64, u64) + Send + Sync + 'static>>,
}

struct ProgressReader<R> {
    inner: R,
    uploaded: u64,
    total: u64,
    callback: Option<Box<dyn Fn(u64, u64) + Send + Sync + 'static>>,
}

impl<R: Read> ProgressReader<R> {
    fn new(inner: R, total: u64, callback: Option<Box<dyn Fn(u64, u64) + Send + Sync + 'static>>) -> Self {
        ProgressReader { inner, uploaded: 0, total, callback }
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

impl PutObjectSync {
    pub(super) fn new(oss: Oss) -> Self {
        PutObjectSync { req: OssRequest::new(oss, Method::PUT), mime: None, tags: HashMap::new(), callback: None }
    }
    /// Set the object's MIME type.
    ///
    /// If not set, MIME type is inferred from content or path; fallback is `application/octet-stream`.
    ///
    /// 设置对象的 MIME 类型。
    ///
    /// 未设置时尝试从内容或路径推断，失败则使用 `application/octet-stream`。
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
    /// Set response cache behavior when the object is downloaded.
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
    /// Disallow overwriting existing objects with the same key.
    ///
    /// 禁止覆盖同名对象。
    pub fn forbid_overwrite(mut self) -> Self {
        self.req.insert_header("x-oss-forbid-overwrite", "true");
        self
    }
    /// Set custom metadata.
    ///
    /// Keys must contain only letters, numbers, and hyphens; invalid keys are ignored.
    ///
    /// 设置自定义元数据。
    ///
    /// Key 仅支持字母、数字和连字符；无效 Key 会被忽略。
    pub fn set_meta(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        let key = key.into();
        if !invalid_metadata_key(&key) {
            self.req.insert_header(format!("x-oss-meta-{}", key), value.into());
        }
        self
    }
    /// Add object tags.
    ///
    /// 添加对象标签。
    pub fn set_tagging(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.tags.insert(key.into(), value.into());
        self
    }
    /// Set a progress callback (only applies to `send_file()`).
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
    /// 设置上传进度回调（仅适用于 `send_file()`）。
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
    /// Upload a file from disk to OSS.
    ///
    /// 从磁盘上传文件到 OSS。
    pub fn send_file(mut self, file: impl Into<String>) -> Result<(), Error> {
        let file = file.into();
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
        let file = File::open(&file)?;
        let file_size = file.metadata()?.len();
        if file_size >= 5_368_709_120 {
            return Err(Error::InvalidFileSize);
        }
        self.req.insert_header(header::CONTENT_LENGTH.as_str(), file_size.to_string());
        let reader = BufReader::with_capacity(131072, file);
        let reader: Box<dyn Read> = match self.callback {
            Some(callback) => Box::new(ProgressReader::new(reader, file_size, Some(callback))),
            None => Box::new(reader),
        };
        let body = SendBody::from_owned_reader(reader);
        let response = self.req.send_to_oss_with_body(body)?;
        let status_code = response.status();
        match status_code {
            code if code.is_success() => Ok(()),
            _ => Err(normal_error_sync(response)),
        }
    }
    /// Upload in-memory data to OSS.
    ///
    /// 上传内存数据到 OSS。
    pub fn send_content(mut self, content: Vec<u8>) -> Result<(), Error> {
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
        let content_size = content.len() as u64;
        if content_size >= 5_368_709_120 {
            return Err(Error::InvalidFileSize);
        }
        self.req.insert_header(header::CONTENT_LENGTH.as_str(), content_size.to_string());
        self.req.set_body(content);
        let response = self.req.send_to_oss()?;
        let status_code = response.status();
        match status_code {
            code if code.is_success() => Ok(()),
            _ => Err(normal_error_sync(response)),
        }
    }
}
