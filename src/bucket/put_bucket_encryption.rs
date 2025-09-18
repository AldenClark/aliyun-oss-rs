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
pub struct PutBucketEncryption {
    req: OssRequest,
    encryption: BucketEncryption,
}

impl PutBucketEncryption {
    pub(super) fn new(oss: Oss) -> Self {
        let mut req = OssRequest::new(oss, Method::PUT);
        req.insert_query("encryption", "");
        PutBucketEncryption {
            req,
            encryption: BucketEncryption::default(),
        }
    }

    /// Set the encryption algorithm (for example, `AES256`, `KMS`).
    pub fn set_algorithm(mut self, algorithm: impl ToString) -> Self {
        self.encryption.rule.default_sse.sse_algorithm = algorithm.to_string();
        self
    }

    /// Set the KMS master key ID. Only valid when the algorithm is `KMS`.
    pub fn set_kms_master_key_id(mut self, key_id: impl ToString) -> Self {
        self.encryption.rule.default_sse.kms_master_key_id = Some(key_id.to_string());
        self
    }

    /// Replace the entire encryption document.
    pub fn set_encryption(mut self, encryption: BucketEncryption) -> Self {
        self.encryption = encryption;
        self
    }

    /// Send the request.
    pub async fn send(mut self) -> Result<(), Error> {
        let body =
            serde_xml_rs::to_string(&self.encryption).map_err(|_| Error::InvalidCharacter)?;
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
