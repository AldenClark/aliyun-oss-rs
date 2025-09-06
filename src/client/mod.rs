//! Basic service including AccessKey and endpoint information

#[cfg(feature = "async")]
pub use self::describe_regions::DescribeRegions;
#[cfg(feature = "async")]
pub use self::list_buckets::ListBuckets;
pub use self::oss_client::OssClient;

#[cfg(feature = "async")]
mod describe_regions;
#[cfg(feature = "async")]
mod list_buckets;
mod oss_client;
