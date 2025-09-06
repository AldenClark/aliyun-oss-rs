use std::borrow::Cow;

/// Shared OSS configuration used by async and sync clients.
#[derive(Debug, Clone)]
pub(crate) struct Oss {
    pub ak_id: Cow<'static, str>,
    pub ak_secret: Cow<'static, str>,
    pub security_token: Option<Cow<'static, str>>,
    pub endpoint: Cow<'static, str>,
    pub custom_domain: Option<Cow<'static, str>>,
    pub bucket: Option<Cow<'static, str>>,
    pub object: Option<Cow<'static, str>>,
    pub enable_https: bool,
}

impl Oss {
    pub fn new(ak_id: &str, ak_secret: &str) -> Self {
        Oss {
            ak_id: ak_id.to_owned().into(),
            ak_secret: ak_secret.to_owned().into(),
            security_token: None,
            endpoint: "oss.aliyuncs.com".to_owned().into(),
            custom_domain: None,
            bucket: None,
            object: None,
            enable_https: true,
        }
    }

    pub fn set_bucket(&mut self, bucket: impl ToString) {
        self.bucket = Some(bucket.to_string().into());
    }

    pub fn set_object(&mut self, object: impl ToString) {
        self.object = Some(object.to_string().into());
    }

    pub fn set_endpoint(&mut self, endpoint: impl ToString) {
        self.endpoint = endpoint.to_string().into();
    }

    pub fn set_https(&mut self, enable: bool) {
        self.enable_https = enable;
    }

    pub fn set_custom_domain(&mut self, domain: impl ToString) {
        self.custom_domain = Some(domain.to_string().into());
    }

    pub fn set_security_token(&mut self, token: impl ToString) {
        self.security_token = Some(token.to_string().into());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_and_setters() {
        let mut oss = Oss::new("id", "secret");
        assert_eq!(oss.ak_id, "id");
        assert!(oss.enable_https);
        oss.set_bucket("bucket");
        oss.set_object("object");
        oss.set_endpoint("endpoint");
        oss.set_https(false);
        oss.set_custom_domain("example.com");
        oss.set_security_token("token");
        assert_eq!(oss.bucket.as_deref(), Some("bucket"));
        assert_eq!(oss.object.as_deref(), Some("object"));
        assert_eq!(oss.endpoint.as_ref(), "endpoint");
        assert!(!oss.enable_https);
        assert_eq!(oss.custom_domain.as_deref(), Some("example.com"));
        assert_eq!(oss.security_token.as_deref(), Some("token"));
    }
}
