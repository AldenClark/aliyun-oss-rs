use crate::{
    Error,
    common::body_to_bytes_sync,
    error::normal_error_sync,
    request_sync::{Oss, OssRequest},
};
use http::Method;

use super::TransferAccelerationConfiguration;

/// Retrieve the transfer acceleration configuration of a bucket.
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/zh/oss/developer-reference/getbuckettransferacceleration) for details.
///
/// 获取 Bucket 传输加速配置。
///
/// 详情见 [阿里云文档](https://help.aliyun.com/zh/oss/developer-reference/getbuckettransferacceleration)。
pub struct GetBucketTransferAccelerationSync {
    req: OssRequest,
}

impl GetBucketTransferAccelerationSync {
    pub(super) fn new(oss: Oss) -> Self {
        let mut req = OssRequest::new(oss, Method::GET);
        req.insert_query("transferAcceleration", "");
        GetBucketTransferAccelerationSync { req }
    }

    /// Send the request and return the parsed configuration.
    ///
    /// 发送请求并返回解析后的配置。
    pub fn send(self) -> Result<TransferAccelerationConfiguration, Error> {
        let response = self.req.send_to_oss()?;
        match response.status() {
            code if code.is_success() => {
                let bytes = body_to_bytes_sync(response.into_body())?;
                let config: TransferAccelerationConfiguration =
                    serde_xml_rs::from_reader(bytes.as_ref())
                        .map_err(|_| Error::OssInvalidResponse(Some(bytes)))?;
                Ok(config)
            }
            _ => Err(normal_error_sync(response)),
        }
    }
}
