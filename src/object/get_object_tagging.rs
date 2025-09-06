use crate::{
    error::normal_error,
    request::{Oss, OssRequest},
    Error,
};
use http::Method;
use crate::common::body_to_bytes;
use serde_derive::Deserialize;

// Returned content
#[derive(Debug, Deserialize)]
pub(crate) struct Tagging {
    #[serde(rename = "TagSet")]
    pub tag_set: TagSet,
}

#[derive(Debug, Deserialize)]
pub(crate) struct TagSet {
    #[serde(rename = "Tag")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Deserialize)]
/// Tag information
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,
}

/// Retrieve tag information of an object
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/document_detail/114878.html) for details
pub struct GetObjectTagging {
    req: OssRequest,
}
impl GetObjectTagging {
    pub(super) fn new(oss: Oss) -> Self {
        let mut req = OssRequest::new(oss, Method::GET);
        req.insert_query("tagging", "");
        GetObjectTagging { req }
    }
    /// Send the request
    ///
    pub async fn send(self) -> Result<Option<Vec<Tag>>, Error> {
        // Build the HTTP request
        let response = self.req.send_to_oss()?.await?;
        // Parse the response
        let status_code = response.status();
        match status_code {
            code if code.is_success() => {
                let response_bytes = body_to_bytes(response.into_body())
                    .await
                    .map_err(|_| Error::OssInvalidResponse(None))?;
                let tagging: Tagging = serde_xml_rs::from_reader(&*response_bytes)
                    .map_err(|_| Error::OssInvalidResponse(Some(response_bytes)))?;
                Ok(tagging.tag_set.tags)
            }
            _ => Err(normal_error(response).await),
        }
    }
}
