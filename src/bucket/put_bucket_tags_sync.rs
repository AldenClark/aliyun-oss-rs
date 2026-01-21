use crate::{
    Error,
    error::normal_error_sync,
    request_sync::{Oss, OssRequest},
};
use http::Method;

use super::{BucketTagging, Tag};

/// Configure bucket tags.
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/zh/oss/developer-reference/putbuckettags) for details.
///
/// 配置 Bucket 标签。
///
/// 详情见 [阿里云文档](https://help.aliyun.com/zh/oss/developer-reference/putbuckettags)。
pub struct PutBucketTagsSync {
    req: OssRequest,
    tagging: BucketTagging,
}

impl PutBucketTagsSync {
    pub(super) fn new(oss: Oss, tags: Vec<(impl Into<String>, impl Into<String>)>) -> Self {
        let mut req = OssRequest::new(oss, Method::PUT);
        req.insert_query("tagging", "");
        let mut tagging = BucketTagging::default();
        tagging.tag_set.tags =
            tags.into_iter().map(|(key, value)| Tag { key: key.into(), value: Some(value.into()) }).collect();
        PutBucketTagsSync { req, tagging }
    }

    /// Add more tags.
    ///
    /// 追加标签。
    pub fn add_tags(mut self, tags: Vec<(impl Into<String>, impl Into<String>)>) -> Self {
        self.tagging
            .tag_set
            .tags
            .extend(tags.into_iter().map(|(key, value)| Tag { key: key.into(), value: Some(value.into()) }));
        self
    }

    /// Replace the entire tagging document.
    ///
    /// 替换完整标签文档。
    pub fn set_tagging(mut self, tagging: BucketTagging) -> Self {
        self.tagging = tagging;
        self
    }

    /// Send the request.
    ///
    /// 发送请求。
    pub fn send(mut self) -> Result<(), Error> {
        let body = serde_xml_rs::to_string(&self.tagging).map_err(|_| Error::InvalidCharacter)?;
        self.req.set_body(body.into_bytes());
        let response = self.req.send_to_oss()?;
        match response.status() {
            code if code.is_success() => Ok(()),
            _ => Err(normal_error_sync(response)),
        }
    }
}
