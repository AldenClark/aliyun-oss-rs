use crate::{
    Error,
    error::normal_error_sync,
    request_sync::{Oss, OssRequest},
};
use http::Method;

use super::{RefererConfiguration, RefererList};

/// Configure bucket hotlink protection (Referer whitelist).
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/zh/oss/developer-reference/putbucketreferer) for details.
///
/// 配置 Bucket 防盗链（Referer 白名单）。
///
/// 详情参见 [阿里云文档](https://help.aliyun.com/zh/oss/developer-reference/putbucketreferer)。
pub struct PutBucketRefererSync {
    req: OssRequest,
    config: RefererConfiguration,
}

impl PutBucketRefererSync {
    pub(super) fn new(oss: Oss) -> Self {
        let mut req = OssRequest::new(oss, Method::PUT);
        req.insert_query("referer", "");
        PutBucketRefererSync {
            req,
            config: RefererConfiguration {
                allow_empty_referer: true,
                referer_list: RefererList::default(),
            },
        }
    }

    /// Set whether empty Referer headers are allowed.
    ///
    /// 设置是否允许空 Referer。
    pub fn allow_empty_referer(mut self, allow: bool) -> Self {
        self.config.allow_empty_referer = allow;
        self
    }

    /// Replace the referer whitelist.
    ///
    /// 替换 Referer 白名单。
    pub fn set_whitelist(mut self, referers: Vec<impl Into<String>>) -> Self {
        self.config.referer_list.items = referers.into_iter().map(Into::into).collect();
        self
    }

    /// Send the request.
    ///
    /// 发送请求。
    pub fn send(mut self) -> Result<(), Error> {
        let body = serde_xml_rs::to_string(&self.config).map_err(|_| Error::InvalidCharacter)?;
        self.req.set_body(body.into_bytes());
        let response = self.req.send_to_oss()?;
        match response.status() {
            code if code.is_success() => Ok(()),
            _ => Err(normal_error_sync(response)),
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
