use crate::{
    Error,
    common::{Acl, StorageClass},
    error::normal_error,
    request::{Oss, OssRequest},
};
use http::Method;

/// Create a symlink object.
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/document_detail/45126.html) for details.
///
/// 创建符号链接对象。
///
/// 详情参见 [阿里云文档](https://help.aliyun.com/document_detail/45126.html)。
pub struct PutSymlink {
    req: OssRequest,
}
impl PutSymlink {
    pub(super) fn new(oss: Oss, symlink_target: impl Into<String>) -> Self {
        let mut req = OssRequest::new(oss, Method::PUT);
        req.insert_query("symlink", "");
        req.insert_header("x-oss-symlink-target", symlink_target.into());
        PutSymlink { req }
    }
    /// Set object ACL.
    ///
    /// 设置对象 ACL。
    pub fn set_acl(mut self, acl: Acl) -> Self {
        self.req.insert_header("x-oss-object-acl", acl.to_string());
        self
    }
    /// Set object storage class.
    ///
    /// 设置对象存储类型。
    pub fn set_storage_class(mut self, storage_class: StorageClass) -> Self {
        self.req.insert_header("x-oss-storage-class", storage_class.to_string());
        self
    }
    /// Disallow overwriting objects with the same key.
    ///
    /// 禁止覆盖同名对象。
    pub fn forbid_overwrite(mut self) -> Self {
        self.req.insert_header("x-oss-forbid-overwrite", "true");
        self
    }
    /// Send the request.
    ///
    /// 发送请求。
    pub async fn send(self) -> Result<(), Error> {
        // Build the HTTP request
        let response = self.req.send_to_oss()?.await?;
        // Parse the response
        let status_code = response.status();
        match status_code {
            code if code.is_success() => Ok(()),
            _ => Err(normal_error(response).await),
        }
    }
}
