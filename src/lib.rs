//! # aliyun-oss-rs
//!
//! `aliyun-oss-rs` is an unofficial Rust SDK for Alibaba Cloud Object Storage Service (OSS).
//! It provides a small, chainable API surface: `OssClient -> OssBucket -> OssObject -> Operation`.
//! Async APIs are enabled by default; a `sync` feature exposes a blocking subset.
//!
//! ## Feature flags
//!
//! ```toml
//! aliyun-oss-rs = { version = "0.2.0" } # async by default
//! ```
//!
//! ```toml
//! aliyun-oss-rs = { version = "0.2.0", default-features = false, features = ["sync"] }
//! ```
//!
//! ## Async and sync usage
//!
//! - Async APIs require a Tokio runtime.
//! - When `sync` is enabled, all APIs provide `*_sync` variants.
//!   Object APIs support streaming upload/download with blocking readers/writers.
//!
//! ## Regions and endpoints
//!
//! Signature V4 requires a region. `OssClient::new` takes a region and derives the default
//! public endpoint as `oss-<region>.aliyuncs.com`. Use `set_endpoint` for internal,
//! dualstack, or custom domains.
//!
//! ## Quick start (async)
//!
//! ```ignore
//! use aliyun_oss_rs::OssClient;
//!
//! #[tokio::main]
//! async fn main() {
//!     let mut client = OssClient::new("<AccessKeyId>", "<AccessKeySecret>", "cn-zhangjiakou");
//!     // Optional: internal/dualstack/custom endpoints
//!     // client.set_endpoint("oss-cn-zhangjiakou-internal.aliyuncs.com");
//!
//!     let buckets = client.list_buckets().set_prefix("rust").send().await;
//!     println!("buckets = {:?}", buckets);
//! }
//! ```
//!
//! ## Working with a bucket
//!
//! ```ignore
//! use aliyun_oss_rs::OssClient;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), aliyun_oss_rs::Error> {
//!     let client = OssClient::new("<AccessKeyId>", "<AccessKeySecret>", "cn-zhangjiakou");
//!     let bucket = client.bucket("example-bucket");
//!     let object = bucket.object("rust.png");
//!
//!     object.put_object().send_file("/path/to/file.png").await?;
//!     Ok(())
//! }
//! ```
//!
//! ## Pre-signed URL
//!
//! ```ignore
//! use aliyun_oss_rs::OssClient;
//! use time::{Duration, OffsetDateTime};
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = OssClient::new("<AccessKeyId>", "<AccessKeySecret>", "cn-zhangjiakou");
//!     let url = client
//!         .bucket("example-bucket")
//!         .object("rust.png")
//!         .get_object_url()
//!         .url(OffsetDateTime::now_utc() + Duration::hours(24));
//!
//!     println!("url = {}", url);
//! }
//! ```
//!
//! ---
//!
//! # aliyun-oss-rs
//!
//! `aliyun-oss-rs` 是阿里云对象存储服务（OSS）的非官方 Rust SDK。
//! 提供精简、可链式调用的 API：`OssClient -> OssBucket -> OssObject -> Operation`。
//! 默认启用异步 API；开启 `sync` feature 后提供同步子集。
//!
//! ## 功能特性
//!
//! ```toml
//! aliyun-oss-rs = { version = "0.2.0" } # 默认异步
//! ```
//!
//! ```toml
//! aliyun-oss-rs = { version = "0.2.0", default-features = false, features = ["sync"] }
//! ```
//!
//! ## 异步与同步
//!
//! - 异步 API 需要 Tokio runtime。
//! - 启用 `sync` 后，所有 API 均提供 `*_sync` 变体。
//!   Object 相关 API 支持阻塞式流上传/下载。
//!
//! ## Region 与 Endpoint
//!
//! Signature V4 需要提供 region。`OssClient::new` 会根据 region
//! 推导默认公网 Endpoint：`oss-<region>.aliyuncs.com`。
//! 如需内网、双栈或自定义域名，请使用 `set_endpoint` 覆盖。
//!
//! ## 快速开始（异步）
//!
//! ```ignore
//! use aliyun_oss_rs::OssClient;
//!
//! #[tokio::main]
//! async fn main() {
//!     let mut client = OssClient::new("<AccessKeyId>", "<AccessKeySecret>", "cn-zhangjiakou");
//!     // 可选：内网/双栈/自定义 Endpoint
//!     // client.set_endpoint("oss-cn-zhangjiakou-internal.aliyuncs.com");
//!
//!     let buckets = client.list_buckets().set_prefix("rust").send().await;
//!     println!("buckets = {:?}", buckets);
//! }
//! ```
//!
//! ## Bucket 常用操作
//!
//! ```ignore
//! use aliyun_oss_rs::OssClient;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), aliyun_oss_rs::Error> {
//!     let client = OssClient::new("<AccessKeyId>", "<AccessKeySecret>", "cn-zhangjiakou");
//!     let bucket = client.bucket("example-bucket");
//!     let object = bucket.object("rust.png");
//!
//!     object.put_object().send_file("/path/to/file.png").await?;
//!     Ok(())
//! }
//! ```
//!
//! ## 预签名 URL
//!
//! ```ignore
//! use aliyun_oss_rs::OssClient;
//! use time::{Duration, OffsetDateTime};
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = OssClient::new("<AccessKeyId>", "<AccessKeySecret>", "cn-zhangjiakou");
//!     let url = client
//!         .bucket("example-bucket")
//!         .object("rust.png")
//!         .get_object_url()
//!         .url(OffsetDateTime::now_utc() + Duration::hours(24));
//!
//!     println!("url = {}", url);
//! }
//! ```

#[doc(inline)]
pub use crate::bucket::OssBucket;
#[doc(inline)]
pub use crate::client::OssClient;
#[doc(inline)]
pub use crate::error::Error;

#[cfg(all(feature = "_async-base", not(any(feature = "async", feature = "async-native-tls"))))]
compile_error!("Internal feature `_async-base` is not supported directly; enable `async` or `async-native-tls`.");

#[cfg(all(feature = "_async-rustls", not(feature = "async")))]
compile_error!("Internal feature `_async-rustls` is not supported directly; enable `async`.");

#[cfg(all(feature = "_sync-base", not(any(feature = "sync", feature = "sync-native-tls"))))]
compile_error!("Internal feature `_sync-base` is not supported directly; enable `sync` or `sync-native-tls`.");

#[cfg(all(feature = "_sync-rustls", not(feature = "sync")))]
compile_error!("Internal feature `_sync-rustls` is not supported directly; enable `sync`.");
#[cfg(any(feature = "_async-base", feature = "_sync-base"))]
#[doc(inline)]
pub use crate::object::OssObject;

pub mod bucket;
pub mod client;
pub mod common;
mod error;
#[cfg(any(feature = "_async-base", feature = "_sync-base"))]
pub mod object;
mod oss;
#[cfg(feature = "_async-base")]
mod request;
#[cfg(feature = "_sync-base")]
pub mod request_sync;
