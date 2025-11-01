//! Common data definitions
//!
//!
#[cfg(feature = "async")]
use bytes::Bytes;
#[cfg(feature = "async")]
use http_body_util::BodyExt;
#[cfg(feature = "async")]
use hyper::body::Incoming;
use percent_encoding::{AsciiSet, NON_ALPHANUMERIC, utf8_percent_encode};
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::sync::OnceLock;
use time::{OffsetDateTime, UtcOffset, format_description};

// -------------------------- Common functions --------------------------
// Encode query parameter values
const URL_ENCODE: &AsciiSet = &NON_ALPHANUMERIC.remove(b'-').remove(b'/');
#[inline]
pub(crate) fn url_encode(input: &str) -> String {
    utf8_percent_encode(input, URL_ENCODE).to_string()
}

// Check if metadata keys are valid
#[inline]
pub(crate) fn invalid_metadata_key(input: &str) -> bool {
    !input
        .bytes()
        .all(|b| b.is_ascii_alphanumeric() || b == b'-')
}

#[cfg(feature = "async")]
#[inline]
pub(crate) async fn body_to_bytes(body: Incoming) -> Result<Bytes, hyper::Error> {
    Ok(body.collect().await?.to_bytes())
}

static GMT_FORMAT: OnceLock<Vec<format_description::FormatItem<'static>>> = OnceLock::new();

#[inline]
pub(crate) fn format_gmt(datetime: OffsetDateTime) -> String {
    let format = GMT_FORMAT.get_or_init(|| {
        format_description::parse(
            "[weekday repr:short], [day] [month repr:short] [year] [hour]:[minute]:[second] GMT",
        )
        .expect("valid format")
    });
    datetime
        .to_offset(UtcOffset::UTC)
        .format(format)
        .expect("formatting")
}

/// Query parameters that should be included in the canonicalized resource
/// when signing requests. These values are derived from the officially
/// maintained SDKs for other languages to keep parity.
///
/// Parameters beginning with `x-oss-` are automatically included and therefore
/// omitted from this list.
pub(crate) const EXCLUDED_VALUES: [&str; 85] = [
    "accessPoint",
    "accessPointPolicy",
    "acl",
    "append",
    "asyncFetch",
    "bucketArchiveDirectRead",
    "bucketInfo",
    "callback",
    "callback-var",
    "cname",
    "comp",
    "continuation-token",
    "cors",
    "delete",
    "encryption",
    "endTime",
    "group",
    "httpsConfig",
    "img",
    "inventory",
    "inventoryId",
    "lifecycle",
    "link",
    "live",
    "location",
    "logging",
    "metaQuery",
    "objectInfo",
    "objectMeta",
    "partNumber",
    "policy",
    "policyStatus",
    "position",
    "processConfiguration",
    "publicAccessBlock",
    "qos",
    "qosInfo",
    "qosRequester",
    "redundancyTransition",
    "referer",
    "regionList",
    "replication",
    "replicationLocation",
    "replicationProgress",
    "requestPayment",
    "requesterQosInfo",
    "resourceGroup",
    "resourcePool",
    "resourcePoolBuckets",
    "resourcePoolInfo",
    "response-cache-control",
    "response-content-disposition",
    "response-content-encoding",
    "response-content-language",
    "response-content-type",
    "response-expires",
    "restore",
    "security-token",
    "sequential",
    "startTime",
    "stat",
    "status",
    "style",
    "styleName",
    "symlink",
    "tagging",
    "transferAcceleration",
    "udf",
    "udfApplication",
    "udfApplicationLog",
    "udfImage",
    "udfImageDesc",
    "udfName",
    "uploadId",
    "uploads",
    "versionId",
    "versioning",
    "versions",
    "vip",
    "vod",
    "vpcip",
    "website",
    "worm",
    "wormExtend",
    "wormId",
];

// -------------------------- Common data --------------------------

/// Access permissions (ACL)
#[derive(Debug, Deserialize, Clone)]
pub enum Acl {
    /// Only for object ACL; indicates the object's ACL inherits the bucket ACL
    #[serde(rename = "default")]
    Default,
    /// Private; all read and write requests require authorization
    #[serde(rename = "private")]
    Private,
    /// Public read; objects can be read anonymously but not written
    #[serde(rename = "public-read")]
    PublicRead,
    /// Public read/write; objects can be read and written anonymously
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

/// Storage class
#[derive(Debug, Clone, Serialize, Deserialize, Copy)]
pub enum StorageClass {
    /// Standard storage
    Standard,
    /// Infrequent access
    IA,
    /// Archive storage
    Archive,
    /// Cold archive storage
    ColdArchive,
    /// Deep cold archive storage
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

/// Data redundancy type
#[derive(Debug, Clone, Serialize, Deserialize, Copy)]
pub enum DataRedundancyType {
    /// Local redundancy (LRS) stores data on multiple devices within the same availability zone; up to two devices can fail simultaneously without data loss and access remains normal.
    LRS,
    /// Zonal redundancy (ZRS) stores data redundantly across multiple availability zones in the same region, ensuring access even if one zone becomes unavailable.
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

/// Restore priority
#[derive(Debug, Clone, Serialize, Deserialize, Copy)]
pub enum RestoreTier {
    /// Expedited
    Expedited,
    /// Standard
    Standard,
    /// Bulk
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

/// HTTP header: cache_control
#[derive(Debug, Clone)]
pub enum CacheControl {
    NoCache,
    NoStore,
    Public,
    Private,
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

/// HTTP header: content-disposition
#[derive(Debug, Clone)]
pub enum ContentDisposition {
    Inline,
    Attachment,
    AttachmentWithNewName(String),
}
impl fmt::Display for ContentDisposition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ContentDisposition::Inline => f.write_str("inline"),
            ContentDisposition::AttachmentWithNewName(file_name) => write!(
                f,
                "attachment;filename=\"{0}\";filename*=UTF-8''{0}",
                url_encode(file_name)
            ),
            ContentDisposition::Attachment => f.write_str("attachment"),
        }
    }
}

/// Owner information
#[derive(Debug, Deserialize)]
pub struct Owner {
    /// User ID
    #[serde(rename = "ID")]
    pub id: u64,
    /// User name
    #[serde(rename = "DisplayName")]
    pub display_name: String,
}

/// Result returned by `GetBucketAcl`, containing the owner and ACL.
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
        assert!(!invalid_metadata_key("abc-123"));
        assert!(invalid_metadata_key("abc_123"));
    }

    #[test]
    fn test_cache_control_and_content_disposition() {
        assert_eq!(CacheControl::MaxAge(60).to_string(), "max-age=60");
        let name = "file name.txt";
        let expected = format!(
            "attachment;filename=\"{0}\";filename*=UTF-8''{0}",
            url_encode(name)
        );
        assert_eq!(
            ContentDisposition::AttachmentWithNewName(name.into()).to_string(),
            expected
        );
    }
}
