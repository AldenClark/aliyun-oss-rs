# Changelog

This project follows Semantic Versioning (SemVer). `0.1.0` is the first usable release.

## 0.1.1 - 2025-09-18

- Added STS-friendly helpers on clients, buckets, and objects; security tokens now flow automatically in async and sync requests.
- Expanded bucket management coverage with CORS, lifecycle, referer, website hosting, policy, default encryption, WORM retention, and inventory APIs.
- Implemented `SelectObject` for SQL-style queries and introduced a shared `MissingRequestBody` error to guard misconfigured builders.
- Updated documentation, README checklists, and internal tests.

## 0.1.0 - 2025-09-06

- Initial release: unofficial Rust SDK for Alibaba Cloud OSS.
- Simple, chainable API; async by default with optional `sync` feature.
- Covers common operations: list buckets/objects, upload/download, multipart upload, ACL, tagging, and pre-signed URLs.
