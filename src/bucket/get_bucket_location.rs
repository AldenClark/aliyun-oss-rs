use crate::common::body_to_bytes;
use crate::{
    Error,
    error::normal_error,
    request::{Oss, OssRequest},
};
use http::Method;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct LocationConstraint {
    #[serde(rename = "$value")]
    pub location: String,
}

/// Retrieve bucket location.
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/zh/oss/developer-reference/getbucketlocation) for details.
///
/// 获取 Bucket 所在地域。
///
/// 详情参见 [阿里云文档](https://help.aliyun.com/zh/oss/developer-reference/getbucketlocation)。
pub struct GetBucketLocation {
    req: OssRequest,
}
impl GetBucketLocation {
    pub(super) fn new(oss: Oss) -> Self {
        let mut req = OssRequest::new(oss, Method::GET);
        req.insert_query("location", "");
        GetBucketLocation { req }
    }
    /// Send the request and return the region identifier.
    ///
    /// 发送请求并返回地域标识。
    pub async fn send(self) -> Result<String, Error> {
        let response = self.req.send_to_oss()?.await?;
        let status_code = response.status();
        match status_code {
            code if code.is_success() => {
                let response_bytes =
                    body_to_bytes(response.into_body()).await.map_err(|_| Error::OssInvalidResponse(None))?;
                let location: LocationConstraint = serde_xml_rs::from_reader(&*response_bytes)
                    .map_err(|_| Error::OssInvalidResponse(Some(response_bytes)))?;
                Ok(location.location)
            }
            _ => Err(normal_error(response).await),
        }
    }
}
