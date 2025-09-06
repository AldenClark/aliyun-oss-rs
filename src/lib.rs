//!
//! Alibaba Cloud Object Storage Service (OSS) is a massive, secure, low-cost, and reliable cloud storage service provided by Alibaba Cloud.
//!
//! Designed with simplicity and practicality in mind, it uses a chained structure (OssClient -> OssBucket -> OssObject -> Operation) to implement common APIs. Unsupported APIs will be added gradually.
//!
//! #### Notes
//! - Versioning is not supported yet; if your bucket has versioning enabled, functionality and data may be incomplete
//! - Server-side encryption is currently unsupported
//! - Most methods do not validate parameter characters. You must strictly follow OSS requirements or local or remote errors may occur
//!
//! ## Usage
//! ##### Initialization
//!  ```ignore
//! let client = OssClient::new(
//! "Your AccessKey ID",
//! "Your AccessKey Secret",
//! );
//!
//! ```
//!
//! ##### List buckets
//! ```ignore
//! let bucket_list = client.list_buckets().set_prefix("rust").send().await;
//!
//! ```
//!
//! ##### List objects in a bucket
//! ```ignore
//! let bucket = client.bucket("for-rs-test","oss-cn-zhangjiakou.aliyuncs.com");
//! let files = bucket.list_objects().send().await;
//! ```
//!
//! ##### Upload a file
//! ```ignore
//! let object = bucket.object("rust.png");
//! let result = object.put_object().send_file("Your File Path").await;
//! ```
//!
//! ##### Get an object's URL
//! ```ignore
//! use time::{Duration, OffsetDateTime};
//!
//! let date = OffsetDateTime::now_utc() + Duration::days(3);
//! let url = object.get_object_url().url(date);
//!
//! ```
//!

#[doc(inline)]
pub use crate::bucket::OssBucket;
#[doc(inline)]
pub use crate::client::OssClient;
#[doc(inline)]
pub use crate::error::Error;
#[cfg(feature = "async")]
#[doc(inline)]
pub use crate::object::OssObject;

pub mod bucket;
pub mod client;
pub mod common;
mod error;
#[cfg(feature = "async")]
pub mod object;
mod oss;
#[cfg(feature = "async")]
mod request;
#[cfg(feature = "sync")]
pub mod request_sync;
