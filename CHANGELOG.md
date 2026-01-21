# Changelog

This project follows Semantic Versioning (SemVer). `0.1.0` is the first usable release.

本项目遵循语义化版本（SemVer），`0.1.0` 为第一个可用版本。

## 0.2.0 - 2026-01-20

- Added bucket APIs for versioning, transfer acceleration, requester pays, bucket tags, list object versions, and ListObjects (v1).
- Upgraded request signing to OSS Signature V4 for headers and pre-signed URLs.
- Fixed object path encoding to keep dots unescaped in URLs.
- Added `Content-Length` for file uploads to avoid chunked transfer.
- Added synchronous variants for all APIs, including streaming upload/download support using ureq.
- Documented last-updated dates for implemented APIs in the README and summarized missing API categories.
- Updated dependencies to the latest crates.io releases (as of 2026-01-20).

- 新增 Bucket API：版本控制、传输加速、请求者付费、Bucket 标签、列举对象版本、ListObjects（v1）。
- 请求签名升级为 OSS Signature V4，覆盖请求头与预签名 URL。
- 修复对象路径编码，保持 URL 中的 `.` 不被转义。
- 上传文件时补充 `Content-Length`，避免 chunked 传输。
- 为所有 API 提供同步版本，并基于 ureq 支持流式上传/下载。
- 在 README 中记录已实现 API 的文档更新时间，并汇总缺失 API 分类。
- 依赖升级至 crates.io 最新版本（截至 2026-01-20）。

## 0.1.1 - 2025-09-18

- Added STS-friendly helpers on clients, buckets, and objects; security tokens now flow automatically in async and sync requests.
- Expanded bucket management coverage with CORS, lifecycle, referer, website hosting, policy, default encryption, WORM retention, and inventory APIs.
- Implemented `SelectObject` for SQL-style queries and introduced a shared `MissingRequestBody` error to guard misconfigured builders.
- Updated documentation, README checklists, and internal tests.

- 客户端、Bucket、Object 增加 STS 友好辅助方法；安全令牌自动在异步与同步请求中透传。
- 扩展 Bucket 管理能力：CORS、生命周期、Referer、静态网站托管、策略、默认加密、WORM 保留与清单。
- 实现 `SelectObject` SQL 风格查询，并引入共享的 `MissingRequestBody` 错误以防止配置错误。
- 更新文档、README 清单与内部测试。

## 0.1.0 - 2025-09-06

- Initial release: unofficial Rust SDK for Alibaba Cloud OSS.
- Simple, chainable API; async by default with optional `sync` feature.
- Covers common operations: list buckets/objects, upload/download, multipart upload, ACL, tagging, and pre-signed URLs.

- 初始发布：阿里云 OSS 非官方 Rust SDK。
- 链式调用 API；默认异步，可选 `sync` feature。
- 覆盖常用操作：列举 Bucket/对象、上传/下载、分片上传、ACL、标签与预签名 URL。
