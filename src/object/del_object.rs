use crate::{
    Error,
    error::normal_error,
    request::{Oss, OssRequest},
};
use http::Method;

/// Delete an object.
///
/// OSS does not check object existence; a valid request usually succeeds.
///
/// If versioning is enabled, response headers carry delete marker and version ID info.
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/document_detail/31982.html) for details.
///
/// 删除对象。
///
/// OSS 不检查对象是否存在，合法请求通常会成功。
///
/// 若开启版本控制，响应头会包含删除标记与版本 ID 信息。
///
/// 详情参见 [阿里云文档](https://help.aliyun.com/document_detail/31982.html)。
pub struct DelObject {
    req: OssRequest,
}
impl DelObject {
    pub(super) fn new(oss: Oss) -> Self {
        DelObject {
            req: OssRequest::new(oss, Method::DELETE),
        }
    }
    /// Send the delete request.
    ///
    /// Response headers are meaningful only when versioning is enabled.
    ///
    /// `x-oss-delete-marker` indicates delete marker; `x-oss-version-id` is the deleted version ID.
    ///
    /// 发送删除请求。
    ///
    /// 仅在开启版本控制时响应头才有意义。
    ///
    /// `x-oss-delete-marker` 表示删除标记；`x-oss-version-id` 表示删除的版本 ID。
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
