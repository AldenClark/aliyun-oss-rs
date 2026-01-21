use std::borrow::Cow;

/// Shared OSS configuration used by async and sync clients.
///
/// 异步与同步客户端共享的 OSS 配置。
#[derive(Debug, Clone)]
pub(crate) struct Oss {
    pub ak_id: Cow<'static, str>,
    pub ak_secret: Cow<'static, str>,
    pub security_token: Option<Cow<'static, str>>,
    pub region: Cow<'static, str>,
    pub endpoint: Cow<'static, str>,
    pub custom_domain: Option<Cow<'static, str>>,
    pub bucket: Option<Cow<'static, str>>,
    pub object: Option<Cow<'static, str>>,
    pub enable_https: bool,
}

impl Oss {
    pub fn new(ak_id: impl Into<String>, ak_secret: impl Into<String>, region: impl Into<String>) -> Self {
        let region = region.into();
        let region = region.trim().to_string();
        let endpoint = if region.is_empty() {
            Cow::Borrowed("oss.aliyuncs.com")
        } else {
            Cow::Owned(format!("oss-{}.aliyuncs.com", region))
        };
        Oss {
            ak_id: Cow::Owned(ak_id.into()),
            ak_secret: Cow::Owned(ak_secret.into()),
            security_token: None,
            region: Cow::Owned(region),
            endpoint,
            custom_domain: None,
            bucket: None,
            object: None,
            enable_https: true,
        }
    }

    pub fn set_bucket(&mut self, bucket: impl Into<String>) {
        self.bucket = Some(Cow::Owned(bucket.into()));
    }

    pub fn set_object(&mut self, object: impl Into<String>) {
        self.object = Some(Cow::Owned(object.into()));
    }

    pub fn set_endpoint(&mut self, endpoint: impl Into<String>) {
        self.endpoint = Cow::Owned(endpoint.into());
    }

    pub fn set_https(&mut self, enable: bool) {
        self.enable_https = enable;
    }

    pub fn set_custom_domain(&mut self, domain: impl Into<String>) {
        self.custom_domain = Some(Cow::Owned(domain.into()));
    }

    pub fn set_security_token(&mut self, token: impl Into<String>) {
        self.security_token = Some(Cow::Owned(token.into()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_and_setters() {
        let mut oss = Oss::new("id", "secret", "cn-hangzhou");
        assert_eq!(oss.ak_id, "id");
        assert!(oss.enable_https);
        assert_eq!(oss.region.as_ref(), "cn-hangzhou");
        oss.set_bucket("bucket");
        oss.set_object("object");
        oss.set_endpoint("endpoint");
        oss.set_https(false);
        oss.set_custom_domain("example.com");
        oss.set_security_token("token");
        assert_eq!(oss.bucket.as_deref(), Some("bucket"));
        assert_eq!(oss.object.as_deref(), Some("object"));
        assert_eq!(oss.endpoint.as_ref(), "endpoint");
        assert_eq!(oss.region.as_ref(), "cn-hangzhou");
        assert!(!oss.enable_https);
        assert_eq!(oss.custom_domain.as_deref(), Some("example.com"));
        assert_eq!(oss.security_token.as_deref(), Some("token"));
    }
}
