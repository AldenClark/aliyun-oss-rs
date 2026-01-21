#[cfg(feature = "async")]
use super::{
    AbortUpload, AppendObject, CompleteUpload, CopyObject, CopyToPart, DelObjectTagging, GetObject,
    GetObjectAcl, GetObjectMeta, GetObjectTagging, GetObjectUrl, GetSymlink, HeadObject,
    InitUpload, ListParts, PutObject, PutObjectAcl, PutObjectTagging, PutSymlink, RestoreObject,
    SelectObject, UploadPart, del_object::DelObject,
};
#[cfg(feature = "sync")]
use super::{
    AbortUploadSync, AppendObjectSync, CompleteUploadSync, CopyObjectSync, CopyToPartSync,
    DelObjectSync, DelObjectTaggingSync, GetObjectAclSync, GetObjectMetaSync, GetObjectSync,
    GetObjectTaggingSync, GetObjectUrlSync, GetSymlinkSync, HeadObjectSync, InitUploadSync,
    ListPartsSync, PutObjectAclSync, PutObjectSync, PutObjectTaggingSync, PutSymlinkSync,
    RestoreObjectSync, SelectObjectSync, UploadPartSync,
};
use crate::{common::Acl, oss::Oss};

/// Object handle exposing object-level APIs such as upload, download, and metadata.
///
/// 对象句柄，提供上传、下载、元数据等对象级 API。
#[derive(Debug, Clone)]
pub struct OssObject {
    oss: Oss,
}

