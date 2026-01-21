use crate::{
    Error,
    common::{Acl, DataRedundancyType, StorageClass},
    error::normal_error_sync,
    request_sync::{Oss, OssRequest},
};
use http::Method;

/// Create a bucket with PutBucketSync.
///
/// An Alibaba Cloud account can create up to 100 buckets in a single region.
///
/// See the [Alibaba Cloud documentation](https://help.aliyun.com/document_detail/31959.html) for details.
///
/// 调用 PutBucketSync 创建 Bucket。
///
/// 单个账号在同一地域最多可创建 100 个 Bucket。
///
/// 详情参见 [阿里云文档](https://help.aliyun.com/document_detail/31959.html)。
pub struct PutBucketSync {
    req: OssRequest,
    storage_class: Option<StorageClass>,
    data_redundancy_type: Option<DataRedundancyType>,
}
impl PutBucketSync {
    pub(super) fn new(oss: Oss) -> Self {
        PutBucketSync { req: OssRequest::new(oss, Method::PUT), storage_class: None, data_redundancy_type: None }
    }
    /// Set bucket ACL.
    ///
    /// 设置 Bucket ACL。
    pub fn set_acl(mut self, acl: Acl) -> Self {
        self.req.insert_header("x-oss-acl", acl.to_string());
        self
    }
    /// Specify the resource group ID.
    ///
    /// If provided, the bucket is created in that group; `rg-default-id` means the default group.
    ///
    /// If omitted, the bucket is created in the default resource group.
    ///
    /// 指定资源组 ID。
    ///
    /// 若提供，将创建在该资源组；`rg-default-id` 表示默认资源组。
    ///
    /// 省略则创建在默认资源组。
    pub fn set_group_id(mut self, group_id: impl Into<String>) -> Self {
        self.req.insert_header("x-oss-resource-group-id", group_id.into());
        self
    }
    /// Set bucket storage class.
    ///
    /// `Archive` cannot be combined with `DataRedundancyType::ZRS`.
    ///
    /// 设置 Bucket 存储类型。
    ///
    /// `Archive` 不支持与 `DataRedundancyType::ZRS` 组合。
    pub fn set_storage_class(mut self, storage_class: StorageClass) -> Self {
        self.storage_class = Some(storage_class);
        self.data_redundancy_type = normalize_redundancy(self.storage_class, self.data_redundancy_type);
        let body_str = build_create_bucket_body(self.storage_class, self.data_redundancy_type);
        self.req.set_body(body_str.into_bytes());
        self
    }
    /// Set bucket data redundancy type.
    ///
    /// `DataRedundancyType::ZRS` is not supported for `Archive` storage class.
    ///
    /// 设置 Bucket 冗余类型。
    ///
    /// `Archive` 不支持 `DataRedundancyType::ZRS`。
    pub fn set_redundancy_type(mut self, redundancy_type: DataRedundancyType) -> Self {
        self.data_redundancy_type = Some(redundancy_type);
        self.data_redundancy_type = normalize_redundancy(self.storage_class, self.data_redundancy_type);
        let body_str = build_create_bucket_body(self.storage_class, self.data_redundancy_type);
        self.req.set_body(body_str.into_bytes());
        self
    }
    /// Send the request.
    ///
    /// 发送请求。
    pub fn send(self) -> Result<(), Error> {
        // Build the HTTP request
        let response = self.req.send_to_oss()?;
        // Parse the response
        let status_code = response.status();
        match status_code {
            code if code.is_success() => Ok(()),
            _ => Err(normal_error_sync(response)),
        }
    }
}

fn normalize_redundancy(
    storage_class: Option<StorageClass>,
    redundancy: Option<DataRedundancyType>,
) -> Option<DataRedundancyType> {
    if storage_class == Some(StorageClass::Archive) && redundancy == Some(DataRedundancyType::ZRS) {
        None
    } else {
        redundancy
    }
}

fn build_create_bucket_body(storage_class: Option<StorageClass>, redundancy: Option<DataRedundancyType>) -> String {
    let mut body = String::with_capacity(192);
    body.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
    body.push_str("<CreateBucketConfiguration>");
    if let Some(value) = storage_class {
        body.push_str("<StorageClass>");
        body.push_str(storage_class_value(value));
        body.push_str("</StorageClass>");
    }
    if let Some(value) = redundancy {
        body.push_str("<DataRedundancyType>");
        body.push_str(redundancy_value(value));
        body.push_str("</DataRedundancyType>");
    }
    body.push_str("</CreateBucketConfiguration>");
    body
}

fn storage_class_value(value: StorageClass) -> &'static str {
    match value {
        StorageClass::Standard => "Standard",
        StorageClass::IA => "IA",
        StorageClass::Archive => "Archive",
        StorageClass::ColdArchive => "ColdArchive",
        StorageClass::DeepColdArchive => "DeepColdArchive",
    }
}

fn redundancy_value(value: DataRedundancyType) -> &'static str {
    match value {
        DataRedundancyType::LRS => "LRS",
        DataRedundancyType::ZRS => "ZRS",
    }
}
