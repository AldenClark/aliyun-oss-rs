use crate::{
    Error,
    common::body_to_bytes,
    error::normal_error,
    request::{Oss, OssRequest},
};
use http::Method;

use super::RefererConfiguration;

/// Retrieve the referer configuration of a bucket.
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/zh/oss/developer-reference/getbucketreferer) for details.
///
/// 获取 Bucket 的防盗链配置。
///
/// 详情参见 [阿里云文档](https://help.aliyun.com/zh/oss/developer-reference/getbucketreferer)。
pub struct GetBucketReferer {
    req: OssRequest,
}

impl GetBucketReferer {
    pub(super) fn new(oss: Oss) -> Self {
        let mut req = OssRequest::new(oss, Method::GET);
        req.insert_query("referer", "");
        GetBucketReferer { req }
    }

    /// Send the request and return the parsed configuration.
    ///
    /// 发送请求并返回解析后的配置。
    pub async fn send(self) -> Result<RefererConfiguration, Error> {
        let response = self.req.send_to_oss()?.await?;
        match response.status() {
            code if code.is_success() => {
                let bytes = body_to_bytes(response.into_body()).await?;
                let config: RefererConfiguration =
                    serde_xml_rs::from_reader(bytes.as_ref()).map_err(|_| Error::OssInvalidResponse(Some(bytes)))?;
                Ok(config)
            }
            _ => Err(normal_error(response).await),
        }
    }
}
