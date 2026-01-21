use crate::{
    Error,
    common::{url_encode, url_encode_path},
};
use aws_lc_rs::{digest, hmac};
use bytes::Bytes;
use http::{Method, header};
use http_body::Body as HttpBody;
use http_body_util::{BodyExt, Empty, combinators::BoxBody};
use hyper::Request;
#[cfg(feature = "_async-rustls")]
use hyper_rustls::HttpsConnectorBuilder;
#[cfg(feature = "async-native-tls")]
use hyper_tls::HttpsConnector;
use hyper_util::client::legacy::{Client, ResponseFuture};
use hyper_util::rt::TokioExecutor;
use std::borrow::Cow;
use std::collections::HashMap;
use std::error::Error as StdError;
use time::OffsetDateTime;

pub(crate) use crate::oss::Oss;

#[cfg(all(feature = "_async-rustls", feature = "async-native-tls"))]
compile_error!(
    "Async TLS backend conflict: enable exactly one of `async` (default, rustls) or `async-native-tls`."
);

#[cfg(all(feature = "_async-base", not(any(feature = "_async-rustls", feature = "async-native-tls"))))]
compile_error!(
    "Async TLS backend missing: enable `async` (default, rustls) or `async-native-tls`."
);

/// Builder for requests sent to OSS.
///
/// OSS 请求构建器。
#[derive(Debug)]
pub(crate) struct OssRequest {
    pub(crate) oss: Oss,
    method: Method,
    headers: HashMap<String, String>,
    queries: HashMap<String, String>,
    body: BoxBody<Bytes, Box<dyn StdError + Send + Sync>>,
}

impl OssRequest {
    /// Create a new request builder with default empty headers, queries and body.
    ///
    /// 创建请求构建器，默认头、查询参数和请求体为空。
    pub fn new(oss: Oss, method: Method) -> Self {
        OssRequest {
            oss,
            method,
            headers: HashMap::with_capacity(10),
            queries: HashMap::with_capacity(10),
            body: Empty::<Bytes>::new().map_err(|never| -> Box<dyn StdError + Send + Sync> { match never {} }).boxed(),
        }
    }

    /// Override the endpoint used for the request.
    ///
    /// 覆盖本次请求使用的 Endpoint。
    pub fn set_endpoint(&mut self, endpoint: impl Into<String>) -> &mut Self {
        self.oss.endpoint = Cow::Owned(endpoint.into());
        self
    }

    /// Enable or disable HTTPS.
    ///
    /// 启用或禁用 HTTPS。
    pub fn set_https(&mut self, https: bool) -> &mut Self {
        self.oss.enable_https = https;
        self
    }

    /// Insert a header key/value pair.
    ///
    /// 插入请求头键值对。
    pub fn insert_header(&mut self, key: impl Into<String>, value: impl Into<String>) -> &mut Self {
        self.headers.insert(key.into(), value.into());
        self
    }

    /// Insert a query parameter.
    ///
    /// 插入查询参数。
    pub fn insert_query(&mut self, key: impl Into<String>, value: impl Into<String>) -> &mut Self {
        self.queries.insert(key.into(), value.into());
        self
    }

    /// Set the request body.
    ///
    /// 设置请求体。
    pub fn set_body<B>(&mut self, body: B) -> &mut Self
    where
        B: HttpBody<Data = Bytes> + Send + Sync + 'static,
        B::Error: Into<Box<dyn StdError + Send + Sync>>,
    {
        self.body = body.map_err(Into::into).boxed();
        self
    }

