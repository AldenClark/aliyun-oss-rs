//! Basic client service including AccessKey and endpoint information.
//!
//! 基础客户端服务，包含 AccessKey 与 Endpoint 等信息。

#[cfg(feature = "_async-base")]
pub use self::describe_regions::DescribeRegions;
#[cfg(feature = "_sync-base")]
pub use self::describe_regions_sync::DescribeRegionsSync;
#[cfg(feature = "_async-base")]
pub use self::list_buckets::ListBuckets;
#[cfg(feature = "_sync-base")]
pub use self::list_buckets_sync::ListBucketsSync;
pub use self::oss_client::OssClient;

#[cfg(feature = "_async-base")]
mod describe_regions;
#[cfg(feature = "_sync-base")]
mod describe_regions_sync;
#[cfg(feature = "_async-base")]
mod list_buckets;
#[cfg(feature = "_sync-base")]
mod list_buckets_sync;
mod oss_client;
