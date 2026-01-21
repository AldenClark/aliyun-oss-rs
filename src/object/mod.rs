//! An object is the basic unit for storing data in OSS, composed of metadata, user data, and a key that uniquely identifies it within a bucket.
//!
//! 对象是 OSS 存储数据的基本单元，由元数据、用户数据和在桶内唯一标识的 Key 组成。

#[doc(hidden)]
pub use self::oss_object::OssObject;
#[cfg(feature = "_async-base")]
pub use self::{
    append_object::AppendObject, copy_object::CopyObject, del_object::DelObject, del_object_tagging::DelObjectTagging,
    get_object::GetObject, get_object_acl::GetObjectAcl, get_object_meta::GetObjectMeta,
    get_object_tagging::GetObjectTagging, get_object_url::GetObjectUrl, get_symlink::GetSymlink,
    head_object::HeadObject, multipart_abort_upload::AbortUpload, multipart_complete_upload::CompleteUpload,
    multipart_copyto_part::CopyToPart, multipart_init_upload::InitUpload, multipart_list_parts::ListParts,
    multipart_upload_part::UploadPart, put_object::PutObject, put_object_acl::PutObjectAcl,
    put_object_tagging::PutObjectTagging, put_symlink::PutSymlink, restore_object::RestoreObject,
    select_object::SelectObject,
};
#[cfg(feature = "_sync-base")]
pub use self::{
    append_object_sync::AppendObjectSync, copy_object_sync::CopyObjectSync, del_object_sync::DelObjectSync,
    del_object_tagging_sync::DelObjectTaggingSync, get_object_acl_sync::GetObjectAclSync,
    get_object_meta_sync::GetObjectMetaSync, get_object_sync::GetObjectSync,
    get_object_tagging_sync::GetObjectTaggingSync, get_object_url_sync::GetObjectUrlSync,
    get_symlink_sync::GetSymlinkSync, head_object_sync::HeadObjectSync, multipart_abort_upload_sync::AbortUploadSync,
    multipart_complete_upload_sync::CompleteUploadSync, multipart_copyto_part_sync::CopyToPartSync,
    multipart_init_upload_sync::InitUploadSync, multipart_list_parts_sync::ListPartsSync,
    multipart_upload_part_sync::UploadPartSync, put_object_acl_sync::PutObjectAclSync, put_object_sync::PutObjectSync,
    put_object_tagging_sync::PutObjectTaggingSync, put_symlink_sync::PutSymlinkSync,
    restore_object_sync::RestoreObjectSync, select_object_sync::SelectObjectSync,
};

#[cfg(feature = "_async-base")]
mod append_object;
#[cfg(feature = "_sync-base")]
mod append_object_sync;
#[cfg(feature = "_async-base")]
mod copy_object;
#[cfg(feature = "_sync-base")]
mod copy_object_sync;
#[cfg(feature = "_async-base")]
mod del_object;
#[cfg(feature = "_sync-base")]
mod del_object_sync;
#[cfg(feature = "_async-base")]
mod del_object_tagging;
#[cfg(feature = "_sync-base")]
mod del_object_tagging_sync;
#[cfg(feature = "_async-base")]
mod get_object;
#[cfg(feature = "_async-base")]
mod get_object_acl;
#[cfg(feature = "_sync-base")]
mod get_object_acl_sync;
#[cfg(feature = "_async-base")]
mod get_object_meta;
#[cfg(feature = "_sync-base")]
mod get_object_meta_sync;
#[cfg(feature = "_sync-base")]
mod get_object_sync;
#[cfg(feature = "_async-base")]
mod get_object_tagging;
#[cfg(feature = "_sync-base")]
mod get_object_tagging_sync;
#[cfg(feature = "_async-base")]
mod get_object_url;
#[cfg(feature = "_sync-base")]
mod get_object_url_sync;
#[cfg(feature = "_async-base")]
mod get_symlink;
#[cfg(feature = "_sync-base")]
mod get_symlink_sync;
#[cfg(feature = "_async-base")]
mod head_object;
#[cfg(feature = "_sync-base")]
mod head_object_sync;
#[cfg(feature = "_async-base")]
mod multipart_abort_upload;
#[cfg(feature = "_sync-base")]
mod multipart_abort_upload_sync;
#[cfg(feature = "_async-base")]
mod multipart_complete_upload;
#[cfg(feature = "_sync-base")]
mod multipart_complete_upload_sync;
#[cfg(feature = "_async-base")]
mod multipart_copyto_part;
#[cfg(feature = "_sync-base")]
mod multipart_copyto_part_sync;
#[cfg(feature = "_async-base")]
mod multipart_init_upload;
#[cfg(feature = "_sync-base")]
mod multipart_init_upload_sync;
#[cfg(feature = "_async-base")]
mod multipart_list_parts;
#[cfg(feature = "_sync-base")]
mod multipart_list_parts_sync;
#[cfg(feature = "_async-base")]
mod multipart_upload_part;
#[cfg(feature = "_sync-base")]
mod multipart_upload_part_sync;
mod oss_object;
#[cfg(feature = "_async-base")]
mod put_object;
#[cfg(feature = "_async-base")]
mod put_object_acl;
#[cfg(feature = "_sync-base")]
mod put_object_acl_sync;
#[cfg(feature = "_sync-base")]
mod put_object_sync;
#[cfg(feature = "_async-base")]
mod put_object_tagging;
#[cfg(feature = "_sync-base")]
mod put_object_tagging_sync;
#[cfg(feature = "_async-base")]
mod put_symlink;
#[cfg(feature = "_sync-base")]
mod put_symlink_sync;
#[cfg(feature = "_async-base")]
mod restore_object;
#[cfg(feature = "_sync-base")]
mod restore_object_sync;
#[cfg(feature = "_async-base")]
mod select_object;
#[cfg(feature = "_sync-base")]
mod select_object_sync;
