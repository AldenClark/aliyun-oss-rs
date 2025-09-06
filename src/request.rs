use crate::{
    Error,
    common::{EXCLUDED_VALUES, format_gmt, url_encode},
};
use base64::{Engine, engine::general_purpose};
use bytes::Bytes;
use http::{Method, header};
use http_body::Body as HttpBody;
use http_body_util::{BodyExt, Empty, combinators::BoxBody};
use hyper::Request;
use hyper_tls::HttpsConnector;
use hyper_util::client::legacy::{Client, ResponseFuture};
use hyper_util::rt::TokioExecutor;
use ring::hmac;
use std::collections::{BTreeMap, HashMap};
use std::error::Error as StdError;
use time::OffsetDateTime;

pub(crate) use crate::oss::Oss;

/// Builder for requests sent to OSS.
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
    pub fn new(oss: Oss, method: Method) -> Self {
        OssRequest {
            oss,
            method,
            headers: HashMap::with_capacity(10),
            queries: HashMap::with_capacity(10),
            body: Empty::<Bytes>::new()
                .map_err(|never| -> Box<dyn StdError + Send + Sync> { match never {} })
                .boxed(),
        }
    }

    /// Override the endpoint used for the request.
    pub fn set_endpoint(&mut self, endpoint: impl ToString) -> &mut Self {
        self.oss.endpoint = endpoint.to_string().into();
        self
    }

    /// Enable or disable HTTPS.
    pub fn set_https(&mut self, https: bool) -> &mut Self {
        self.oss.enable_https = https;
        self
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

    /// Set the request body.
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
        let protocol = if self.oss.enable_https {
            "https://"
        } else {
            "http://"
        };
        // host
        let host = if let Some(custom_domain) = self.oss.custom_domain.clone() {
            custom_domain.to_string()
        } else {
            format!(
                "{}{}",
                self.oss
                    .bucket
                    .clone()
                    .map(|v| format!("{}.", v))
                    .unwrap_or_else(|| String::new()),
                self.oss.endpoint
            )
        };
        // query string
        let query = self
            .queries
            .iter()
            .map(|(key, value)| {
                let value = value.to_string();
                if value.is_empty() {
                    key.to_string()
                } else {
                    format!("{}={}", key, url_encode(&value))
                }
            })
            .collect::<Vec<_>>()
            .join("&");
        let query_str = if query.is_empty() {
            String::new()
        } else {
            format!("?{}", query)
        };
        // build final url
        format!(
            "{}{}/{}{}",
            protocol,
            host,
            url_encode(
                &self
                    .oss
                    .object
                    .clone()
                    .unwrap_or_else(|| String::new().into())
            ),
            query_str
        )
    }
    pub fn query_sign(&mut self, expires: OffsetDateTime) {
        // extract header data
        let mut content_type = String::new();
        let mut content_md5 = String::new();
        let mut canonicalized_ossheaders = BTreeMap::new();
        self.headers.iter().for_each(|(key, value)| {
            if key.starts_with("x-oss-") {
                canonicalized_ossheaders.insert(key, value);
            }
            if key.starts_with(&header::CONTENT_TYPE.to_string()) {
                content_type = value.to_string();
            }
            if key == "Content-MD5" {
                content_md5 = value.to_string();
            }
        });
        // build canonicalized oss headers
        let mut canonicalized_ossheaders = canonicalized_ossheaders
            .into_iter()
            .map(|(key, value)| format!("{}:{}", key, value))
            .collect::<Vec<String>>()
            .join("\n");
        if !canonicalized_ossheaders.is_empty() {
            canonicalized_ossheaders.push_str("\n");
        }
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
        // canonicalized resource
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
        // build string to sign
        let unsign_str = format!(
            "{}\n{}\n{}\n{}\n{}{}",
            self.method,
            content_md5,
            content_type,
            expires.unix_timestamp(),
            canonicalized_ossheaders,
            canonicalized_resource
        );
        // calculate signature
        let key_str = hmac::Key::new(
            hmac::HMAC_SHA1_FOR_LEGACY_USE_ONLY,
            self.oss.ak_secret.as_bytes(),
        );
        let sign_str =
            general_purpose::STANDARD.encode(hmac::sign(&key_str, unsign_str.as_bytes()));
        self.insert_header(header::DATE, format_gmt(OffsetDateTime::now_utc()));
        self.insert_query("Signature", sign_str);
        self.insert_query("OSSAccessKeyId", &self.oss.ak_id.clone());
        self.insert_query("Expires", expires.unix_timestamp());
    }
    pub fn header_sign(&mut self) {
        // extract header data
        let mut content_type = String::new();
        let mut content_md5 = String::new();
        let mut canonicalized_ossheaders = BTreeMap::new();
        self.headers.iter().for_each(|(key, value)| {
            if key.starts_with("x-oss-") {
                canonicalized_ossheaders.insert(key, value);
            }
            if key.starts_with(&header::CONTENT_TYPE.to_string()) {
                content_type = value.to_string();
            }
            if key == "Content-MD5" {
                content_md5 = value.to_string();
            }
        });
        // build canonicalized oss headers
        let mut canonicalized_ossheaders = canonicalized_ossheaders
            .into_iter()
            .map(|(key, value)| format!("{}:{}", key, value))
            .collect::<Vec<String>>()
            .join("\n");
        if !canonicalized_ossheaders.is_empty() {
            canonicalized_ossheaders.push_str("\n");
        }
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
        // canonicalized resource
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
        // build string to sign
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
        // calculate signature
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

    pub fn send_to_oss(mut self) -> Result<ResponseFuture, Error> {
        // insert temporary security token if provided
        if let Some(security_token) = self.oss.security_token.clone() {
            self.insert_header("x-oss-security-token", security_token);
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
            let https = HttpsConnector::new();
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

#[cfg(test)]
mod tests {
    use super::*;
    use time::OffsetDateTime;

    #[test]
    fn test_uri_builder() {
        let mut oss = Oss::new("id", "secret");
        oss.set_bucket("bucket");
        oss.set_object("file.txt");
        let req = OssRequest::new(oss, Method::GET);
        assert_eq!(req.uri(), "https://bucket.oss.aliyuncs.com/file%2Etxt");
    }

    #[test]
    fn test_query_sign_inserts_signature() {
        let oss = Oss::new("id", "secret");
        let mut req = OssRequest::new(oss, Method::GET);
        let expires = OffsetDateTime::from_unix_timestamp(0).unwrap();
        req.query_sign(expires);
        let uri = req.uri();
        assert!(uri.contains("Signature="));
        assert!(uri.contains("OSSAccessKeyId=id"));
        assert!(uri.contains("Expires=0"));
    }
}
