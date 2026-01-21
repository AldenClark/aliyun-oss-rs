use crate::{
    Error,
    common::Acl,
    error::normal_error,
    request::{Oss, OssRequest},
};
use http::Method;

/// Set bucket ACL.
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/document_detail/31965.html) for details.
///
/// 设置 Bucket ACL。
///
/// 详情参见 [阿里云文档](https://help.aliyun.com/document_detail/31965.html)。
pub struct PutBucketAcl {
    req: OssRequest,
}
impl PutBucketAcl {
    pub(crate) fn new(oss: Oss) -> Self {
        let mut req = OssRequest::new(oss, Method::PUT);
        req.insert_query("acl", "");
        PutBucketAcl { req }
    }
    /// Set the access control.
    ///
    /// 设置访问控制。
    pub fn set_acl(mut self, acl: Acl) -> Self {
        self.req.insert_header("x-oss-acl", acl.to_string());
        self
    }
    /// Send the request.
    ///
    /// 发送请求。
    pub async fn send(self) -> Result<(), Error> {
        let response = self.req.send_to_oss()?.await?;
        let status_code = response.status();
        match status_code {
            code if code.is_success() => Ok(()),
            _ => Err(normal_error(response).await),
        }
    }
}
