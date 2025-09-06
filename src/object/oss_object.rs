use super::{
    del_object::DelObject, AbortUpload, AppendObject, CompleteUpload, CopyObject, CopyToPart,
    DelObjectTagging, GetObject, GetObjectAcl, GetObjectMeta, GetObjectTagging, GetObjectUrl,
    GetSymlink, HeadObject, InitUpload, ListParts, PutObject, PutObjectAcl, PutObjectTagging,
    PutSymlink, RestoreObject, UploadPart,
    };
    use crate::{common::Acl, oss::Oss};

/// OSS object implementing APIs such as uploading and deleting files
#[derive(Debug, Clone)]
pub struct OssObject {
    oss: Oss,
}

impl OssObject {
    pub(crate) fn new(mut oss: Oss, object: impl ToString) -> Self {
        oss.set_object(object);
        OssObject { oss }
    }
    /// Upload a file to OSS
    pub fn put_object(&self) -> PutObject {
        PutObject::new(self.oss.clone())
    }
    /// Append to a file
    pub fn append_object(&self) -> AppendObject {
        AppendObject::new(self.oss.clone())
    }
    /// Delete a file
    pub fn del_object(&self) -> DelObject {
        DelObject::new(self.oss.clone())
    }
    /// Get the object's access URL
    pub fn get_object_url(&self) -> GetObjectUrl {
        GetObjectUrl::new(self.oss.clone())
    }
    /// Get the object's tag information
    pub fn get_object_tagging(&self) -> GetObjectTagging {
        GetObjectTagging::new(self.oss.clone())
    }
    /// Get the object's full metadata
    pub fn head_object(&self) -> HeadObject {
        HeadObject::new(self.oss.clone())
    }
    /// Get the object's meta information
    pub fn get_object_meta(&self) -> GetObjectMeta {
        GetObjectMeta::new(self.oss.clone())
    }
    /// Get the object's ACL
    pub fn get_object_acl(&self) -> GetObjectAcl {
        GetObjectAcl::new(self.oss.clone())
    }
    /// Get the object's content
    pub fn get_object(&self) -> GetObject {
        GetObject::new(self.oss.clone())
    }
    /// Copy the object
    pub fn copy_object(&self, copy_source: &str) -> CopyObject {
        CopyObject::new(self.oss.clone(), copy_source)
    }
    /// Restore the object
    pub fn restore_object(&self) -> RestoreObject {
        RestoreObject::new(self.oss.clone())
    }
    /// Set the object's ACL
    pub fn put_object_acl(&self, acl: Acl) -> PutObjectAcl {
        PutObjectAcl::new(self.oss.clone(), acl)
    }
    /// Set the object's tags
    pub fn put_object_tagging(
        &self,
        tags: Vec<(impl ToString, impl ToString)>,
    ) -> PutObjectTagging {
        PutObjectTagging::new(self.oss.clone(), tags)
    }
    /// Create a symlink
    pub fn put_symlink(&self, symlink_target: impl ToString) -> PutSymlink {
        PutSymlink::new(self.oss.clone(), symlink_target)
    }
    /// Get the symlink
    pub fn get_symlink(&self) -> GetSymlink {
        GetSymlink::new(self.oss.clone())
    }
    /// Remove all tags from the object
    pub fn del_object_tagging(&self) -> DelObjectTagging {
        DelObjectTagging::new(self.oss.clone())
    }
    /// Initialize a multipart upload
    pub fn multipart_init_upload(&self) -> InitUpload {
        InitUpload::new(self.oss.clone())
    }
    /// Upload a part
    pub fn multipart_upload_part(&self, part_number: u32, upload_id: impl ToString) -> UploadPart {
        UploadPart::new(self.oss.clone(), part_number, upload_id)
    }
    /// Copy object content to a part
    pub fn multipart_copy_part(
        &self,
        part_number: u32,
        upload_id: impl ToString,
        copy_source: impl ToString,
    ) -> CopyToPart {
        CopyToPart::new(self.oss.clone(), part_number, upload_id, copy_source)
    }
    /// Complete the multipart upload
    pub fn multipart_complete_upload(&self, upload_id: impl ToString) -> CompleteUpload<'_> {
        CompleteUpload::new(self.oss.clone(), upload_id)
    }
    /// Abort the multipart upload
    pub fn multipart_abort_upload(&self, upload_id: impl ToString) -> AbortUpload {
        AbortUpload::new(self.oss.clone(), upload_id)
    }
    /// List all successfully uploaded parts for the specified Upload ID
    pub fn multipart_list_parts(&self, upload_id: impl ToString) -> ListParts {
        ListParts::new(self.oss.clone(), upload_id)
    }
}
