use crate::{
    Error,
    error::normal_error,
    request::{Oss, OssRequest},
};
use bytes::Bytes;
use http::Method;
use http_body_util::Full;

use super::BucketEncryption;

/// Configure default server-side encryption for the bucket.
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/zh/oss/developer-reference/putbucketencryption) for details.
///
/// 配置 Bucket 默认服务端加密。
///
/// 详情参见 [阿里云文档](https://help.aliyun.com/zh/oss/developer-reference/putbucketencryption)。
pub struct PutBucketEncryption {
    req: OssRequest,
    encryption: BucketEncryption,
}

impl PutBucketEncryption {
    pub(super) fn new(oss: Oss) -> Self {
        let mut req = OssRequest::new(oss, Method::PUT);
        req.insert_query("encryption", "");
        PutBucketEncryption { req, encryption: BucketEncryption::default() }
    }

    /// Set the encryption algorithm (e.g., `AES256`, `KMS`).
    ///
    /// 设置加密算法（如 `AES256`、`KMS`）。
    pub fn set_algorithm(mut self, algorithm: impl Into<String>) -> Self {
        self.encryption.rule.default_sse.sse_algorithm = algorithm.into();
        self
    }

    /// Set the KMS master key ID (only valid when algorithm is `KMS`).
    ///
    /// 设置 KMS 主密钥 ID（仅在算法为 `KMS` 时生效）。
    pub fn set_kms_master_key_id(mut self, key_id: impl Into<String>) -> Self {
        self.encryption.rule.default_sse.kms_master_key_id = Some(key_id.into());
        self
    }

    /// Replace the entire encryption document.
    ///
    /// 替换整个加密配置文档。
    pub fn set_encryption(mut self, encryption: BucketEncryption) -> Self {
        self.encryption = encryption;
        self
    }

    /// Send the request.
    ///
    /// 发送请求。
    pub async fn send(mut self) -> Result<(), Error> {
        let body = serde_xml_rs::to_string(&self.encryption).map_err(|_| Error::InvalidCharacter)?;
        self.req.set_body(Full::new(Bytes::from(body)));
        let response = self.req.send_to_oss()?.await?;
        match response.status() {
            code if code.is_success() => Ok(()),
            _ => Err(normal_error(response).await),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encryption_serialization() {
        let xml = serde_xml_rs::to_string(&BucketEncryption::default()).unwrap();
        assert!(xml.contains("<ServerSideEncryptionConfiguration>"));
        assert!(xml.contains("<SSEAlgorithm>AES256</SSEAlgorithm>"));
    }
}
