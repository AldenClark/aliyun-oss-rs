use crate::{
    Error,
    error::normal_error_sync,
    request_sync::{Oss, OssRequest},
};
use http::Method;

/// Configure an access policy for the bucket.
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/zh/oss/developer-reference/putbucketpolicy) for details.
///
/// 配置 Bucket 访问策略。
///
/// 详情参见 [阿里云文档](https://help.aliyun.com/zh/oss/developer-reference/putbucketpolicy)。
pub struct PutBucketPolicySync {
    req: OssRequest,
    policy: Option<String>,
}

impl PutBucketPolicySync {
    pub(super) fn new(oss: Oss) -> Self {
        let mut req = OssRequest::new(oss, Method::PUT);
        req.insert_query("policy", "");
        PutBucketPolicySync { req, policy: None }
    }

    /// Set the policy document in JSON format.
    ///
    /// 设置 JSON 格式的策略文档。
    pub fn set_policy(mut self, policy: impl Into<String>) -> Self {
        self.policy = Some(policy.into());
        self
    }

    /// Send the request.
    ///
    /// 发送请求。
    pub fn send(mut self) -> Result<(), Error> {
        let body = self.policy.ok_or(Error::MissingRequestBody)?;
        self.req.set_body(body.into_bytes());
        let response = self.req.send_to_oss()?;
        match response.status() {
            code if code.is_success() => Ok(()),
            _ => Err(normal_error_sync(response)),
        }
    }
}
