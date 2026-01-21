use crate::common::body_to_bytes;
use crate::{
    Error,
    common::Acl,
    error::normal_error,
    request::{Oss, OssRequest},
};
use http::Method;
use serde_derive::Deserialize;

// Returned content
/// Object ACL information.
///
/// 对象 ACL 信息。
#[derive(Debug, Deserialize)]
struct AccessControlPolicy {
    #[serde(rename = "AccessControlList")]
    access_control_list: AccessControlList,
}

#[derive(Debug, Deserialize)]
struct AccessControlList {
    #[serde(rename = "Grant")]
    grant: Acl,
}

/// Retrieve the object's ACL.
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/document_detail/31987.html) for details.
///
/// 获取对象 ACL。
///
/// 详情参见 [阿里云文档](https://help.aliyun.com/document_detail/31987.html)。
pub struct GetObjectAcl {
    req: OssRequest,
}
impl GetObjectAcl {
    pub(super) fn new(oss: Oss) -> Self {
        let mut req = OssRequest::new(oss, Method::GET);
        req.insert_query("acl", "");
        GetObjectAcl { req }
    }
    /// Send the request and return the ACL.
    ///
    /// 发送请求并返回对象 ACL。
    pub async fn send(self) -> Result<Acl, Error> {
        // Build the HTTP request
        let response = self.req.send_to_oss()?.await?;
        // Parse the response
        let status_code = response.status();
        match status_code {
            code if code.is_success() => {
                let response_bytes = body_to_bytes(response.into_body())
                    .await
                    .map_err(|_| Error::OssInvalidResponse(None))?;
                let acl: AccessControlPolicy = serde_xml_rs::from_reader(&*response_bytes)
                    .map_err(|_| Error::OssInvalidResponse(Some(response_bytes)))?;
                Ok(acl.access_control_list.grant)
            }
            _ => Err(normal_error(response).await),
        }
    }
}
