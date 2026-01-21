use crate::{
    Error,
    common::body_to_bytes,
    error::normal_error,
    request::{Oss, OssRequest},
};
use http::Method;

use super::BucketTagging;

/// Retrieve bucket tags.
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/zh/oss/developer-reference/getbuckettags) for details.
///
/// 获取 Bucket 标签。
///
/// 详情见 [阿里云文档](https://help.aliyun.com/zh/oss/developer-reference/getbuckettags)。
pub struct GetBucketTags {
    req: OssRequest,
}

impl GetBucketTags {
    pub(super) fn new(oss: Oss) -> Self {
        let mut req = OssRequest::new(oss, Method::GET);
        req.insert_query("tagging", "");
        GetBucketTags { req }
    }

    /// Send the request and return the parsed tags.
    ///
    /// 发送请求并返回解析后的标签。
    pub async fn send(self) -> Result<BucketTagging, Error> {
        let response = self.req.send_to_oss()?.await?;
        match response.status() {
            code if code.is_success() => {
                let bytes = body_to_bytes(response.into_body()).await?;
                let tags: BucketTagging =
                    serde_xml_rs::from_reader(bytes.as_ref()).map_err(|_| Error::OssInvalidResponse(Some(bytes)))?;
                Ok(tags)
            }
            _ => Err(normal_error(response).await),
        }
    }
}
