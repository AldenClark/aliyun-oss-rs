#[cfg(feature = "async")]
use super::{DescribeRegions, ListBuckets};
use crate::{oss::Oss, OssBucket};

/// Entry point for OSS, implementing APIs to query available regions and list buckets
#[derive(Debug, Clone)]
pub struct OssClient {
    pub(crate) oss: Oss,
}

impl OssClient {
    /// Initialize an OssClient for subsequent use
    ///
    /// - ak_id: Alibaba Cloud AccessKey ID
    /// - ak_secret: Alibaba Cloud AccessKey Secret
    ///
    pub fn new(ak_id: &str, ak_secret: &str) -> Self {
        OssClient {
            oss: Oss::new(ak_id, ak_secret),
        }
    }
    /// Disable HTTPS
    pub fn disable_https(mut self) -> Self {
        self.oss.set_https(false);
        self
    }
    /// Initialize an OssBucket
    pub fn bucket(&self, bucket: &str, endpoint: &str) -> OssBucket {
        OssBucket::new(self.oss.clone(), bucket, endpoint)
    }
    /// Query the endpoint information of all regions
    #[cfg(feature = "async")]
    pub fn describe_regions(&self) -> DescribeRegions {
        DescribeRegions::new(self.oss.clone())
    }
    /// List all created buckets
    #[cfg(feature = "async")]
    pub fn list_buckets(&self) -> ListBuckets {
        ListBuckets::new(self.oss.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_basic() {
        let client = OssClient::new("id", "secret");
        assert_eq!(client.oss.ak_id, "id");
        let client = client.clone().disable_https();
        assert!(!client.oss.enable_https);
        let bucket = client.bucket("bucket", "endpoint");
        assert_eq!(bucket.oss.bucket.as_deref(), Some("bucket"));
        assert_eq!(bucket.oss.endpoint.as_ref(), "endpoint");
    }
}
