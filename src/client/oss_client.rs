#[cfg(feature = "_async-base")]
use super::{DescribeRegions, ListBuckets};
#[cfg(feature = "_sync-base")]
use super::{DescribeRegionsSync, ListBucketsSync};
use crate::{OssBucket, oss::Oss};

/// Entry point for OSS, providing service-level APIs such as listing buckets and regions.
///
/// OSS 的入口类型，提供列举存储空间与地域等服务级 API。
#[derive(Debug, Clone)]
pub struct OssClient {
    pub(crate) oss: Oss,
}

impl OssClient {
    /// Create a new client with AccessKey credentials and region.
    ///
    /// `ak_id` is the AccessKey ID, `ak_secret` is the AccessKey Secret.
    ///
    /// `region` is required for Signature V4 (for example, `cn-hangzhou`).
    ///
    /// 使用 AccessKey 与地域创建客户端。
    ///
    /// `ak_id` 为 AccessKey ID，`ak_secret` 为 AccessKey Secret。
    ///
    /// `region` 为 Signature V4 必填（例如 `cn-hangzhou`）。
    pub fn new(ak_id: impl Into<String>, ak_secret: impl Into<String>, region: impl Into<String>) -> Self {
        OssClient { oss: Oss::new(ak_id, ak_secret, region) }
    }
    /// Disable HTTPS and use HTTP for all requests.
    ///
    /// 禁用 HTTPS，所有请求改为使用 HTTP。
    pub fn disable_https(mut self) -> Self {
        self.oss.set_https(false);
        self
    }
    /// Attach a temporary security token for STS authentication.
    ///
    /// 设置临时安全令牌用于 STS 鉴权。
    pub fn with_security_token(mut self, token: impl Into<String>) -> Self {
        self.oss.set_security_token(token);
        self
    }
    /// Override the endpoint used for subsequent requests.
    ///
    /// 覆盖后续请求使用的 Endpoint。
    pub fn set_endpoint(&mut self, endpoint: impl Into<String>) {
        self.oss.set_endpoint(endpoint);
    }
    /// Update the security token in place for reuse.
    ///
    /// 就地更新安全令牌，便于复用。
    pub fn set_security_token(&mut self, token: impl Into<String>) {
        self.oss.set_security_token(token);
    }
    /// Bind a bucket name and create a bucket handle.
    ///
    /// 绑定 Bucket 名称并创建桶句柄。
    pub fn bucket(&self, bucket: impl Into<String>) -> OssBucket {
        OssBucket::new(self.oss.clone(), bucket)
    }
    /// List OSS regions and their endpoints.
    ///
    /// 列举 OSS 支持的地域与对应的 Endpoint。
    #[cfg(feature = "_async-base")]
    pub fn describe_regions(&self) -> DescribeRegions {
        DescribeRegions::new(self.oss.clone())
    }
    /// List OSS regions and their endpoints (sync).
    ///
    /// 列举 OSS 支持的地域与对应的 Endpoint（同步）。
    #[cfg(feature = "_sync-base")]
    pub fn describe_regions_sync(&self) -> DescribeRegionsSync {
        DescribeRegionsSync::new(self.oss.clone())
    }
    /// List all buckets owned by the current account.
    ///
    /// 列举当前账号拥有的所有 Bucket。
    #[cfg(feature = "_async-base")]
    pub fn list_buckets(&self) -> ListBuckets {
        ListBuckets::new(self.oss.clone())
    }
    /// List all buckets owned by the current account (sync).
    ///
    /// 列举当前账号拥有的所有 Bucket（同步）。
    #[cfg(feature = "_sync-base")]
    pub fn list_buckets_sync(&self) -> ListBucketsSync {
        ListBucketsSync::new(self.oss.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_basic() {
        let client = OssClient::new("id", "secret", "cn-hangzhou");
        assert_eq!(client.oss.ak_id, "id");
        let client = client.clone().disable_https();
        assert!(!client.oss.enable_https);
        let client = client.clone().with_security_token("token");
        assert_eq!(client.oss.security_token.as_deref(), Some("token"));
        let mut client_mut = client.clone();
        client_mut.set_security_token("token2");
        assert_eq!(client_mut.oss.security_token.as_deref(), Some("token2"));
        client_mut.set_endpoint("endpoint");
        let bucket = client_mut.bucket("bucket");
        assert_eq!(bucket.oss.bucket.as_deref(), Some("bucket"));
        assert_eq!(bucket.oss.endpoint.as_ref(), "endpoint");
        assert_eq!(bucket.oss.security_token.as_deref(), Some("token2"));
    }
}
