use crate::common::body_to_bytes;
use crate::{
    Error,
    common::format_gmt,
    error::normal_error,
    request::{Oss, OssRequest},
};
use bytes::Bytes;
use futures_util::{Stream, StreamExt};
use http::Method;
use http_body_util::BodyExt;
use std::pin::Pin;
use time::OffsetDateTime;
use tokio::{
    fs::{OpenOptions, create_dir_all},
    io::{AsyncWriteExt, BufWriter},
};

/// Retrieve the object's content.
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/document_detail/31980.html) for details.
///
/// 获取对象内容。
///
/// 详情见 [阿里云文档](https://help.aliyun.com/document_detail/31980.html)。
pub struct GetObject {
    req: OssRequest,
}
impl GetObject {
    pub(super) fn new(oss: Oss) -> Self {
        GetObject {
            req: OssRequest::new(oss, Method::GET),
        }
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
            format!(
                "bytes={}-{}",
                start,
                end.map(|v| v.to_string()).unwrap_or_else(|| String::new())
            ),
        );
        self
    }
    /// Succeeds if the object was modified after the given time.
    ///
    /// 若对象在给定时间后被修改，请求成功。
    pub fn set_if_modified_since(mut self, if_modified_since: OffsetDateTime) -> Self {
        self.req
            .insert_header("If-Modified-Since", format_gmt(if_modified_since));
        self
    }
    /// Succeeds if the object was not modified since the given time.
    ///
    /// 若对象自给定时间起未修改，请求成功。
    pub fn set_if_unmodified_since(mut self, if_unmodified_since: OffsetDateTime) -> Self {
        self.req
            .insert_header("If-Unmodified-Since", format_gmt(if_unmodified_since));
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
    pub async fn download_to_file(self, save_path: impl Into<String>) -> Result<(), Error> {
        let save_path = save_path.into();
        // Validate path
        if save_path.contains("://") {
            return Err(Error::PathNotSupported);
        }
        // Send request
        let response = self.req.send_to_oss()?.await?;
        // Parse the response
        let status_code = response.status();
        match status_code {
            code if code.is_success() => {
                // Create directory
                let parent_dir = std::path::Path::new(&save_path).parent();
                if let Some(dir) = parent_dir {
                    create_dir_all(dir).await?;
                }
                // Create file
                let file = OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .open(&save_path)
                    .await?;
                // Create write buffer
                let mut writer = BufWriter::with_capacity(131072, file);
                // Read byte stream
                let mut response_bytes = response.into_body().into_data_stream();
                while let Some(chunk) = response_bytes.next().await {
                    match chunk {
                        Ok(data) => writer.write_all(data.as_ref()).await?,
                        Err(e) => return Err(Error::HyperError(e)),
                    }
                }
                writer.flush().await?;
                writer.shutdown().await?;
                Ok(())
            }
            _ => Err(normal_error(response).await),
        }
    }
    /// Download the object and return the content as bytes.
    ///
    /// Large objects may consume too much memory; use with caution.
    ///
    /// 下载对象并返回字节内容。
    ///
    /// 大对象可能占用大量内存，请谨慎使用。
    pub async fn download(self) -> Result<Bytes, Error> {
        // Send request
        let response = self.req.send_to_oss()?.await?;
        // Parse the response
        let status_code = response.status();
        match status_code {
            code if code.is_success() => Ok(body_to_bytes(response.into_body()).await?),
            _ => Err(normal_error(response).await),
        }
    }
    /// Download the object as a stream.
    ///
    /// Use this for large objects and process the stream yourself.
    ///
    /// ```ignore
    /// use futures_util::StreamExt;
    ///
    /// let mut stream = object.get_object().download_to_stream().await.unwrap();
    /// while let Some(item) = stream.next().await {
    /// match item {
    /// Ok(bytes) => {
    /// // Do something with bytes...
    /// }
    /// Err(e) => eprintln!("Error: {}", e),
    /// }
    /// }
    /// ```
    ///
    /// 以流式方式下载对象。
    ///
    /// 适用于大对象，调用方自行处理流。
    ///
    /// ```ignore
    /// use futures_util::StreamExt;
    ///
    /// let mut stream = object.get_object().download_to_stream().await.unwrap();
    /// while let Some(item) = stream.next().await {
    /// match item {
    /// Ok(bytes) => {
    /// // Do something with bytes...
    /// }
    /// Err(e) => eprintln!("Error: {}", e),
    /// }
    /// }
    /// ```
    pub async fn download_to_stream(
        self,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<bytes::Bytes, Error>> + Send>>, Error> {
        // Send request
        let response = self.req.send_to_oss()?.await?;
        // Parse the response
        let status_code = response.status();
        match status_code {
            code if code.is_success() => {
                let stream = response
                    .into_body()
                    .into_data_stream()
                    .map(|item| match item {
                        Ok(bytes) => Ok(bytes),
                        Err(e) => Err(e.into()),
                    });
                Ok(Box::pin(stream))
            }
            _ => Err(normal_error(response).await),
        }
    }
}
