//! Common data definitions.
//!
//! 通用数据定义。
#[cfg(any(feature = "_async-base", feature = "_sync-base"))]
use bytes::Bytes;
#[cfg(feature = "_async-base")]
use http_body_util::BodyExt;
#[cfg(feature = "_async-base")]
use hyper::body::Incoming;
use percent_encoding::{AsciiSet, NON_ALPHANUMERIC, utf8_percent_encode};
use serde_derive::{Deserialize, Serialize};
use std::fmt;
#[cfg(feature = "_sync-base")]
use std::io::Read;
use std::sync::OnceLock;
use time::{OffsetDateTime, UtcOffset, format_description};
#[cfg(feature = "_sync-base")]
use ureq::Body;

// -------------------------- Common functions --------------------------
// Encode query parameter values (RFC 3986, space as %20).
const URL_ENCODE: &AsciiSet = &NON_ALPHANUMERIC.remove(b'-').remove(b'_').remove(b'.').remove(b'~');
// Encode object key path segments while preserving '/'.
const URL_PATH_ENCODE: &AsciiSet = &NON_ALPHANUMERIC.remove(b'-').remove(b'_').remove(b'.').remove(b'~').remove(b'/');
#[inline]
pub(crate) fn url_encode(input: &str) -> String {
    utf8_percent_encode(input, URL_ENCODE).to_string()
}

#[inline]
pub(crate) fn url_encode_path(input: &str) -> String {
    utf8_percent_encode(input, URL_PATH_ENCODE).to_string()
}

// Check if metadata keys are valid
#[inline]
pub(crate) fn invalid_metadata_key(input: &str) -> bool {
    !input.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-')
}

#[cfg(feature = "_async-base")]
#[inline]
pub(crate) async fn body_to_bytes(body: Incoming) -> Result<Bytes, hyper::Error> {
    Ok(body.collect().await?.to_bytes())
}

#[cfg(feature = "_sync-base")]
#[inline]
pub(crate) fn body_to_bytes_sync(body: Body) -> Result<Bytes, std::io::Error> {
    let mut reader = body.into_reader();
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    Ok(Bytes::from(buf))
}

static GMT_FORMAT: OnceLock<Vec<format_description::FormatItem<'static>>> = OnceLock::new();

#[inline]
pub(crate) fn format_gmt(datetime: OffsetDateTime) -> String {
    let format = GMT_FORMAT.get_or_init(|| {
        format_description::parse("[weekday repr:short], [day] [month repr:short] [year] [hour]:[minute]:[second] GMT")
            .expect("valid format")
    });
    datetime.to_offset(UtcOffset::UTC).format(format).expect("formatting")
}

// -------------------------- Common data --------------------------

/// Access permissions (ACL).
///
/// 访问权限（ACL）。
#[derive(Debug, Deserialize, Clone)]
pub enum Acl {
    /// Only for object ACL; the object inherits the bucket ACL.
    ///
    /// 仅用于对象 ACL；表示对象继承 Bucket ACL。
    #[serde(rename = "default")]
    Default,
    /// Private; all requests require authorization.
    ///
    /// 私有；所有请求需要授权。
    #[serde(rename = "private")]
    Private,
    /// Public read; anonymous reads are allowed.
    ///
    /// 公共读；允许匿名读取。
    #[serde(rename = "public-read")]
    PublicRead,
    /// Public read/write; anonymous reads and writes are allowed.
    ///
    /// 公共读写；允许匿名读写。
    #[serde(rename = "public-read-write")]
    PublicReadWrite,
}
impl fmt::Display for Acl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Acl::Default => "default",
            Acl::Private => "private",
            Acl::PublicRead => "public-read",
            Acl::PublicReadWrite => "public-read-write",
        };
        write!(f, "{}", value)
    }
}

/// Storage class.
///
/// 存储类型。
#[derive(Debug, Clone, Serialize, Deserialize, Copy, PartialEq)]
pub enum StorageClass {
    /// Standard storage.
    ///
    /// 标准存储。
    Standard,
    /// Infrequent access.
    ///
    /// 低频访问。
    IA,
    /// Archive storage.
    ///
    /// 归档存储。
    Archive,
    /// Cold archive storage.
    ///
    /// 冷归档存储。
    ColdArchive,
    /// Deep cold archive storage.
    ///
    /// 深度冷归档存储。
    DeepColdArchive,
}
impl fmt::Display for StorageClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StorageClass::Standard => f.write_str("Standard"),
            StorageClass::IA => f.write_str("IA"),
            StorageClass::Archive => f.write_str("Archive"),
            StorageClass::ColdArchive => f.write_str("ColdArchive"),
            StorageClass::DeepColdArchive => f.write_str("DeepColdArchive"),
        }
    }
}

