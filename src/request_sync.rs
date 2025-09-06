use crate::{
    Error,
    common::{EXCLUDED_VALUES, format_gmt, url_encode},
};
use base64::{Engine, engine::general_purpose};
use http::{Method, header};
use ring::hmac;
use std::collections::{BTreeMap, HashMap};
use time::OffsetDateTime;
use ureq::{self, Body};

pub(crate) use crate::oss::Oss;

/// Builder for synchronous OSS requests.
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
    pub fn new(oss: Oss, method: Method) -> Self {
        OssRequest {
            oss,
            method,
            headers: HashMap::new(),
            queries: HashMap::new(),
            body: Vec::new(),
        }
    }

    /// Insert a header key/value pair.
    pub fn insert_header(&mut self, key: impl ToString, value: impl ToString) -> &mut Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }

    /// Insert a query parameter.
    pub fn insert_query(&mut self, key: impl ToString, value: impl ToString) -> &mut Self {
        self.queries.insert(key.to_string(), value.to_string());
        self
    }

    /// Set request body.
    pub fn set_body(&mut self, body: Vec<u8>) -> &mut Self {
        self.body = body;
        self
    }
    fn uri(&self) -> String {
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
                path.push_str(&format!("/{}", url_encode(object)));
            }
        } else if let Some(object) = self.oss.object.as_deref() {
            path.push_str(&format!("/{}", url_encode(object)));
        }
        let mut uri = format!("{}://{}", scheme, path);
        if !self.queries.is_empty() {
            let query_string = self
                .queries
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<_>>()
                .join("&");
            uri.push_str(&format!("?{}", query_string));
        }
        uri
    }
    pub fn url_sign(&mut self, expires: &OffsetDateTime) {
        let mut canonicalized_resource = format!(
            "/{}{}",
            self.oss
                .bucket
                .as_deref()
                .map_or(String::new(), |v| format!("{}/", v)),
            self.oss
                .object
                .as_deref()
                .map_or(String::new(), |v| format!("{}", v))
        );
        // build sub resource
        let sub_resource = self
            .queries
            .iter()
            .filter_map(|(key, value)| {
                if key.starts_with("x-oss-") || EXCLUDED_VALUES.contains(&key.as_str()) {
                    Some((key.to_owned(), value.to_owned()))
                } else {
                    None
                }
            })
            .collect::<BTreeMap<String, String>>()
            .into_iter()
            .map(|(key, value)| {
                if value.is_empty() {
                    key.to_owned()
                } else {
                    format!("{}={}", key, value)
                }
            })
            .collect::<Vec<_>>()
            .join("&");
        if !sub_resource.is_empty() {
            canonicalized_resource.push_str(&format!("?{}", sub_resource));
        }
        // build string to sign
        let unsign_str = format!(
            "{}\n\n\n{}\n{}",
            self.method,
            expires.unix_timestamp(),
            canonicalized_resource
        );
        // calculate signature
        let key_str = hmac::Key::new(
            hmac::HMAC_SHA1_FOR_LEGACY_USE_ONLY,
            self.oss.ak_secret.as_bytes(),
        );
        let sign_str =
            general_purpose::STANDARD.encode(hmac::sign(&key_str, unsign_str.as_bytes()));
        self.insert_query("Expires", expires.unix_timestamp());
        self.insert_query("OSSAccessKeyId", &self.oss.ak_id.clone());
        self.insert_query("Signature", sign_str);
    }
    pub fn header_sign(&mut self) {
        let mut content_type = String::new();
        let mut content_md5 = String::new();
        let mut canonicalized_ossheaders = BTreeMap::new();
        self.headers.iter().for_each(|(key, value)| {
            if key.starts_with("x-oss-") {
                canonicalized_ossheaders.insert(key, value);
            };
            if key.starts_with(&header::CONTENT_TYPE.to_string()) {
                content_type = value.to_string();
            };
            if key == "Content-MD5" {
                content_md5 = value.to_string();
            };
        });
        let mut canonicalized_ossheaders = canonicalized_ossheaders
            .into_iter()
            .map(|(key, value)| format!("{}:{}", key, value))
            .collect::<Vec<String>>()
            .join("\n");
        if !canonicalized_ossheaders.is_empty() {
            canonicalized_ossheaders.push_str("\n")
        }
        let sub_resource = self
            .queries
            .iter()
            .filter_map(|(key, value)| {
                if key.starts_with("x-oss-") || EXCLUDED_VALUES.contains(&key.as_str()) {
                    Some((key.to_owned(), value.to_owned()))
                } else {
                    None
                }
            })
            .collect::<BTreeMap<String, String>>()
            .into_iter()
            .map(|(key, value)| {
                if value.is_empty() {
                    key.to_owned()
                } else {
                    format!("{}={}", key, value)
                }
            })
            .collect::<Vec<_>>()
            .join("&");
        let mut canonicalized_resource = format!(
            "/{}{}",
            self.oss
                .bucket
                .as_deref()
                .map_or(String::new(), |v| format!("{}/", v)),
            self.oss
                .object
                .as_deref()
                .map_or(String::new(), |v| format!("{}", v))
        );
        if !sub_resource.is_empty() {
            canonicalized_resource.push_str(&format!("?{}", sub_resource));
        }
        let date = format_gmt(OffsetDateTime::now_utc());
        let unsign_str = format!(
            "{}\n{}\n{}\n{}\n{}{}",
            self.method,
            content_md5,
            content_type,
            date,
            canonicalized_ossheaders,
            canonicalized_resource
        );
        let key_str = hmac::Key::new(
            hmac::HMAC_SHA1_FOR_LEGACY_USE_ONLY,
            self.oss.ak_secret.as_bytes(),
        );
        let sign_str =
            general_purpose::STANDARD.encode(hmac::sign(&key_str, unsign_str.as_bytes()));
        self.insert_header(header::DATE, date);
        self.insert_header(
            header::AUTHORIZATION,
            format!("OSS {}:{}", self.oss.ak_id, sign_str),
        );
    }
    pub fn send_to_oss(mut self) -> Result<http::Response<Body>, Error> {
        if let Some(security_token) = self.oss.security_token.clone() {
            self.insert_header("x-oss-security-token", security_token);
        };
        self.header_sign();
        let url = self.uri();

        let mut builder = http::Request::builder()
            .method(self.method.clone())
            .uri(&url);
        for (k, v) in self.headers.iter() {
            builder = builder.header(k, v);
        }
        let request = builder.body(self.body.clone())?;
        let response = ureq::run(request)?;
        Ok(response)
    }
}
