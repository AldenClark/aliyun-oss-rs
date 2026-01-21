use crate::{
    common::{CacheControl, ContentDisposition},
    request::{Oss, OssRequest},
};
use http::Method;
use std::net::IpAddr;
use time::OffsetDateTime;

/// Build a pre-signed URL for an object.
///
/// Private objects can be downloaded directly via a signed URL.
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/document_detail/31952.html) for details.
///
/// 生成对象的预签名 URL。
///
/// 私有对象可通过签名 URL 直接下载。
///
/// 详情见 [阿里云文档](https://help.aliyun.com/document_detail/31952.html)。
pub struct GetObjectUrl {
    req: OssRequest,
}
impl GetObjectUrl {
    pub(super) fn new(oss: Oss) -> Self {
        GetObjectUrl {
            req: OssRequest::new(oss, Method::GET),
        }
    }
    /// Restrict access by source IP.
    ///
    /// To allow a single IP, set `subnet_mask` to 32.
    ///
    /// 按来源 IP 限制访问。
    ///
    /// 若仅允许单个 IP，将 `subnet_mask` 设为 32。
    pub fn set_source_ip(mut self, source_ip: IpAddr, subnet_mask: u8) -> Self {
        self.req
            .insert_query("x-oss-ac-source-ip", source_ip.to_string());
        self.req
            .insert_query("x-oss-ac-subnet-mask", subnet_mask.to_string());
        self
    }
    /// Restrict access by VPC ID.
    ///
    /// 按 VPC ID 限制访问。
    pub fn set_vpc_id(mut self, vpc_id: impl Into<String>) -> Self {
        self.req.insert_query("x-oss-ac-vpc-id", vpc_id.into());
        self
    }
    /// Allow request forwarding (disabled by default).
    ///
    /// 允许请求转发（默认关闭）。
    pub fn forward_allow(mut self) -> Self {
        self.req.insert_query("x-oss-ac-forward-allow", "true");
        self
    }
    /// Override the response `Content-Type`.
    ///
    /// 覆盖响应的 `Content-Type`。
    pub fn set_response_mime(
        mut self,
        mime: impl Into<String>,
        charset: Option<impl Into<String>>,
    ) -> Self {
        let mut mime_str = mime.into();
        if let Some(charset) = charset {
            mime_str.push_str(";charset=");
            mime_str.push_str(&charset.into());
        }
        self.req.insert_query("response-content-type", mime_str);
        self
    }
    /// Override the response `Cache-Control`.
    ///
    /// 覆盖响应的 `Cache-Control`。
    pub fn set_response_cache_control(mut self, cache_control: CacheControl) -> Self {
        self.req.insert_query(
            "response-cache-control",
            cache_control.to_string(),
        );
        self
    }
    /// Override the response `Content-Disposition`.
    ///
    /// 覆盖响应的 `Content-Disposition`。
    pub fn set_response_content_disposition(
        mut self,
        content_disposition: ContentDisposition,
    ) -> Self {
        self.req.insert_query(
            "response-content-disposition",
            content_disposition.to_string(),
        );
        self
    }
    /// Bind a custom domain and choose whether to use HTTPS.
    ///
    /// 绑定自定义域名并设置是否使用 HTTPS。
    pub fn set_custom_domain(mut self, custom_domain: impl Into<String>, enable_https: bool) -> Self {
        self.req.set_endpoint(custom_domain.into());
        self.req.set_https(enable_https);
        self
    }
    /// Generate the signed URL with the given expiration time.
    ///
    /// 使用给定的过期时间生成签名 URL。
    pub fn url(mut self, expires: OffsetDateTime) -> String {
        self.req.query_sign(expires);
        self.req.uri()
    }
}