/// Data redundancy type.
///
/// 数据冗余类型。
#[derive(Debug, Clone, Serialize, Deserialize, Copy, PartialEq)]
pub enum DataRedundancyType {
    /// Local redundancy (LRS) stores data on multiple devices within one zone.
    ///
    /// 本地冗余（LRS）在同一可用区内多设备冗余存储。
    LRS,
    /// Zonal redundancy (ZRS) stores data across multiple zones in one region.
    ///
    /// 同城冗余（ZRS）在同一地域多可用区冗余存储。
    ZRS,
}
impl fmt::Display for DataRedundancyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataRedundancyType::LRS => f.write_str("LRS"),
            DataRedundancyType::ZRS => f.write_str("ZRS"),
        }
    }
}

/// Restore priority.
///
/// 解冻优先级。
#[derive(Debug, Clone, Serialize, Deserialize, Copy)]
pub enum RestoreTier {
    /// Expedited.
    ///
    /// 极速解冻。
    Expedited,
    /// Standard.
    ///
    /// 标准解冻。
    Standard,
    /// Bulk.
    ///
    /// 批量解冻。
    Bulk,
}
impl fmt::Display for RestoreTier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RestoreTier::Standard => f.write_str("Standard"),
            RestoreTier::Expedited => f.write_str("Expedited"),
            RestoreTier::Bulk => f.write_str("Bulk"),
        }
    }
}

/// HTTP header `Cache-Control`.
///
/// HTTP 头 `Cache-Control`。
#[derive(Debug, Clone)]
pub enum CacheControl {
    /// Do not use caches without revalidation.
    ///
    /// 不使用缓存或需重新验证。
    NoCache,
    /// Do not store in caches.
    ///
    /// 禁止缓存存储。
    NoStore,
    /// Public cache.
    ///
    /// 公共缓存。
    Public,
    /// Private cache.
    ///
    /// 私有缓存。
    Private,
    /// Maximum cache age (seconds).
    ///
    /// 最大缓存时长（秒）。
    MaxAge(u32),
}
impl fmt::Display for CacheControl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CacheControl::NoCache => f.write_str("no-cache"),
            CacheControl::NoStore => f.write_str("no-store"),
            CacheControl::Public => f.write_str("public"),
            CacheControl::Private => f.write_str("private"),
            CacheControl::MaxAge(val) => write!(f, "max-age={}", val),
        }
    }
}

/// HTTP header `Content-Disposition`.
///
/// HTTP 头 `Content-Disposition`。
#[derive(Debug, Clone)]
pub enum ContentDisposition {
    /// Display inline.
    ///
    /// 内联显示。
    Inline,
    /// Display as attachment.
    ///
    /// 作为附件下载。
    Attachment,
    /// Display as attachment with a new filename.
    ///
    /// 作为附件下载并指定新文件名。
    AttachmentWithNewName(String),
}
impl fmt::Display for ContentDisposition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ContentDisposition::Inline => f.write_str("inline"),
            ContentDisposition::AttachmentWithNewName(file_name) => {
                write!(f, "attachment;filename=\"{0}\";filename*=UTF-8''{0}", url_encode(file_name))
            }
            ContentDisposition::Attachment => f.write_str("attachment"),
        }
    }
}

/// Owner information.
///
/// 所有者信息。
#[derive(Debug, Deserialize)]
pub struct Owner {
    /// User ID.
    ///
    /// 用户 ID。
    #[serde(rename = "ID")]
    pub id: u64,
    /// User name.
    ///
    /// 用户名。
    #[serde(rename = "DisplayName")]
    pub display_name: String,
}

/// Result returned by `GetBucketAcl`, containing the owner and ACL.
///
/// `GetBucketAcl` 返回结果，包含所有者与 ACL。
#[derive(Debug)]
pub struct BucketAcl {
    pub owner: Owner,
    pub acl: Acl,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_encode_and_invalid_metadata_key() {
        assert_eq!(url_encode("a b"), "a%20b");
        assert_eq!(url_encode("a.b"), "a.b");
        assert!(!invalid_metadata_key("abc-123"));
        assert!(invalid_metadata_key("abc_123"));
    }

    #[test]
    fn test_cache_control_and_content_disposition() {
        assert_eq!(CacheControl::MaxAge(60).to_string(), "max-age=60");
        let name = "file name.txt";
        let expected = format!("attachment;filename=\"{0}\";filename*=UTF-8''{0}", url_encode(name));
        assert_eq!(ContentDisposition::AttachmentWithNewName(name.into()).to_string(), expected);
    }
}
