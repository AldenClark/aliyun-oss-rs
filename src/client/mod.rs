//! Basic client service including AccessKey and endpoint information.
//!
//! 基础客户端服务，包含 AccessKey 与 Endpoint 等信息。

#[cfg(feature = "async")]
pub use self::describe_regions::DescribeRegions;
#[cfg(feature = "async")]
pub use self::list_buckets::ListBuckets;
#[cfg(feature = "sync")]
pub use self::describe_regions_sync::DescribeRegionsSync;
#[cfg(feature = "sync")]
pub use self::list_buckets_sync::ListBucketsSync;
pub use self::oss_client::OssClient;

#[cfg(feature = "async")]
mod describe_regions;
#[cfg(feature = "async")]
mod list_buckets;
#[cfg(feature = "sync")]
mod describe_regions_sync;
#[cfg(feature = "sync")]
mod list_buckets_sync;
mod oss_client;
