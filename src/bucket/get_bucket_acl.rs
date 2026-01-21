use crate::common::body_to_bytes;
use crate::{
    Error,
    common::{Acl, BucketAcl, Owner},
    error::normal_error,
    request::{Oss, OssRequest},
};
use http::Method;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct AccessControlPolicy {
    owner: Owner,
    access_control_list: AccessControlList,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct AccessControlList {
    grant: Acl,
}

/// Retrieve bucket ACL.
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/document_detail/31964.html) for details.
///
/// 获取 Bucket ACL。
///
/// 详情参见 [阿里云文档](https://help.aliyun.com/document_detail/31964.html)。
pub struct GetBucketAcl {
    req: OssRequest,
}
impl GetBucketAcl {
    pub(crate) fn new(oss: Oss) -> Self {
        let mut req = OssRequest::new(oss, Method::GET);
        req.insert_query("acl", "");
        GetBucketAcl { req }
    }
    /// Send the request and return bucket ACL info.
    ///
    /// 发送请求并返回 Bucket ACL 信息。
    pub async fn send(self) -> Result<BucketAcl, Error> {
        let response = self.req.send_to_oss()?.await?;
        let status_code = response.status();
        match status_code {
            code if code.is_success() => {
                let response_bytes = body_to_bytes(response.into_body())
                    .await
                    .map_err(|_| Error::OssInvalidResponse(None))?;
                let result: AccessControlPolicy = serde_xml_rs::from_reader(&*response_bytes)
                    .map_err(|_| Error::OssInvalidResponse(Some(response_bytes)))?;
                Ok(BucketAcl {
                    owner: result.owner,
                    acl: result.access_control_list.grant,
                })
            }
            _ => Err(normal_error(response).await),
        }
    }
}
