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
    pub fn set_policy(mut self, policy: impl ToString) -> Self {
        self.policy = Some(policy.to_string());
        self
    }

    /// Send the request.
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
