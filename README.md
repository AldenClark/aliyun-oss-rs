# aliyun-oss-rs

[![Crates.io](https://img.shields.io/crates/v/aliyun-oss-rs)](https://crates.io/crates/aliyun-oss-rs)
[![Documentation](https://img.shields.io/badge/docs-latest-blue.svg)](https://docs.rs/aliyun-oss-rs)
[![License: 0BSD](https://img.shields.io/badge/license-0BSD-blue.svg)](LICENSE)

`aliyun-oss-rs` is an unofficial Rust SDK for Alibaba Cloud Object Storage Service (OSS).
It provides a simple, chainable API with minimal abstractions. Async is enabled by default; a `sync` feature is available for selected APIs.

## Install

Add the dependency in your `Cargo.toml`:

```toml
# Async by default
aliyun-oss-rs = { version = "0.1.1" }
```

Enable synchronous APIs if needed:

```toml
aliyun-oss-rs = { version = "0.1.1", features = ["sync"] }
```

## Quick Start (Async)

```rust
use aliyun_oss_rs::OssClient;

#[tokio::main]
async fn main() {
    let client = OssClient::new("<AccessKeyId>", "<AccessKeySecret>");

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
    let client = OssClient::new("<AccessKeyId>", "<AccessKeySecret>");
    let bucket = client.bucket("example-bucket", "oss-cn-zhangjiakou.aliyuncs.com");

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
    let client = OssClient::new("<AccessKeyId>", "<AccessKeySecret>");
    let bucket = client.bucket("example-bucket", "oss-cn-zhangjiakou.aliyuncs.com");
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
    let client = OssClient::new("<AccessKeyId>", "<AccessKeySecret>");
    let bucket = client.bucket("example-bucket", "oss-cn-zhangjiakou.aliyuncs.com");
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
    let client = OssClient::new("<AccessKeyId>", "<AccessKeySecret>");
    let bucket = client.bucket("example-bucket", "oss-cn-zhangjiakou.aliyuncs.com");
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

- When enabling `features = ["sync"]`, several bucket-level APIs offer `*_sync` variants (e.g. `get_bucket_location_sync`).
- For object operations that remain async-only, prefer using async directly for now;
  the `sync` feature set will continue to expand over time.

## Implemented APIs

- Basic
  - [x] List buckets (`ListBuckets`)
  - [x] List regions (`DescribeRegions`)

- Bucket operations
  - [x] Create bucket (`PutBucket`)
  - [x] Delete bucket (`DeleteBucket`)
  - [x] List objects in bucket (`ListObjectsV2`)
  - [x] Get bucket information (`GetBucketInfo`)
  - [x] Get bucket statistics (`GetBucketStat`)
  - [x] Delete multiple objects (`DeleteMultipleObjects`)
  - [x] List unfinished multipart uploads (`ListMultipartUploads`)
  - [x] Get bucket ACL (`GetBucketAcl`)
  - [x] Set bucket ACL (`PutBucketAcl`)
  - [x] Get bucket location (`GetBucketLocation`)
  - [x] Get bucket logging (`GetBucketLogging`)
  - [x] Set bucket logging (`PutBucketLogging`)
  - [x] Delete bucket logging (`DeleteBucketLogging`)
  - [x] Manage bucket CORS (`Get/Put/DeleteBucketCors`)
  - [x] Manage lifecycle rules (`Get/Put/DeleteBucketLifecycle`)
  - [x] Configure referer protection (`Get/Put/DeleteBucketReferer`)
  - [x] Configure static website hosting (`Get/Put/DeleteBucketWebsite`)
  - [x] Manage bucket policies (`Get/Put/DeleteBucketPolicy`)
  - [x] Manage default encryption (`Get/Put/DeleteBucketEncryption`)
  - [x] Work with WORM retention (`Initiate/Get/Complete/Extend/AbortBucketWorm`)
  - [x] Configure inventory reports (`Put/Get/Delete/ListBucketInventory`)

- Object operations
  - [x] Upload object (`PutObject`)
  - [x] Download object (`GetObject`)
  - [x] Query object with OSS Select (`SelectObject`)
  - [x] Copy object (`CopyObject`)
  - [x] Append object (`AppendObject`)
  - [x] Delete object (`DeleteObject`)
  - [x] Restore object (`RestoreObject`)
  - [x] Head object (`HeadObject`)
  - [x] Get object metadata (`GetObjectMeta`)
  - [x] Generate object URL (`GetObjectUrl`)
  - Multipart upload
    - [x] Initiate multipart upload (`InitiateMultipartUpload`)
    - [x] Upload part (`UploadPart`)
    - [x] Upload part copy (`UploadPartCopy`)
    - [x] Complete multipart upload (`CompleteMultipartUpload`)
    - [x] Abort multipart upload (`AbortMultipartUpload`)
    - [x] List parts (`ListParts`)
  - Object ACL
    - [x] Get object ACL (`GetObjectAcl`)
    - [x] Set object ACL (`PutObjectAcl`)
  - Tagging
    - [x] Get object tagging (`GetObjectTagging`)
    - [x] Set object tagging (`PutObjectTagging`)
    - [x] Delete object tagging (`DeleteObjectTagging`)
  - Symlink
    - [x] Create symlink (`PutSymlink`)
    - [x] Get symlink (`GetSymlink`)

This project is a work in progress and not an official Alibaba Cloud product.
Pull requests are welcome.
