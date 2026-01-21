use crate::common::body_to_bytes;
use crate::{
    Error,
    error::normal_error,
    request::{Oss, OssRequest},
};
use http::Method;
use serde_derive::Deserialize;

// Returned content
/// Basic region information.
///
/// 地域基础信息。
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RegionInfo {
    /// Region ID.
    ///
    /// 地域 ID。
    pub region: String,
    /// Acceleration endpoint for the region.
    ///
    /// 地域传输加速 Endpoint。
    pub accelerate_endpoint: String,
    /// Internal endpoint for the region.
    ///
    /// 地域内网 Endpoint。
    pub internal_endpoint: String,
    /// Public endpoint for the region.
    ///
    /// 地域公网 Endpoint。
    pub internet_endpoint: String,
}

#[doc(hidden)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct RegionInfoList {
    pub region_info: Vec<RegionInfo>,
}

/// Query the endpoint information of regions.
///
/// Use `set_regions` to query specific regions; by default all regions are returned.
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/document_detail/345596.html) for details.
///
/// ```ignore
/// let client = OssClient::new("AccessKey ID", "AccessKey Secret", "cn-beijing");
/// let regions = client.describe_regions()
/// .set_regions("oss-cn-hangzhou")
/// .send().await;
/// println!("{:#?}", regions);
/// ```
///
/// 查询地域 Endpoint 信息。
///
/// 可使用 `set_regions` 指定地域；默认返回全部地域。
///
/// 详情见 [阿里云文档](https://help.aliyun.com/document_detail/345596.html)。
///
/// ```ignore
/// let client = OssClient::new("AccessKey ID", "AccessKey Secret", "cn-beijing");
/// let regions = client.describe_regions()
/// .set_regions("oss-cn-hangzhou")
/// .send().await;
/// println!("{:#?}", regions);
/// ```
pub struct DescribeRegions {
    req: OssRequest,
}

impl DescribeRegions {
    pub(super) fn new(oss: Oss) -> Self {
        let mut req = OssRequest::new(oss, Method::GET);
        req.insert_query("regions", "");
        DescribeRegions { req }
    }

    /// Specify a single region to query (e.g., oss-cn-hangzhou).
    ///
    /// 指定要查询的地域（如 oss-cn-hangzhou）。
    pub fn set_regions(mut self, regions: impl Into<String>) -> Self {
        self.req.insert_query("regions", regions.into());
        self
    }

    /// Specify which endpoint to use for the request.
    ///
    /// Defaults to oss.aliyuncs.com. If unreachable, use an accessible endpoint.
    ///
    /// 指定本次请求使用的 Endpoint。
    ///
    /// 默认 oss.aliyuncs.com，不可达时请设置可访问的 Endpoint。
    pub fn set_endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.req.set_endpoint(endpoint.into());
        self
    }

    /// Send the request.
    ///
    /// 发送请求。
    pub async fn send(self) -> Result<Vec<RegionInfo>, Error> {
        // Build the HTTP request
        let response = self.req.send_to_oss()?.await?;
        // Parse the response
        let status_code = response.status();
        match status_code {
            code if code.is_success() => {
                let response_bytes = body_to_bytes(response.into_body())
                    .await
                    .map_err(|_| Error::OssInvalidResponse(None))?;
                let regions: RegionInfoList = serde_xml_rs::from_reader(&*response_bytes)
                    .map_err(|_| Error::OssInvalidResponse(Some(response_bytes)))?;
                Ok(regions.region_info)
            }
            _ => Err(normal_error(response).await),
        }
    }
}
