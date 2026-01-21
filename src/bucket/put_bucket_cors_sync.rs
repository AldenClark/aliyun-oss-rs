use crate::{
    Error,
    error::normal_error_sync,
    request_sync::{Oss, OssRequest},
};
use http::Method;
use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(rename = "CORSConfiguration")]
struct CorsConfiguration {
    #[serde(rename = "CORSRule", default)]
    rules: Vec<CorsRule>,
}

/// Configure CORS rules for a bucket.
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/zh/oss/developer-reference/putbucketcors) for details.
///
/// 配置 Bucket 的 CORS 规则。
///
/// 详情参见 [阿里云文档](https://help.aliyun.com/zh/oss/developer-reference/putbucketcors)。
pub struct PutBucketCorsSync {
    req: OssRequest,
    cors: CorsConfiguration,
}

impl PutBucketCorsSync {
    pub(super) fn new(oss: Oss) -> Self {
        let mut req = OssRequest::new(oss, Method::PUT);
        req.insert_query("cors", "");
        PutBucketCorsSync {
            req,
            cors: CorsConfiguration::default(),
        }
    }

    /// Replace the complete set of CORS rules.
    ///
    /// 替换全部 CORS 规则。
    pub fn set_rules(mut self, rules: Vec<CorsRule>) -> Self {
        self.cors.rules = rules;
        self
    }

    /// Append an additional CORS rule.
    ///
    /// 追加一条 CORS 规则。
    pub fn add_rule(mut self, rule: CorsRule) -> Self {
        self.cors.rules.push(rule);
        self
    }

    /// Send the request.
    ///
    /// 发送请求。
    pub fn send(mut self) -> Result<(), Error> {
        let body = serde_xml_rs::to_string(&self.cors).map_err(|_| Error::InvalidCharacter)?;
        self.req.set_body(body.into_bytes());
        let response = self.req.send_to_oss()?;
        let status_code = response.status();
        match status_code {
            code if code.is_success() => Ok(()),
            _ => Err(normal_error_sync(response)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
/// CORS rule definition.
///
/// CORS 规则定义。
pub struct CorsRule {
    #[serde(rename = "AllowedOrigin")]
    /// Allowed origins.
    ///
    /// 允许的来源。
    pub allowed_origins: Vec<String>,
    #[serde(rename = "AllowedMethod")]
    /// Allowed HTTP methods.
    ///
    /// 允许的 HTTP 方法。
    pub allowed_methods: Vec<String>,
    #[serde(
        rename = "AllowedHeader",
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    /// Allowed headers.
    ///
    /// 允许的请求头。
    pub allowed_headers: Vec<String>,
    #[serde(
        rename = "ExposeHeader",
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    /// Exposed headers.
    ///
    /// 允许暴露的响应头。
    pub expose_headers: Vec<String>,
    #[serde(rename = "MaxAgeSeconds", skip_serializing_if = "Option::is_none")]
    /// Cache max age seconds.
    ///
    /// 预检缓存秒数。
    pub max_age_seconds: Option<u32>,
}

impl CorsRule {
    /// Create a new CORS rule with required origins and methods.
    ///
    /// 使用必填的来源和方法创建 CORS 规则。
    pub fn new(
        allowed_origins: Vec<impl Into<String>>,
        allowed_methods: Vec<impl Into<String>>,
    ) -> Self {
        CorsRule {
            allowed_origins: allowed_origins.into_iter().map(Into::into).collect(),
            allowed_methods: allowed_methods.into_iter().map(Into::into).collect(),
            allowed_headers: Vec::new(),
            expose_headers: Vec::new(),
            max_age_seconds: None,
        }
    }

    /// Set allowed request headers.
    ///
    /// 设置允许的请求头。
    pub fn set_allowed_headers(mut self, headers: Vec<impl Into<String>>) -> Self {
        self.allowed_headers = headers.into_iter().map(Into::into).collect();
        self
    }

    /// Set exposed response headers.
    ///
    /// 设置可暴露的响应头。
    pub fn set_expose_headers(mut self, headers: Vec<impl Into<String>>) -> Self {
        self.expose_headers = headers.into_iter().map(Into::into).collect();
        self
    }

    /// Set max age seconds for preflight caching.
    ///
    /// 设置预检缓存时间（秒）。
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
