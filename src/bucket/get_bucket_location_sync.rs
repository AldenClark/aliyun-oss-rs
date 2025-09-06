use crate::{
    error::{normal_error_sync, Error},
    request_sync::{Oss, OssRequest},
};
use bytes::Bytes;
use http::Method;
use serde_derive::Deserialize;
use std::io::Read;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct LocationConstraint {
    location_constraint: String,
}

/// Retrieve bucket location information (synchronous)
pub struct GetBucketLocationSync {
    req: OssRequest,
}
impl GetBucketLocationSync {
    pub(crate) fn new(oss: Oss) -> Self {
        let mut req = OssRequest::new(oss, Method::GET);
        req.insert_query("location", "");
        GetBucketLocationSync { req }
    }
    /// Send the request
    pub fn send(self) -> Result<String, Error> {
        let response = self.req.send_to_oss()?;
        let status = response.status();
        if status.is_success() {
            let mut reader = response.into_body().into_reader();
            let mut buf = Vec::new();
            reader.read_to_end(&mut buf)?;
            let result: LocationConstraint = serde_xml_rs::from_reader(&*buf)
                .map_err(|_| Error::OssInvalidResponse(Some(Bytes::from(buf))))?;
            Ok(result.location_constraint)
        } else {
            Err(normal_error_sync(response))
        }
    }
}
