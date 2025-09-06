use crate::{
    common::format_gmt,
    error::{Error, normal_error},
    request::{Oss, OssRequest},
};
use http::Method;
use time::OffsetDateTime;

/// Initialize a multipart upload part
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/document_detail/31994.html) for details
pub struct CopyToPart {
    req: OssRequest,
}
impl CopyToPart {
    pub(super) fn new(
        oss: Oss,
        part_number: u32,
        upload_id: impl ToString,
        copy_source: impl ToString,
    ) -> Self {
        let mut req = OssRequest::new(oss, Method::PUT);
        req.insert_query("partNumber", part_number);
        req.insert_query("uploadId", upload_id);
        req.insert_header("x-oss-copy-source", copy_source);
        CopyToPart { req }
    }
    /// Set the source file copy range
    ///
    /// By default, the entire file is copied; byte indexing starts at 0
    pub fn set_source_range(mut self, start: usize, end: Option<usize>) -> Self {
        self.req.insert_header(
            "x-oss-copy-source-range",
            format!(
                "bytes={}-{}",
                start,
                end.map(|v| v.to_string()).unwrap_or_else(|| String::new())
            ),
        );
        self
    }
    /// If the specified time is earlier than the file's actual modification time, the copy proceeds.
    ///
    pub fn set_if_modified_since(mut self, if_modified_since: OffsetDateTime) -> Self {
        self.req.insert_header(
            "x-oss-copy-source-if-modified-since",
            format_gmt(if_modified_since),
        );
        self
    }
    /// If the specified time is equal to or later than the file's actual modification time, the copy proceeds.
    ///
    pub fn set_if_unmodified_since(mut self, if_unmodified_since: OffsetDateTime) -> Self {
        self.req.insert_header(
            "x-oss-copy-source-if-unmodified-since",
            format_gmt(if_unmodified_since),
        );
        self
    }
    /// Copy the source file only if its ETag matches the value you provide.
    ///
    /// The ETag is used to verify whether the data has changed; you can use it to check data integrity.
    pub fn set_if_match(mut self, if_match: impl ToString) -> Self {
        self.req
            .insert_header("x-oss-copy-source-if-match", if_match);
        self
    }
    /// Copy the source file only if its ETag does not match the value you provide.
    ///
    /// The ETag is used to verify whether the data has changed; you can use it to check data integrity.
    pub fn set_if_none_match(mut self, if_none_match: impl ToString) -> Self {
        self.req
            .insert_header("x-oss-copy-source-if-none-match", if_none_match);
        self
    }
    /// Copy object content to a part
    ///
    /// Returns the ETag
    pub async fn send(self) -> Result<String, Error> {
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