    pub fn uri(&self) -> String {
        // protocol
        let protocol = if self.oss.enable_https { "https://" } else { "http://" };
        // host
        let host = if let Some(custom_domain) = self.oss.custom_domain.clone() {
            custom_domain.to_string()
        } else {
            format!(
                "{}{}",
                self.oss.bucket.clone().map(|v| format!("{}.", v)).unwrap_or_else(|| String::new()),
                self.oss.endpoint
            )
        };
        // query string
        let query = self
            .queries
            .iter()
            .map(|(key, value)| {
                let key = url_encode(key);
                let value = value.to_string();
                if value.is_empty() { key } else { format!("{}={}", key, url_encode(&value)) }
            })
            .collect::<Vec<_>>()
            .join("&");
        let query_str = if query.is_empty() { String::new() } else { format!("?{}", query) };
        // build final url
        format!(
            "{}{}/{}{}",
            protocol,
            host,
            url_encode_path(&self.oss.object.clone().unwrap_or_else(|| String::new().into())),
            query_str
        )
    }
    pub fn query_sign(&mut self, expires: OffsetDateTime) {
        let now = OffsetDateTime::now_utc();
        let expires = (expires - now).whole_seconds().max(1);
        let date = format_oss_date(now);
        let date_short = format_oss_date_short(now);
        let region = self.oss.region.to_string();

        self.insert_query("x-oss-signature-version", "OSS4-HMAC-SHA256");
        self.insert_query(
            "x-oss-credential",
            format!("{}/{}/{}/oss/aliyun_v4_request", self.oss.ak_id, date_short, region),
        );
        self.insert_query("x-oss-date", &date);
        self.insert_query("x-oss-expires", expires.to_string());
        if let Some(token) = self.oss.security_token.clone() {
            self.insert_query("x-oss-security-token", token);
        }

        let additional_headers = additional_headers_v4(&self.headers);
        if !additional_headers.is_empty() {
            self.insert_query("x-oss-additional-headers", additional_headers.join(";"));
        }

        let canonical_request = self.canonical_request_v4(&additional_headers);
        let signature = self.signature_v4(&canonical_request, &date, &date_short, &region);
        self.insert_query("x-oss-signature", signature);
    }
    pub fn header_sign(&mut self) {
        let now = OffsetDateTime::now_utc();
        let date = format_oss_date(now);
        let date_short = format_oss_date_short(now);
        let region = self.oss.region.to_string();

        self.insert_header("x-oss-date", &date);
        self.insert_header("x-oss-content-sha256", "UNSIGNED-PAYLOAD");

        let additional_headers = additional_headers_v4(&self.headers);
        let canonical_request = self.canonical_request_v4(&additional_headers);
        let signature = self.signature_v4(&canonical_request, &date, &date_short, &region);

        let credential = format!("{}/{}/{}/oss/aliyun_v4_request", self.oss.ak_id, date_short, region);
        let authorization = if additional_headers.is_empty() {
            format!("OSS4-HMAC-SHA256 Credential={},Signature={}", credential, signature)
        } else {
            format!(
                "OSS4-HMAC-SHA256 Credential={},AdditionalHeaders={},Signature={}",
                credential,
                additional_headers.join(";"),
                signature
            )
        };
        self.insert_header(header::AUTHORIZATION.as_str(), authorization);
    }

    fn apply_security_token(&mut self) {
        if let Some(security_token) = self.oss.security_token.clone() {
            self.insert_header("x-oss-security-token", security_token);
        }
    }

    pub fn send_to_oss(mut self) -> Result<ResponseFuture, Error> {
        // insert temporary security token if provided
        self.apply_security_token();
        // ensure required V4 headers exist before signing
        if !self.headers.contains_key("x-oss-content-sha256") {
            self.insert_header("x-oss-content-sha256", "UNSIGNED-PAYLOAD");
        }
        // sign headers
        self.header_sign();
        // build http request
        let mut req = Request::builder().method(&self.method).uri(&self.uri());
        for (key, value) in self.headers.iter() {
            req = req.header(key, value);
        }
        let request = req.body(self.body)?;
        if self.oss.enable_https {
            #[cfg(feature = "async-native-tls")]
            let https = HttpsConnector::new();
            #[cfg(feature = "_async-rustls")]
            let https = HttpsConnectorBuilder::new().with_webpki_roots().https_or_http().enable_http1().build();
            let client: Client<_, BoxBody<Bytes, Box<dyn StdError + Send + Sync>>> =
                Client::builder(TokioExecutor::new()).build(https);
            Ok(client.request(request))
        } else {
            let client: Client<_, BoxBody<Bytes, Box<dyn StdError + Send + Sync>>> =
                Client::builder(TokioExecutor::new()).build_http();
            Ok(client.request(request))
        }
    }
}

fn format_oss_date(datetime: OffsetDateTime) -> String {
    datetime
        .format(&time::format_description::parse("[year][month][day]T[hour][minute][second]Z").expect("valid format"))
        .expect("formatting")
}

fn format_oss_date_short(datetime: OffsetDateTime) -> String {
    datetime.format(&time::format_description::parse("[year][month][day]").expect("valid format")).expect("formatting")
}

fn additional_headers_v4(headers: &HashMap<String, String>) -> Vec<String> {
    let mut list = Vec::new();
    for key in headers.keys() {
        let lower = key.to_ascii_lowercase();
        if lower == "content-type" || lower == "content-md5" || lower.starts_with("x-oss-") || lower == "authorization"
        {
            continue;
        }
        list.push(lower);
    }
    list.sort();
    list
}

fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

