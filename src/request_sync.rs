use crate::{
    Error,
    common::{url_encode, url_encode_path},
};
use http::{Method, header};
use ring::{digest, hmac};
use std::collections::HashMap;
use time::OffsetDateTime;
use ureq::{self, AsSendBody, Body};

pub(crate) use crate::oss::Oss;

/// Builder for synchronous OSS requests.
///
/// OSS 同步请求构建器。
#[derive(Debug)]
pub(crate) struct OssRequest {
    pub(crate) oss: Oss,
    method: Method,
    headers: HashMap<String, String>,
    queries: HashMap<String, String>,
    body: Vec<u8>,
}

impl OssRequest {
    /// Create a new request builder.
    ///
    /// 创建请求构建器。
    pub fn new(oss: Oss, method: Method) -> Self {
        OssRequest {
            oss,
            method,
            headers: HashMap::new(),
            queries: HashMap::new(),
            body: Vec::new(),
        }
    }

    /// Override the endpoint used for the request.
    ///
    /// 覆盖本次请求使用的 Endpoint。
    pub fn set_endpoint(&mut self, endpoint: impl Into<String>) -> &mut Self {
        self.oss.endpoint = std::borrow::Cow::Owned(endpoint.into());
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

    /// Set request body.
    ///
    /// 设置请求体。
    pub fn set_body(&mut self, body: Vec<u8>) -> &mut Self {
        self.body = body;
        self
    }
    pub fn uri(&self) -> String {
        let scheme = if self.oss.enable_https {
            "https"
        } else {
            "http"
        };
        let domain = self
            .oss
            .custom_domain
            .as_deref()
            .map_or_else(|| format!("{}", self.oss.endpoint), |v| v.to_string());
        let mut path = String::new();
        if self.oss.custom_domain.is_none() {
            if let Some(bucket) = self.oss.bucket.as_deref() {
                path.push_str(&format!("{}.", bucket));
            }
            path.push_str(&domain);
            if let Some(object) = self.oss.object.as_deref() {
                path.push_str(&format!("/{}", url_encode_path(object)));
            }
        } else if let Some(object) = self.oss.object.as_deref() {
            path.push_str(&format!("/{}", url_encode_path(object)));
        }
        let mut uri = format!("{}://{}", scheme, path);
        if !self.queries.is_empty() {
            let query_string = self
                .queries
                .iter()
                .map(|(k, v)| {
                    let key = url_encode(k);
                    if v.is_empty() {
                        key
                    } else {
                        format!("{}={}", key, url_encode(v))
                    }
                })
                .collect::<Vec<_>>()
                .join("&");
            uri.push_str(&format!("?{}", query_string));
        }
        uri
    }
    pub fn url_sign(&mut self, expires: &OffsetDateTime) {
        let now = OffsetDateTime::now_utc();
        let expires = (*expires - now).whole_seconds().max(1);
        let date = format_oss_date(now);
        let date_short = format_oss_date_short(now);
        let region = self.oss.region.to_string();

        self.insert_query("x-oss-signature-version", "OSS4-HMAC-SHA256");
        self.insert_query(
            "x-oss-credential",
            format!(
                "{}/{}/{}/oss/aliyun_v4_request",
                self.oss.ak_id, date_short, region
            ),
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

    pub fn query_sign(&mut self, expires: OffsetDateTime) {
        self.url_sign(&expires);
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

        let credential = format!(
            "{}/{}/{}/oss/aliyun_v4_request",
            self.oss.ak_id, date_short, region
        );
        let authorization = if additional_headers.is_empty() {
            format!(
                "OSS4-HMAC-SHA256 Credential={},Signature={}",
                credential, signature
            )
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

    pub fn send_to_oss(mut self) -> Result<http::Response<Body>, Error> {
        let body = std::mem::take(&mut self.body);
        self.send_to_oss_with_body(body)
    }

    pub fn send_to_oss_with_body<B: AsSendBody>(
        mut self,
        body: B,
    ) -> Result<http::Response<Body>, Error> {
        self.apply_security_token();
        if !self.headers.contains_key("x-oss-content-sha256") {
            self.insert_header("x-oss-content-sha256", "UNSIGNED-PAYLOAD");
        }
        self.header_sign();
        let url = self.uri();

        let mut builder = http::Request::builder()
            .method(self.method.clone())
            .uri(&url);
        for (k, v) in self.headers.iter() {
            builder = builder.header(k, v);
        }
        let request = builder.body(body)?;
        let response = ureq::run(request)?;
        Ok(response)
    }
}

fn format_oss_date(datetime: OffsetDateTime) -> String {
    datetime
        .format(&time::format_description::parse("[year][month][day]T[hour][minute][second]Z").expect("valid format"))
        .expect("formatting")
}

fn format_oss_date_short(datetime: OffsetDateTime) -> String {
    datetime
        .format(&time::format_description::parse("[year][month][day]").expect("valid format"))
        .expect("formatting")
}

fn additional_headers_v4(headers: &HashMap<String, String>) -> Vec<String> {
    let mut list = Vec::new();
    for key in headers.keys() {
        let lower = key.to_ascii_lowercase();
        if lower == "content-type"
            || lower == "content-md5"
            || lower.starts_with("x-oss-")
            || lower == "authorization"
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
        let mut items: Vec<(String, String)> = self
            .queries
            .iter()
            .map(|(k, v)| (url_encode(k), url_encode(v)))
            .collect();
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
        if path.is_empty() {
            "/".to_string()
        } else {
            path
        }
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
        let hashed_payload = self
            .headers
            .get("x-oss-content-sha256")
            .map(|v| v.as_str())
            .unwrap_or("UNSIGNED-PAYLOAD");
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

    fn signature_v4(
        &self,
        canonical_request: &str,
        date: &str,
        date_short: &str,
        region: &str,
    ) -> String {
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

    #[test]
    fn test_security_token_header_injected_sync() {
        let mut oss = Oss::new("id", "secret", "cn-hangzhou");
        oss.set_security_token("token");
        let mut req = OssRequest::new(oss, Method::GET);
        req.apply_security_token();
        assert_eq!(
            req.headers.get("x-oss-security-token").map(|s| s.as_str()),
            Some("token")
        );
    }
}
