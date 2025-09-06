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

/// Retrieve the object's content
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/document_detail/31980.html) for details
pub struct GetObject {
    req: OssRequest,
}
impl GetObject {
    pub(super) fn new(oss: Oss) -> Self {
        GetObject {
            req: OssRequest::new(oss, Method::GET),
        }
    }
    /// Set the response range
    ///
    /// `end` must be greater than or equal to `start` and both must be within the valid range; invalid values result in downloading the entire file
    ///
    /// Byte indexing starts at 0; for a 500-byte file, the range is 0-499
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
    /// If the specified time is earlier than the actual modification time, the request succeeds
    ///
    pub fn set_if_modified_since(mut self, if_modified_since: OffsetDateTime) -> Self {
        self.req
            .insert_header("If-Modified-Since", format_gmt(if_modified_since));
        self
    }
    /// If the specified time is equal to or later than the actual modification time, the request succeeds
    ///
    pub fn set_if_unmodified_since(mut self, if_unmodified_since: OffsetDateTime) -> Self {
        self.req
            .insert_header("If-Unmodified-Since", format_gmt(if_unmodified_since));
        self
    }
    /// If the provided ETag matches the object's ETag, the request succeeds
    ///
    /// The ETag verifies whether the data has changed and can be used to check data integrity
    pub fn set_if_match(mut self, if_match: impl ToString) -> Self {
        self.req.insert_header("If-Match", if_match);
        self
    }
    /// If the provided ETag differs from the object's ETag, the request succeeds
    ///
    pub fn set_if_none_match(mut self, if_none_match: impl ToString) -> Self {
        self.req.insert_header("If-None-Match", if_none_match);
        self
    }
    /// Download the object to disk
    ///
    /// Network paths are not supported. For SMB/NFS and similar storage, mount locally and use the local path
    pub async fn download_to_file(self, save_path: &str) -> Result<(), Error> {
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
                let parent_dir = std::path::Path::new(save_path).parent();
                if let Some(dir) = parent_dir {
                    create_dir_all(dir).await?;
                }
                // Create file
                let file = OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .open(save_path)
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
    /// Download the object and return the content
    ///
    /// If the object is large, this method may use too much memory; use with caution
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
    /// Download the object and return a data stream
    ///
    /// Use this if the object is large and you do not want to save directly to a file; process the stream yourself
    ///
    /// ```ignore
    /// use futures_util::StreamExt;
    ///
    /// let mut stream = object.get_object().download_to_stream().await.unwrap();
    /// while let Some(item) = stream.next().await {
    ///     match item {
    ///         Ok(bytes) => {
    ///             // Do something with bytes...
    ///         }
    ///         Err(e) => eprintln!("Error: {}", e),
    ///     }
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
