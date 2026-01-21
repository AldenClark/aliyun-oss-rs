use crate::common::body_to_bytes_sync;
use crate::{
    Error,
    common::format_gmt,
    error::normal_error_sync,
    request_sync::{Oss, OssRequest},
};
use bytes::Bytes;
use http::Method;
use std::fs::{OpenOptions, create_dir_all};
use std::io::{BufWriter, Write};
use std::path::Path;

/// Retrieve the object's content (sync).
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/document_detail/31980.html) for details.
///
/// 获取对象内容（同步）。
///
/// 详情见 [阿里云文档](https://help.aliyun.com/document_detail/31980.html)。
pub struct GetObjectSync {
    req: OssRequest,
}
impl GetObjectSync {
    pub(super) fn new(oss: Oss) -> Self {
        GetObjectSync { req: OssRequest::new(oss, Method::GET) }
    }
    /// Set the response byte range.
    ///
    /// `end` must be >= `start` and within bounds; invalid values download the whole object.
    ///
    /// Byte indexing starts at 0; for a 500-byte file, the range is 0-499.
    ///
    /// 设置响应字节范围。
    ///
    /// `end` 必须 >= `start` 且在有效范围内；无效值会下载整个对象。
    ///
    /// 字节从 0 开始计数；500 字节文件范围为 0-499。
    pub fn set_range(mut self, start: usize, end: Option<usize>) -> Self {
        self.req.insert_header(
            "Range",
            format!("bytes={}-{}", start, end.map(|v| v.to_string()).unwrap_or_else(|| String::new())),
        );
        self
    }
    /// Succeeds if the object was modified after the given time.
    ///
    /// 若对象在给定时间后被修改，请求成功。
    pub fn set_if_modified_since(mut self, if_modified_since: time::OffsetDateTime) -> Self {
        self.req.insert_header("If-Modified-Since", format_gmt(if_modified_since));
        self
    }
    /// Succeeds if the object was not modified since the given time.
    ///
    /// 若对象自给定时间起未修改，请求成功。
    pub fn set_if_unmodified_since(mut self, if_unmodified_since: time::OffsetDateTime) -> Self {
        self.req.insert_header("If-Unmodified-Since", format_gmt(if_unmodified_since));
        self
    }
    /// Succeeds if the provided ETag matches the object's ETag.
    ///
    /// ETag can be used to verify whether data has changed.
    ///
    /// 当提供的 ETag 与对象 ETag 匹配时请求成功。
    ///
    /// ETag 可用于判断数据是否变化。
    pub fn set_if_match(mut self, if_match: impl Into<String>) -> Self {
        self.req.insert_header("If-Match", if_match.into());
        self
    }
    /// Succeeds if the provided ETag does not match the object's ETag.
    ///
    /// 当提供的 ETag 与对象 ETag 不匹配时请求成功。
    pub fn set_if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
        self.req.insert_header("If-None-Match", if_none_match.into());
        self
    }
    /// Download the object to disk.
    ///
    /// Network paths are not supported; mount SMB/NFS locally instead.
    ///
    /// 下载对象到本地磁盘。
    ///
    /// 不支持网络路径；请先挂载 SMB/NFS 等再使用本地路径。
    pub fn download_to_file(self, save_path: impl Into<String>) -> Result<(), Error> {
        let save_path = save_path.into();
        if save_path.contains("://") {
            return Err(Error::PathNotSupported);
        }
        let response = self.req.send_to_oss()?;
        let status_code = response.status();
        match status_code {
            code if code.is_success() => {
                if let Some(dir) = Path::new(&save_path).parent() {
                    create_dir_all(dir)?;
                }
                let file = OpenOptions::new().write(true).create_new(true).open(&save_path)?;
                let mut writer = BufWriter::with_capacity(131072, file);
                let mut reader = response.into_body().into_reader();
                std::io::copy(&mut reader, &mut writer)?;
                writer.flush()?;
                Ok(())
            }
            _ => Err(normal_error_sync(response)),
        }
    }
    /// Download the object and return the content as bytes.
    ///
    /// Large objects may consume too much memory; use with caution.
    ///
    /// 下载对象并返回字节内容。
    ///
    /// 大对象可能占用大量内存，请谨慎使用。
    pub fn download(self) -> Result<Bytes, Error> {
        let response = self.req.send_to_oss()?;
        let status_code = response.status();
        match status_code {
            code if code.is_success() => Ok(body_to_bytes_sync(response.into_body())?),
            _ => Err(normal_error_sync(response)),
        }
    }
    /// Download the object and return a blocking reader.
    ///
    /// Use this for large objects and process the stream yourself.
    ///
    /// 以阻塞读取方式下载对象。
    ///
    /// 适用于大对象，调用方自行读取处理。
    pub fn download_to_reader(self) -> Result<ureq::BodyReader<'static>, Error> {
        let response = self.req.send_to_oss()?;
        let status_code = response.status();
        match status_code {
            code if code.is_success() => Ok(response.into_body().into_reader()),
            _ => Err(normal_error_sync(response)),
        }
    }
}
