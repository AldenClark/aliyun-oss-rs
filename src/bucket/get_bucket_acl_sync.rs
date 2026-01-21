use crate::{
    common::{Acl, BucketAcl, Owner},
    error::{Error, normal_error_sync},
    request_sync::{Oss, OssRequest},
};
use bytes::Bytes;
use http::Method;
use serde_derive::Deserialize;
use std::io::Read;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct AccessControlPolicy {
    owner: Owner,
    access_control_list: AccessControlList,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct AccessControlList {
    grant: Acl,
}

/// Retrieve bucket ACL (sync).
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/document_detail/31964.html) for details.
///
/// 获取 Bucket ACL（同步）。
///
/// 详情参见 [阿里云文档](https://help.aliyun.com/document_detail/31964.html)。
pub struct GetBucketAclSync {
    req: OssRequest,
}
impl GetBucketAclSync {
    pub(crate) fn new(oss: Oss) -> Self {
        let mut req = OssRequest::new(oss, Method::GET);
        req.insert_query("acl", "");
        GetBucketAclSync { req }
    }
    /// Send the request and return bucket ACL info.
    ///
    /// 发送请求并返回 Bucket ACL 信息。
    pub fn send(self) -> Result<BucketAcl, Error> {
        let response = self.req.send_to_oss()?;
        let status = response.status();
        if status.is_success() {
            let mut reader = response.into_body().into_reader();
            let mut buf = Vec::new();
            reader.read_to_end(&mut buf)?;
            let result: AccessControlPolicy = serde_xml_rs::from_reader(&*buf)
                .map_err(|_| Error::OssInvalidResponse(Some(Bytes::from(buf))))?;
            Ok(BucketAcl {
                owner: result.owner,
                acl: result.access_control_list.grant,
            })
        } else {
            Err(normal_error_sync(response))
        }
    }
}
