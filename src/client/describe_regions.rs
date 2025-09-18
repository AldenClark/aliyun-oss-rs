use crate::common::body_to_bytes;
use crate::{
    Error,
    error::normal_error,
    request::{Oss, OssRequest},
};
use http::Method;
use serde_derive::Deserialize;

// Returned content
/// Basic region information
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RegionInfo {
    /// Region ID
    pub region: String,
    /// Acceleration endpoint for the region
    pub accelerate_endpoint: String,
    /// Internal endpoint for the region
    pub internal_endpoint: String,
    /// Public endpoint for the region
    pub internet_endpoint: String,
}

#[doc(hidden)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct RegionInfoList {
    pub region_info: Vec<RegionInfo>,
}

/// Query the endpoint information of regions
///
/// You can call `set_regions` to query specific regions; by default all are queried. See the [Alibaba Cloud documentation](https://help.aliyun.com/document_detail/345596.html) for details
///
/// ```ignore
/// let client = OssClient::new("AccessKey ID","AccessKey Secret","oss-cn-beijing.aliyuncs.com");
/// let regions = client.describe_regions()
///                     .set_regions("oss-cn-hangzhou")
///                     .send().await;
/// println!("{:#?}", regions);
/// ```
///
pub struct DescribeRegions {
    req: OssRequest,
}

impl DescribeRegions {
    pub(super) fn new(oss: Oss) -> Self {
        let mut req = OssRequest::new(oss, Method::GET);
        req.insert_query("regions", "");
        DescribeRegions { req }
    }

    /// Specify a single region to query; this requires a Region ID such as oss-cn-hangzhou
    pub fn set_regions(mut self, regions: impl ToString) -> Self {
        self.req.insert_query("regions", regions);
        self
    }

    /// Specify which endpoint to use for the request
    ///
    /// Defaults to oss.aliyuncs.com. If your network cannot access it, set an accessible endpoint
    pub fn set_endpoint(mut self, endpoint: impl ToString) -> Self {
        self.req.set_endpoint(endpoint);
        self
    }

    /// Send the request
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
