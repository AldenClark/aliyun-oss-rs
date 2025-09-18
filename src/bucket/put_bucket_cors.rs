use crate::{
    Error,
    error::normal_error,
    request::{Oss, OssRequest},
};
use bytes::Bytes;
use http::Method;
use http_body_util::Full;
use serde_derive::{Deserialize, Serialize};

use super::CorsConfiguration;

/// Configure CORS rules for a bucket
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/zh/oss/developer-reference/putbucketcors) for details
pub struct PutBucketCors {
    req: OssRequest,
    cors: CorsConfiguration,
}

impl PutBucketCors {
    pub(super) fn new(oss: Oss) -> Self {
        let mut req = OssRequest::new(oss, Method::PUT);
        req.insert_query("cors", "");
        PutBucketCors {
            req,
            cors: CorsConfiguration::default(),
        }
    }

    /// Replace the complete set of CORS rules
    pub fn set_rules(mut self, rules: Vec<CorsRule>) -> Self {
        self.cors.rules = rules;
        self
    }

    /// Append an additional rule to the configuration
    pub fn add_rule(mut self, rule: CorsRule) -> Self {
        self.cors.rules.push(rule);
        self
    }

    /// Send the request
    pub async fn send(mut self) -> Result<(), Error> {
        let body = serde_xml_rs::to_string(&self.cors).map_err(|_| Error::InvalidCharacter)?;
        self.req.set_body(Full::new(Bytes::from(body)));
        let response = self.req.send_to_oss()?.await?;
        let status_code = response.status();
        match status_code {
            code if code.is_success() => Ok(()),
            _ => Err(normal_error(response).await),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CorsRule {
    #[serde(rename = "AllowedOrigin")]
    pub allowed_origins: Vec<String>,
    #[serde(rename = "AllowedMethod")]
    pub allowed_methods: Vec<String>,
    #[serde(
        rename = "AllowedHeader",
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    pub allowed_headers: Vec<String>,
    #[serde(
        rename = "ExposeHeader",
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    pub expose_headers: Vec<String>,
    #[serde(rename = "MaxAgeSeconds", skip_serializing_if = "Option::is_none")]
    pub max_age_seconds: Option<u32>,
}

impl CorsRule {
    pub fn new(allowed_origins: Vec<impl ToString>, allowed_methods: Vec<impl ToString>) -> Self {
        CorsRule {
            allowed_origins: allowed_origins.into_iter().map(|s| s.to_string()).collect(),
            allowed_methods: allowed_methods.into_iter().map(|s| s.to_string()).collect(),
            allowed_headers: Vec::new(),
            expose_headers: Vec::new(),
            max_age_seconds: None,
        }
    }

    pub fn set_allowed_headers(mut self, headers: Vec<impl ToString>) -> Self {
        self.allowed_headers = headers.into_iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn set_expose_headers(mut self, headers: Vec<impl ToString>) -> Self {
        self.expose_headers = headers.into_iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn set_max_age_seconds(mut self, seconds: u32) -> Self {
        self.max_age_seconds = Some(seconds);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cors_rule_serialization() {
        let rule = CorsRule::new(vec!["*"], vec!["GET"]).set_allowed_headers(vec!["Authorization"]);
        let cors = CorsConfiguration { rules: vec![rule] };
        let xml = serde_xml_rs::to_string(&cors).unwrap();
        assert!(xml.contains("<CORSConfiguration>"));
        assert!(xml.contains("<AllowedOrigin>*</AllowedOrigin>"));
        assert!(xml.contains("<AllowedMethod>GET</AllowedMethod>"));
        assert!(xml.contains("<AllowedHeader>Authorization</AllowedHeader>"));
    }
}
