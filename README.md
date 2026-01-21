# aliyun-oss-rs

[![Crates.io](https://img.shields.io/crates/v/aliyun-oss-rs)](https://crates.io/crates/aliyun-oss-rs)
[![Documentation](https://img.shields.io/badge/docs-latest-blue.svg)](https://docs.rs/aliyun-oss-rs)
[![License: 0BSD](https://img.shields.io/badge/license-0BSD-blue.svg)](LICENSE)

`aliyun-oss-rs` is an unofficial Rust SDK for Alibaba Cloud Object Storage Service (OSS).
It provides a small, chainable API surface. Async is enabled by default; a `sync` feature is available for selected APIs.
This SDK targets OSS Signature V4; `region` is required and drives the default endpoint selection.

## Install

Add the dependency in your `Cargo.toml`:

```toml
# Async by default
aliyun-oss-rs = { version = "0.2.0" }
```

Enable synchronous APIs if needed:

```toml
aliyun-oss-rs = { version = "0.2.0", features = ["sync"] }
```

## Quick Start (Async)

```rust
use aliyun_oss_rs::OssClient;

#[tokio::main]
async fn main() {
    let mut client = OssClient::new("<AccessKeyId>", "<AccessKeySecret>", "cn-zhangjiakou");
    // Optional: override endpoint for internal/dualstack/custom domains
    // client.set_endpoint("oss-cn-zhangjiakou-internal.aliyuncs.com");

    // List buckets
    let buckets = client
        .list_buckets()
        .set_prefix("rust")
        .send()
        .await;

    println!("buckets = {:?}", buckets);
}
```

> Need to work with STS credentials? Use `OssClient::with_security_token(token)` (or the corresponding bucket/object helpers) before sending requests.

## Common Operations (Async)

List objects in a bucket:

```rust
use aliyun_oss_rs::OssClient;

#[tokio::main]
async fn main() {
    let client = OssClient::new("<AccessKeyId>", "<AccessKeySecret>", "cn-zhangjiakou");
    let bucket = client.bucket("example-bucket");

    let objects = bucket
        .list_objects()
        .set_prefix("rust")
        .set_max_keys(200)
        .send()
        .await;

    println!("objects = {:?}", objects);
}
```

Upload a file from disk:

```rust
use aliyun_oss_rs::OssClient;

#[tokio::main]
async fn main() -> Result<(), aliyun_oss_rs::Error> {
    let client = OssClient::new("<AccessKeyId>", "<AccessKeySecret>", "cn-zhangjiakou");
    let bucket = client.bucket("example-bucket");
    let object = bucket.object("rust.png");

    object
        .put_object()
        .send_file("/path/to/file.png")
        .await?;

    Ok(())
}
```

Download to a local file:

```rust
use aliyun_oss_rs::OssClient;

#[tokio::main]
async fn main() -> Result<(), aliyun_oss_rs::Error> {
    let client = OssClient::new("<AccessKeyId>", "<AccessKeySecret>", "cn-zhangjiakou");
    let bucket = client.bucket("example-bucket");
    let object = bucket.object("rust.png");

    object
        .get_object()
        .download_to_file("./downloads/rust.png")
        .await?;

    Ok(())
}
```

Generate a pre-signed URL:

```rust
use aliyun_oss_rs::OssClient;
use time::{Duration, OffsetDateTime};

#[tokio::main]
async fn main() {
    let client = OssClient::new("<AccessKeyId>", "<AccessKeySecret>", "cn-zhangjiakou");
    let bucket = client.bucket("example-bucket");
    let object = bucket.object("rust.png");

    let expires = OffsetDateTime::now_utc() + Duration::hours(24);
    // Optionally bind a custom domain and HTTPS for the URL
    let url = object
        .get_object_url()
        .set_custom_domain("cdn.example.com", true)
        .url(expires);

    println!("signed url = {}", url);
}
```

Synchronous notes:

- When enabling `features = ["sync"]`, all APIs provide `*_sync` variants.
- Object APIs support streaming uploads/downloads in sync mode via blocking readers/writers.

## Implemented APIs

Doc last updated times are taken from the Alibaba Cloud OSS documentation (UTC).

### Basic

| API | Doc last updated (UTC) |
| --- | --- |
| ListBuckets | 2025-05-06 |
| DescribeRegions | 2025-05-06 |

### Bucket operations

| API | Doc last updated (UTC) |
| --- | --- |
| PutBucket | 2025-12-05 |
| DeleteBucket | 2025-05-06 |
| ListObjects | 2025-10-15 |
| ListObjectsV2 | 2026-01-20 |
| GetBucketInfo | 2025-12-05 |
| GetBucketStat | 2025-11-24 |
| DeleteMultipleObjects | 2026-01-08 |
| ListMultipartUploads | 2025-04-25 |
| GetBucketAcl | 2025-05-06 |
| PutBucketAcl | 2025-05-06 |
| GetBucketLocation | 2025-05-06 |
| GetBucketLogging | 2025-09-23 |
| PutBucketLogging | 2025-09-23 |
| DeleteBucketLogging | 2025-09-23 |
| GetBucketCors | 2025-05-06 |
| PutBucketCors | 2025-07-31 |
| DeleteBucketCors | 2025-05-06 |
| GetBucketLifecycle | 2025-05-06 |
| PutBucketLifecycle | 2025-07-18 |
| DeleteBucketLifecycle | 2025-05-06 |
| GetBucketReferer | 2025-05-06 |
| PutBucketReferer | 2025-05-06 |
| GetBucketWebsite | 2025-02-17 |
| PutBucketWebsite | 2025-12-09 |
| DeleteBucketWebsite | 2025-02-17 |
| GetBucketPolicy | 2025-09-23 |
| PutBucketPolicy | 2025-09-23 |
| DeleteBucketPolicy | 2025-09-23 |
| GetBucketEncryption | 2025-12-03 |
| PutBucketEncryption | 2025-12-03 |
| DeleteBucketEncryption | 2025-12-03 |
| PutBucketTransferAcceleration | 2025-05-06 |
| GetBucketTransferAcceleration | 2025-05-06 |
| PutBucketVersioning | 2025-02-13 |
| GetBucketVersioning | 2025-02-13 |
| ListObjectVersions | 2025-02-13 |
| PutBucketTags | 2025-05-06 |
| GetBucketTags | 2025-05-06 |
| DeleteBucketTags | 2025-05-06 |
| PutBucketRequestPayment | 2025-05-06 |
| GetBucketRequestPayment | 2025-05-06 |
| InitiateBucketWorm | 2025-12-09 |
| GetBucketWorm | 2025-05-06 |
| CompleteBucketWorm | 2025-05-06 |
| ExtendBucketWorm | 2025-05-06 |
| AbortBucketWorm | 2025-05-06 |
| PutBucketInventory | 2025-09-23 |
| GetBucketInventory | 2025-09-23 |
| ListBucketInventory | 2025-09-23 |
| DeleteBucketInventory | 2025-09-23 |

### Object operations

| API | Doc last updated (UTC) |
| --- | --- |
| GetObject | 2025-05-06 |
| GetObjectACL | 2025-05-06 |
| GetObjectMeta | 2025-05-06 |
| GetObjectUrl | 2026-01-20 |
| GetSymlink | 2025-05-06 |
| PutObject | 2025-09-23 |
| AppendObject | 2025-09-23 |
| PutSymlink | 2025-05-06 |
| CopyObject | 2025-05-06 |
| DeleteObject | 2025-05-06 |
| DeleteMultipleObjects | 2026-01-08 |
| HeadObject | 2025-05-06 |
| RestoreObject | 2025-05-06 |
| SelectObject | 2026-01-13 |

### Multipart upload operations

| API | Doc last updated (UTC) |
| --- | --- |
| InitiateMultipartUpload | 2025-04-25 |
| UploadPart | 2025-04-25 |
| UploadPartCopy | 2025-04-25 |
| CompleteMultipartUpload | 2025-04-25 |
| AbortMultipartUpload | 2025-04-25 |
| ListParts | 2025-04-25 |

Missing API categories:
- Replication and cross-region replication (CRR/RTC)
- Live channel and media processing
- CDN and image processing
- Bucket-level advanced security features beyond ACL/policy

---

# aliyun-oss-rs

[![Crates.io](https://img.shields.io/crates/v/aliyun-oss-rs)](https://crates.io/crates/aliyun-oss-rs)
[![Documentation](https://img.shields.io/badge/docs-latest-blue.svg)](https://docs.rs/aliyun-oss-rs)
[![License: 0BSD](https://img.shields.io/badge/license-0BSD-blue.svg)](LICENSE)

`aliyun-oss-rs` 是阿里云对象存储服务（OSS）的非官方 Rust SDK。
提供精简、可链式调用的 API。默认启用异步；开启 `sync` feature 后可使用部分同步 API。
本 SDK 使用 OSS Signature V4；必须提供 `region`，并由此推导默认 Endpoint。

## 安装

在 `Cargo.toml` 中添加依赖：

```toml
# 默认异步
aliyun-oss-rs = { version = "0.2.0" }
```

需要同步 API 时启用特性：

```toml
aliyun-oss-rs = { version = "0.2.0", features = ["sync"] }
```

## 快速开始（异步）

```rust
use aliyun_oss_rs::OssClient;

#[tokio::main]
async fn main() {
    let mut client = OssClient::new("<AccessKeyId>", "<AccessKeySecret>", "cn-zhangjiakou");
    // 可选：设置内网/双栈/自定义 Endpoint
    // client.set_endpoint("oss-cn-zhangjiakou-internal.aliyuncs.com");

    // 列举 Bucket
    let buckets = client
        .list_buckets()
        .set_prefix("rust")
        .send()
        .await;

    println!("buckets = {:?}", buckets);
}
```

> 需要 STS 临时凭证？可使用 `OssClient::with_security_token(token)`（或对应 bucket/object 的 helper）后再发送请求。

## 常用操作（异步）

列举 Bucket 中的对象：

```rust
use aliyun_oss_rs::OssClient;

#[tokio::main]
async fn main() {
    let client = OssClient::new("<AccessKeyId>", "<AccessKeySecret>", "cn-zhangjiakou");
    let bucket = client.bucket("example-bucket");

    let objects = bucket
        .list_objects()
        .set_prefix("rust")
        .set_max_keys(200)
        .send()
        .await;

    println!("objects = {:?}", objects);
}
```

上传本地文件：

```rust
use aliyun_oss_rs::OssClient;

#[tokio::main]
async fn main() -> Result<(), aliyun_oss_rs::Error> {
    let client = OssClient::new("<AccessKeyId>", "<AccessKeySecret>", "cn-zhangjiakou");
    let bucket = client.bucket("example-bucket");
    let object = bucket.object("rust.png");

    object
        .put_object()
        .send_file("/path/to/file.png")
        .await?;

    Ok(())
}
```

下载到本地文件：

```rust
use aliyun_oss_rs::OssClient;

#[tokio::main]
async fn main() -> Result<(), aliyun_oss_rs::Error> {
    let client = OssClient::new("<AccessKeyId>", "<AccessKeySecret>", "cn-zhangjiakou");
    let bucket = client.bucket("example-bucket");
    let object = bucket.object("rust.png");

    object
        .get_object()
        .download_to_file("./downloads/rust.png")
        .await?;

    Ok(())
}
```

生成预签名 URL：

```rust
use aliyun_oss_rs::OssClient;
use time::{Duration, OffsetDateTime};

#[tokio::main]
async fn main() {
    let client = OssClient::new("<AccessKeyId>", "<AccessKeySecret>", "cn-zhangjiakou");
    let bucket = client.bucket("example-bucket");
    let object = bucket.object("rust.png");

    let expires = OffsetDateTime::now_utc() + Duration::hours(24);
    // 可选：绑定自定义域名并使用 HTTPS
    let url = object
        .get_object_url()
        .set_custom_domain("cdn.example.com", true)
        .url(expires);

    println!("signed url = {}", url);
}
```

同步说明：

- 启用 `features = ["sync"]` 后，所有 API 均提供 `*_sync` 变体。
- Object 相关操作在同步模式下支持流式上传/下载（阻塞式读取/写入）。

## 已实现 API

文档更新时间取自阿里云 OSS 文档（UTC）。

### 基础

| API | 文档更新时间（UTC） |
| --- | --- |
| ListBuckets | 2025-05-06 |
| DescribeRegions | 2025-05-06 |

### Bucket 操作

| API | 文档更新时间（UTC） |
| --- | --- |
| PutBucket | 2025-12-05 |
| DeleteBucket | 2025-05-06 |
| ListObjects | 2025-10-15 |
| ListObjectsV2 | 2026-01-20 |
| GetBucketInfo | 2025-12-05 |
| GetBucketStat | 2025-11-24 |
| DeleteMultipleObjects | 2026-01-08 |
| ListMultipartUploads | 2025-04-25 |
| GetBucketAcl | 2025-05-06 |
| PutBucketAcl | 2025-05-06 |
| GetBucketLocation | 2025-05-06 |
| GetBucketLogging | 2025-09-23 |
| PutBucketLogging | 2025-09-23 |
| DeleteBucketLogging | 2025-09-23 |
| GetBucketCors | 2025-05-06 |
| PutBucketCors | 2025-07-31 |
| DeleteBucketCors | 2025-05-06 |
| GetBucketLifecycle | 2025-05-06 |
| PutBucketLifecycle | 2025-07-18 |
| DeleteBucketLifecycle | 2025-05-06 |
| GetBucketReferer | 2025-05-06 |
| PutBucketReferer | 2025-05-06 |
| GetBucketWebsite | 2025-02-17 |
| PutBucketWebsite | 2025-12-09 |
| DeleteBucketWebsite | 2025-02-17 |
| GetBucketPolicy | 2025-09-23 |
| PutBucketPolicy | 2025-09-23 |
| DeleteBucketPolicy | 2025-09-23 |
| GetBucketEncryption | 2025-12-03 |
| PutBucketEncryption | 2025-12-03 |
| DeleteBucketEncryption | 2025-12-03 |
| PutBucketTransferAcceleration | 2025-05-06 |
| GetBucketTransferAcceleration | 2025-05-06 |
| PutBucketVersioning | 2025-02-13 |
| GetBucketVersioning | 2025-02-13 |
| ListObjectVersions | 2025-02-13 |
| PutBucketTags | 2025-05-06 |
| GetBucketTags | 2025-05-06 |
| DeleteBucketTags | 2025-05-06 |
| PutBucketRequestPayment | 2025-05-06 |
| GetBucketRequestPayment | 2025-05-06 |
| InitiateBucketWorm | 2025-12-09 |
| GetBucketWorm | 2025-05-06 |
| CompleteBucketWorm | 2025-05-06 |
| ExtendBucketWorm | 2025-05-06 |
| AbortBucketWorm | 2025-05-06 |
| PutBucketInventory | 2025-09-23 |
| GetBucketInventory | 2025-09-23 |
| ListBucketInventory | 2025-09-23 |
| DeleteBucketInventory | 2025-09-23 |

### Object 操作

| API | 文档更新时间（UTC） |
| --- | --- |
| GetObject | 2025-05-06 |
| GetObjectACL | 2025-05-06 |
| GetObjectMeta | 2025-05-06 |
| GetObjectUrl | 2026-01-20 |
| GetSymlink | 2025-05-06 |
| PutObject | 2025-09-23 |
| AppendObject | 2025-09-23 |
| PutSymlink | 2025-05-06 |
| CopyObject | 2025-05-06 |
| DeleteObject | 2025-05-06 |
| DeleteMultipleObjects | 2026-01-08 |
| HeadObject | 2025-05-06 |
| RestoreObject | 2025-05-06 |
| SelectObject | 2026-01-13 |

### 分片上传

| API | 文档更新时间（UTC） |
| --- | --- |
| InitiateMultipartUpload | 2025-04-25 |
| UploadPart | 2025-04-25 |
| UploadPartCopy | 2025-04-25 |
| CompleteMultipartUpload | 2025-04-25 |
| AbortMultipartUpload | 2025-04-25 |
| ListParts | 2025-04-25 |

缺失 API 分类：
- 跨区域复制与实时同步（CRR/RTC）
- 直播通道与媒体处理
- CDN 与图片处理
- 超出 ACL/Policy 的高级安全能力
