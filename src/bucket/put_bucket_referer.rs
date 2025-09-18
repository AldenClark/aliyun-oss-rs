use crate::{
    Error,
    error::normal_error,
    request::{Oss, OssRequest},
};
use bytes::Bytes;
use http::Method;
use http_body_util::Full;

use super::{RefererConfiguration, RefererList};

/// Configure bucket hotlink protection (Referer whitelist).
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/zh/oss/developer-reference/putbucketreferer) for details.
pub struct PutBucketReferer {
    req: OssRequest,
    config: RefererConfiguration,
}

impl PutBucketReferer {
    pub(super) fn new(oss: Oss) -> Self {
        let mut req = OssRequest::new(oss, Method::PUT);
        req.insert_query("referer", "");
        PutBucketReferer {
            req,
            config: RefererConfiguration {
                allow_empty_referer: true,
                referer_list: RefererList::default(),
            },
        }
    }

    /// Set whether empty Referer headers are allowed.
    pub fn allow_empty_referer(mut self, allow: bool) -> Self {
        self.config.allow_empty_referer = allow;
        self
    }

    /// Replace the referer whitelist.
    pub fn set_whitelist(mut self, referers: Vec<impl ToString>) -> Self {
        self.config.referer_list.items = referers.into_iter().map(|s| s.to_string()).collect();
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
    fn test_referer_serialization() {
        let config = RefererConfiguration {
            allow_empty_referer: false,
            referer_list: RefererList {
                items: vec!["https://example.com".to_string()],
            },
        };
        let xml = serde_xml_rs::to_string(&config).unwrap();
        assert!(xml.contains("<RefererConfiguration>"));
        assert!(xml.contains("<AllowEmptyReferer>false</AllowEmptyReferer>"));
        assert!(xml.contains("<Referer>https://example.com</Referer>"));
    }
}
