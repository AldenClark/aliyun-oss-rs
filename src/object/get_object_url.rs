use crate::{
    common::{CacheControl, ContentDisposition},
    request::{Oss, OssRequest},
};
use http::Method;
use std::net::IpAddr;
use time::OffsetDateTime;

/// Get the object's URL
///
/// Private objects can obtain a signed URL to download directly
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/document_detail/31952.html) for details
pub struct GetObjectUrl {
    req: OssRequest,
}
impl GetObjectUrl {
    pub(super) fn new(oss: Oss) -> Self {
        GetObjectUrl {
            req: OssRequest::new(oss, Method::GET),
        }
    }
    /// Set IP information
    ///
    /// To allow a single IP, set `subnet_mask` to 32
    ///
    pub fn set_source_ip(mut self, source_ip: IpAddr, subnet_mask: u8) -> Self {
        self.req.insert_query("x-oss-ac-source-ip", source_ip);
        self.req
            .insert_query("x-oss-ac-subnet-mask", subnet_mask.to_string());
        self
    }
    /// Set VPC information
    ///
    pub fn set_vpc_id(mut self, vpc_id: impl ToString) -> Self {
        self.req.insert_query("x-oss-ac-vpc-id", vpc_id);
        self
    }
    /// Allow request forwarding
    ///
    /// Disabled by default
    ///
    pub fn forward_allow(mut self) -> Self {
        self.req.insert_query("x-oss-ac-forward-allow", "true");
        self
    }
    /// Set the response content-type
    ///
    pub fn set_response_mime(
        mut self,
        mime: impl ToString,
        charset: Option<impl ToString>,
    ) -> Self {
        let mut mime_str = mime.to_string();
        if let Some(charset) = charset {
            mime_str.push_str(";charset=");
            mime_str.push_str(&charset.to_string());
        }
        self.req.insert_query("response-content-type", mime_str);
        self
    }
    /// Set the response cache-control
    ///
    pub fn set_response_cache_control(mut self, cache_control: CacheControl) -> Self {
        self.req
            .insert_query("response-cache-control", cache_control);
        self
    }
    /// Set the response content-disposition
    ///
    pub fn set_response_content_disposition(
        mut self,
        content_disposition: ContentDisposition,
    ) -> Self {
        self.req
            .insert_query("response-content-disposition", content_disposition);
        self
    }
    /// Set a custom domain
    ///
    pub fn set_custom_domain(mut self, custom_domain: impl ToString, enable_https: bool) -> Self {
        self.req.set_endpoint(custom_domain);
        self.req.set_https(enable_https);
        self
    }
    /// Generate the URL
    ///
    pub fn url(mut self, expires: OffsetDateTime) -> String {
        self.req.query_sign(expires);
        self.req.uri()
    }
}
