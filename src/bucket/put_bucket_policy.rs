use crate::{
    Error,
    error::normal_error,
    request::{Oss, OssRequest},
};
use bytes::Bytes;
use http::Method;
use http_body_util::Full;

/// Configure an access policy for the bucket.
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/zh/oss/developer-reference/putbucketpolicy) for details.
///
/// 配置 Bucket 访问策略。
///
/// 详情参见 [阿里云文档](https://help.aliyun.com/zh/oss/developer-reference/putbucketpolicy)。
pub struct PutBucketPolicy {
    req: OssRequest,
    policy: Option<String>,
}

impl PutBucketPolicy {
    pub(super) fn new(oss: Oss) -> Self {
        let mut req = OssRequest::new(oss, Method::PUT);
        req.insert_query("policy", "");
        PutBucketPolicy { req, policy: None }
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
    pub async fn send(mut self) -> Result<(), Error> {
        let body = self.policy.ok_or(Error::MissingRequestBody)?;
        self.req.set_body(Full::new(Bytes::from(body)));
        let response = self.req.send_to_oss()?.await?;
        match response.status() {
            code if code.is_success() => Ok(()),
            _ => Err(normal_error(response).await),
        }
    }
}
