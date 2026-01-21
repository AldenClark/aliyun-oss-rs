use crate::{
    Error,
    common::body_to_bytes_sync,
    error::normal_error_sync,
    request_sync::{Oss, OssRequest},
};
use http::Method;

/// Retrieve the bucket policy document.
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/zh/oss/developer-reference/getbucketpolicy) for details.
///
/// 获取 Bucket 策略文档。
///
/// 详情参见 [阿里云文档](https://help.aliyun.com/zh/oss/developer-reference/getbucketpolicy)。
pub struct GetBucketPolicySync {
    req: OssRequest,
}

impl GetBucketPolicySync {
    pub(super) fn new(oss: Oss) -> Self {
        let mut req = OssRequest::new(oss, Method::GET);
        req.insert_query("policy", "");
        GetBucketPolicySync { req }
    }

    /// Send the request and return the policy JSON.
    ///
    /// 发送请求并返回策略 JSON。
    pub fn send(self) -> Result<String, Error> {
        let response = self.req.send_to_oss()?;
        match response.status() {
            code if code.is_success() => {
                let bytes = body_to_bytes_sync(response.into_body())?;
                Ok(String::from_utf8_lossy(bytes.as_ref()).into_owned())
            }
            _ => Err(normal_error_sync(response)),
        }
    }
}
