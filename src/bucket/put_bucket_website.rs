use crate::{
    Error,
    error::normal_error,
    request::{Oss, OssRequest},
};
use bytes::Bytes;
use http::Method;
use http_body_util::Full;

use super::{ErrorDocument, IndexDocument, WebsiteConfiguration};

/// Configure bucket static website hosting.
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/zh/oss/developer-reference/putbucketwebsite) for details.
pub struct PutBucketWebsite {
    req: OssRequest,
    config: WebsiteConfiguration,
}

impl PutBucketWebsite {
    pub(super) fn new(oss: Oss) -> Self {
        let mut req = OssRequest::new(oss, Method::PUT);
        req.insert_query("website", "");
        PutBucketWebsite {
            req,
            config: WebsiteConfiguration::default(),
        }
    }

    /// Set the index document suffix (for example, `index.html`).
    pub fn set_index_document(mut self, suffix: impl ToString) -> Self {
        self.config.index_document = Some(IndexDocument {
            suffix: suffix.to_string(),
        });
        self
    }

    /// Set the error document key (for example, `error.html`).
    pub fn set_error_document(mut self, key: impl ToString) -> Self {
        self.config.error_document = Some(ErrorDocument {
            key: key.to_string(),
        });
        self
    }

    /// Replace the raw configuration object.
    pub fn set_configuration(mut self, config: WebsiteConfiguration) -> Self {
        self.config = config;
        self
    }

    /// Send the request.
    pub async fn send(mut self) -> Result<(), Error> {
        let body = serde_xml_rs::to_string(&self.config).map_err(|_| Error::InvalidCharacter)?;
        self.req.set_body(Full::new(Bytes::from(body)));
        let response = self.req.send_to_oss()?.await?;
        match response.status() {
            code if code.is_success() => Ok(()),
            _ => Err(normal_error(response).await),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_website_serialization() {
        let config = WebsiteConfiguration {
            index_document: Some(IndexDocument {
                suffix: "index.html".into(),
            }),
            error_document: Some(ErrorDocument {
                key: "error.html".into(),
            }),
            routing_rules: None,
        };
        let xml = serde_xml_rs::to_string(&config).unwrap();
        assert!(xml.contains("<WebsiteConfiguration>"));
        assert!(xml.contains("<Suffix>index.html</Suffix>"));
        assert!(xml.contains("<Key>error.html</Key>"));
    }
}