impl OssRequest {
    fn canonical_query_v4(&self) -> String {
        let mut items: Vec<(String, String)> =
            self.queries.iter().map(|(k, v)| (url_encode(k), url_encode(v))).collect();
        items.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
        let mut out = String::new();
        for (key, value) in items {
            if !out.is_empty() {
                out.push('&');
            }
            out.push_str(&key);
            if !value.is_empty() {
                out.push('=');
                out.push_str(&value);
            }
        }
        out
    }

    fn canonical_uri_v4(&self) -> String {
        let mut path = String::from("/");
        if let Some(bucket) = self.oss.bucket.as_deref() {
            if !bucket.is_empty() {
                path.push_str(bucket);
                path.push('/');
            }
        }
        if let Some(object) = self.oss.object.as_deref() {
            if !object.is_empty() {
                path.push_str(&url_encode_path(object));
            }
        }
        if path.is_empty() { "/".to_string() } else { path }
    }

    fn canonical_headers_v4(&self, additional_headers: &[String]) -> String {
        let mut pairs: Vec<(String, String)> = Vec::new();
        for (key, value) in self.headers.iter() {
            let lower = key.to_ascii_lowercase();
            let include = lower == "content-type"
                || lower == "content-md5"
                || lower.starts_with("x-oss-")
                || additional_headers.contains(&lower);
            if include {
                pairs.push((lower, value.trim().to_string()));
            }
        }
        pairs.sort_by(|a, b| a.0.cmp(&b.0));
        let mut out = String::new();
        for (key, value) in pairs {
            out.push_str(&key);
            out.push(':');
            out.push_str(&value);
            out.push('\n');
        }
        out
    }

    fn canonical_request_v4(&self, additional_headers: &[String]) -> String {
        let hashed_payload = self.headers.get("x-oss-content-sha256").map(|v| v.as_str()).unwrap_or("UNSIGNED-PAYLOAD");
        format!(
            "{}\n{}\n{}\n{}\n{}\n{}",
            self.method,
            self.canonical_uri_v4(),
            self.canonical_query_v4(),
            self.canonical_headers_v4(additional_headers),
            additional_headers.join(";"),
            hashed_payload
        )
    }

    fn signature_v4(&self, canonical_request: &str, date: &str, date_short: &str, region: &str) -> String {
        let hashed_request = hex_encode(digest::digest(&digest::SHA256, canonical_request.as_bytes()).as_ref());
        let scope = format!("{}/{}/oss/aliyun_v4_request", date_short, region);
        let string_to_sign = format!("OSS4-HMAC-SHA256\n{}\n{}\n{}", date, scope, hashed_request);

        let key = format!("aliyun_v4{}", self.oss.ak_secret);
        let k_date = hmac::sign(&hmac::Key::new(hmac::HMAC_SHA256, key.as_bytes()), date_short.as_bytes());
        let k_region = hmac::sign(&hmac::Key::new(hmac::HMAC_SHA256, k_date.as_ref()), region.as_bytes());
        let k_service = hmac::sign(&hmac::Key::new(hmac::HMAC_SHA256, k_region.as_ref()), b"oss");
        let k_sign = hmac::sign(&hmac::Key::new(hmac::HMAC_SHA256, k_service.as_ref()), b"aliyun_v4_request");

        hex_encode(hmac::sign(&hmac::Key::new(hmac::HMAC_SHA256, k_sign.as_ref()), string_to_sign.as_bytes()).as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::OffsetDateTime;

    #[test]
    fn test_uri_builder() {
        let mut oss = Oss::new("id", "secret", "cn-hangzhou");
        oss.set_bucket("bucket");
        oss.set_object("file.txt");
        let req = OssRequest::new(oss, Method::GET);
        assert_eq!(req.uri(), "https://bucket.oss-cn-hangzhou.aliyuncs.com/file.txt");
    }

    #[test]
    fn test_query_sign_inserts_signature() {
        let oss = Oss::new("id", "secret", "cn-hangzhou");
        let mut req = OssRequest::new(oss, Method::GET);
        let expires = OffsetDateTime::from_unix_timestamp(0).unwrap();
        req.query_sign(expires);
        let uri = req.uri();
        assert!(uri.contains("x-oss-signature="));
        assert!(uri.contains("x-oss-credential=id%2F"));
        assert!(uri.contains("x-oss-expires="));
    }

    #[test]
    fn test_security_token_header_injected() {
        let mut oss = Oss::new("id", "secret", "cn-hangzhou");
        oss.set_security_token("token");
        let mut req = OssRequest::new(oss, Method::GET);
        req.apply_security_token();
        assert_eq!(req.headers.get("x-oss-security-token").map(|s| s.as_str()), Some("token"));
    }
}