impl OssObject {
    pub(crate) fn new(mut oss: Oss, object: impl Into<String>) -> Self {
        oss.set_object(object);
        OssObject { oss }
    }
    /// Attach a temporary security token for STS authentication.
    ///
    /// 设置临时安全令牌用于 STS 鉴权。
    pub fn with_security_token(mut self, token: impl Into<String>) -> Self {
        self.oss.set_security_token(token);
        self
    }
    /// Update the security token in place for reuse.
    ///
    /// 就地更新安全令牌，便于复用。
    pub fn set_security_token(&mut self, token: impl Into<String>) {
        self.oss.set_security_token(token);
    }
    /// Upload an object to OSS.
    ///
    /// 上传对象到 OSS。
#[cfg(feature = "async")]
    pub fn put_object(&self) -> PutObject {
        PutObject::new(self.oss.clone())
    }
    /// Upload an object to OSS (sync).
    ///
    /// 上传对象到 OSS（同步）。
    #[cfg(feature = "sync")]
    pub fn put_object_sync(&self) -> PutObjectSync {
        PutObjectSync::new(self.oss.clone())
    }
    /// Append data to an appendable object.
    ///
    /// 追加数据到可追加对象。
#[cfg(feature = "async")]
    pub fn append_object(&self) -> AppendObject {
        AppendObject::new(self.oss.clone())
    }
    /// Append data to an appendable object (sync).
    ///
    /// 追加数据到可追加对象（同步）。
    #[cfg(feature = "sync")]
    pub fn append_object_sync(&self) -> AppendObjectSync {
        AppendObjectSync::new(self.oss.clone())
    }
    /// Delete the object.
    ///
    /// 删除对象。
#[cfg(feature = "async")]
    pub fn del_object(&self) -> DelObject {
        DelObject::new(self.oss.clone())
    }
    /// Delete the object (sync).
    ///
    /// 删除对象（同步）。
    #[cfg(feature = "sync")]
    pub fn del_object_sync(&self) -> DelObjectSync {
        DelObjectSync::new(self.oss.clone())
    }
    /// Generate a pre-signed URL for the object.
    ///
    /// 生成对象的预签名 URL。
#[cfg(feature = "async")]
    pub fn get_object_url(&self) -> GetObjectUrl {
        GetObjectUrl::new(self.oss.clone())
    }
    /// Generate a pre-signed URL for the object (sync).
    ///
    /// 生成对象的预签名 URL（同步）。
    #[cfg(feature = "sync")]
    pub fn get_object_url_sync(&self) -> GetObjectUrlSync {
        GetObjectUrlSync::new(self.oss.clone())
    }
    /// Retrieve object tags.
    ///
    /// 获取对象标签。
#[cfg(feature = "async")]
    pub fn get_object_tagging(&self) -> GetObjectTagging {
        GetObjectTagging::new(self.oss.clone())
    }
    /// Retrieve object tags (sync).
    ///
    /// 获取对象标签（同步）。
    #[cfg(feature = "sync")]
    pub fn get_object_tagging_sync(&self) -> GetObjectTaggingSync {
        GetObjectTaggingSync::new(self.oss.clone())
    }
    /// Retrieve full object metadata (HEAD).
    ///
    /// 获取对象完整元数据（HEAD）。
#[cfg(feature = "async")]
    pub fn head_object(&self) -> HeadObject {
        HeadObject::new(self.oss.clone())
    }
    /// Retrieve full object metadata (HEAD) (sync).
    ///
    /// 获取对象完整元数据（HEAD）（同步）。
    #[cfg(feature = "sync")]
    pub fn head_object_sync(&self) -> HeadObjectSync {
        HeadObjectSync::new(self.oss.clone())
    }
    /// Retrieve object metadata summary.
    ///
    /// 获取对象元数据摘要。
#[cfg(feature = "async")]
    pub fn get_object_meta(&self) -> GetObjectMeta {
        GetObjectMeta::new(self.oss.clone())
    }
    /// Retrieve object metadata summary (sync).
    ///
    /// 获取对象元数据摘要（同步）。
    #[cfg(feature = "sync")]
    pub fn get_object_meta_sync(&self) -> GetObjectMetaSync {
        GetObjectMetaSync::new(self.oss.clone())
    }
    /// Retrieve object ACL.
    ///
    /// 获取对象 ACL。
#[cfg(feature = "async")]
    pub fn get_object_acl(&self) -> GetObjectAcl {
        GetObjectAcl::new(self.oss.clone())
    }
    /// Retrieve object ACL (sync).
    ///
    /// 获取对象 ACL（同步）。
    #[cfg(feature = "sync")]
    pub fn get_object_acl_sync(&self) -> GetObjectAclSync {
        GetObjectAclSync::new(self.oss.clone())
    }
    /// Download the object content.
    ///
    /// 下载对象内容。
#[cfg(feature = "async")]
    pub fn get_object(&self) -> GetObject {
        GetObject::new(self.oss.clone())
    }
    /// Download the object content (sync).
    ///
    /// 下载对象内容（同步）。
    #[cfg(feature = "sync")]
    pub fn get_object_sync(&self) -> GetObjectSync {
        GetObjectSync::new(self.oss.clone())
    }
    /// Query object content using OSS Select.
    ///
    /// 使用 OSS Select 查询对象内容。
#[cfg(feature = "async")]
    pub fn select_object(&self) -> SelectObject {
        SelectObject::new(self.oss.clone())
    }
    /// Query object content using OSS Select (sync).
    ///
    /// 使用 OSS Select 查询对象内容（同步）。
    #[cfg(feature = "sync")]
    pub fn select_object_sync(&self) -> SelectObjectSync {
        SelectObjectSync::new(self.oss.clone())
    }
    /// Copy the object from another source.
    ///
    /// 从其他来源拷贝对象。
#[cfg(feature = "async")]
    pub fn copy_object(&self, copy_source: impl Into<String>) -> CopyObject {
        CopyObject::new(self.oss.clone(), copy_source)
    }
    /// Copy the object from another source (sync).
    ///
    /// 从其他来源拷贝对象（同步）。
    #[cfg(feature = "sync")]
    pub fn copy_object_sync(&self, copy_source: impl Into<String>) -> CopyObjectSync {
        CopyObjectSync::new(self.oss.clone(), copy_source)
    }
    /// Restore an archived object.
    ///
    /// 解冻归档对象。
#[cfg(feature = "async")]
    pub fn restore_object(&self) -> RestoreObject {
        RestoreObject::new(self.oss.clone())
    }
    /// Restore an archived object (sync).
    ///
    /// 解冻归档对象（同步）。
    #[cfg(feature = "sync")]
    pub fn restore_object_sync(&self) -> RestoreObjectSync {
        RestoreObjectSync::new(self.oss.clone())
    }
    /// Set object ACL.
    ///
    /// 设置对象 ACL。
#[cfg(feature = "async")]
    pub fn put_object_acl(&self, acl: Acl) -> PutObjectAcl {
        PutObjectAcl::new(self.oss.clone(), acl)
    }
    /// Set object ACL (sync).
    ///
    /// 设置对象 ACL（同步）。
    #[cfg(feature = "sync")]
    pub fn put_object_acl_sync(&self, acl: Acl) -> PutObjectAclSync {
        PutObjectAclSync::new(self.oss.clone(), acl)
    }
    /// Set object tags.
    ///
    /// 设置对象标签。
#[cfg(feature = "async")]
    pub fn put_object_tagging(
        &self,
        tags: Vec<(impl Into<String>, impl Into<String>)>,
    ) -> PutObjectTagging {
        PutObjectTagging::new(self.oss.clone(), tags)
    }
    /// Set object tags (sync).
    ///
    /// 设置对象标签（同步）。
    #[cfg(feature = "sync")]
    pub fn put_object_tagging_sync(
        &self,
        tags: Vec<(impl Into<String>, impl Into<String>)>,
    ) -> PutObjectTaggingSync {
        PutObjectTaggingSync::new(self.oss.clone(), tags)
    }
    /// Create a symlink object.
    ///
    /// 创建符号链接对象。
#[cfg(feature = "async")]
    pub fn put_symlink(&self, symlink_target: impl Into<String>) -> PutSymlink {
        PutSymlink::new(self.oss.clone(), symlink_target)
    }
    /// Create a symlink object (sync).
    ///
    /// 创建符号链接对象（同步）。
    #[cfg(feature = "sync")]
    pub fn put_symlink_sync(&self, symlink_target: impl Into<String>) -> PutSymlinkSync {
        PutSymlinkSync::new(self.oss.clone(), symlink_target)
    }
    /// Retrieve the symlink target.
    ///
    /// 获取符号链接目标。
#[cfg(feature = "async")]
    pub fn get_symlink(&self) -> GetSymlink {
        GetSymlink::new(self.oss.clone())
    }
    /// Retrieve the symlink target (sync).
    ///
    /// 获取符号链接目标（同步）。
    #[cfg(feature = "sync")]
    pub fn get_symlink_sync(&self) -> GetSymlinkSync {
        GetSymlinkSync::new(self.oss.clone())
    }
    /// Delete all object tags.
    ///
    /// 删除对象全部标签。
#[cfg(feature = "async")]
    pub fn del_object_tagging(&self) -> DelObjectTagging {
        DelObjectTagging::new(self.oss.clone())
    }
    /// Delete all object tags (sync).
    ///
    /// 删除对象全部标签（同步）。
    #[cfg(feature = "sync")]
    pub fn del_object_tagging_sync(&self) -> DelObjectTaggingSync {
        DelObjectTaggingSync::new(self.oss.clone())
    }
    /// Initiate a multipart upload.
    ///
    /// 初始化分片上传。
#[cfg(feature = "async")]
    pub fn multipart_init_upload(&self) -> InitUpload {
        InitUpload::new(self.oss.clone())
    }
    /// Initiate a multipart upload (sync).
    ///
    /// 初始化分片上传（同步）。
    #[cfg(feature = "sync")]
    pub fn multipart_init_upload_sync(&self) -> InitUploadSync {
        InitUploadSync::new(self.oss.clone())
    }
    /// Upload a multipart part.
    ///
    /// 上传分片。
#[cfg(feature = "async")]
    pub fn multipart_upload_part(
        &self,
        part_number: u32,
        upload_id: impl Into<String>,
    ) -> UploadPart {
        UploadPart::new(self.oss.clone(), part_number, upload_id)
    }
    /// Upload a multipart part (sync).
    ///
    /// 上传分片（同步）。
    #[cfg(feature = "sync")]
    pub fn multipart_upload_part_sync(
        &self,
        part_number: u32,
        upload_id: impl Into<String>,
    ) -> UploadPartSync {
        UploadPartSync::new(self.oss.clone(), part_number, upload_id)
    }
    /// Copy object data into a multipart part.
    ///
    /// 将对象数据拷贝到分片。
#[cfg(feature = "async")]
    pub fn multipart_copy_part(
        &self,
        part_number: u32,
        upload_id: impl Into<String>,
        copy_source: impl Into<String>,
    ) -> CopyToPart {
        CopyToPart::new(self.oss.clone(), part_number, upload_id, copy_source)
    }
    /// Copy object data into a multipart part (sync).
    ///
    /// 将对象数据拷贝到分片（同步）。
    #[cfg(feature = "sync")]
    pub fn multipart_copy_part_sync(
        &self,
        part_number: u32,
        upload_id: impl Into<String>,
        copy_source: impl Into<String>,
    ) -> CopyToPartSync {
        CopyToPartSync::new(self.oss.clone(), part_number, upload_id, copy_source)
    }
    /// Complete multipart upload.
    ///
    /// 完成分片上传。
#[cfg(feature = "async")]
    pub fn multipart_complete_upload(&self, upload_id: impl Into<String>) -> CompleteUpload<'_> {
        CompleteUpload::new(self.oss.clone(), upload_id)
    }
    /// Complete multipart upload (sync).
    ///
    /// 完成分片上传（同步）。
    #[cfg(feature = "sync")]
    pub fn multipart_complete_upload_sync(
        &self,
        upload_id: impl Into<String>,
    ) -> CompleteUploadSync<'_> {
        CompleteUploadSync::new(self.oss.clone(), upload_id)
    }
    /// Abort multipart upload.
    ///
    /// 终止分片上传。
#[cfg(feature = "async")]
    pub fn multipart_abort_upload(&self, upload_id: impl Into<String>) -> AbortUpload {
        AbortUpload::new(self.oss.clone(), upload_id)
    }
    /// Abort multipart upload (sync).
    ///
    /// 终止分片上传（同步）。
    #[cfg(feature = "sync")]
    pub fn multipart_abort_upload_sync(&self, upload_id: impl Into<String>) -> AbortUploadSync {
        AbortUploadSync::new(self.oss.clone(), upload_id)
    }
    /// List uploaded parts for a specific Upload ID.
    ///
    /// 列举指定 Upload ID 的已上传分片。
#[cfg(feature = "async")]
    pub fn multipart_list_parts(&self, upload_id: impl Into<String>) -> ListParts {
        ListParts::new(self.oss.clone(), upload_id)
    }
    /// List uploaded parts for a specific Upload ID (sync).
    ///
    /// 列举指定 Upload ID 的已上传分片（同步）。
    #[cfg(feature = "sync")]
    pub fn multipart_list_parts_sync(&self, upload_id: impl Into<String>) -> ListPartsSync {
        ListPartsSync::new(self.oss.clone(), upload_id)
    }
}
