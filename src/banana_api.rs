use std::{
    rc::Rc,
    time::{SystemTime, UNIX_EPOCH},
};

use serde::{Deserialize, Serialize};
use tracing::*;
use typed_builder::TypedBuilder;

#[cfg(feature = "uuid")]
fn default_uuid() -> Option<String> {
    let uuid = uuid::Uuid::new_v4();
    trace!(uuid =? uuid, "generated uuid");
    Some(uuid.to_string())
}

#[cfg(not(feature = "uuid"))]
fn default_uuid() -> Option<String> {
    trace!("uuid feature not enabled, so no uuid will be generated");
    None
}

trait MessageStatus {
    fn success(&self) -> bool;
    fn error<'o>(&'o self) -> Option<&'o str>;
}

#[derive(Debug, Serialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct StartRequest<MI> {
    /// some uuid to identify the payload
    #[builder(default = default_uuid())]
    id: Option<String>,
    /// the current Unix timestamp in seconds
    #[builder(
		default = SystemTime::now()
			.duration_since(UNIX_EPOCH)
			.expect("Time went backwards, we are before the Unix epoch")
			.as_secs()
	)]
    created: u64,
    /// your api key, for authorization
    api_key: Rc<String>,
    /// the key giving you access to this model
    #[builder(setter(into))]
    model_key: String,
    /// boolean flag to tell backend to return a callID immediately, without awaiting results.
    #[builder(default = false)]
    start_only: bool,
    /// the object passed to the model inference server
    model_inputs: MI,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StartResponse<MO> {
    /// the return payload id
    pub id: String,
    /// success or server error messages. Our API does not throw 500 errors, so always check this field for the substring "error" to catch errors
    pub message: String,
    /// the current Unix timestamp in seconds
    pub created: u64,
    /// identifier on which backend was used, to help us debug
    pub api_version: String,
    /// the async call ID used on the /check/v4 call to see the task's status
    #[serde(rename = "callID")]
    pub call_id: String,
    /// a boolean to communicate that the inference is finished and you can expect values in the modelOutputs field
    pub finished: bool,
    /// the json returned from the model inference server
    pub model_outputs: Option<Vec<MO>>,
}
impl<MO> MessageStatus for StartResponse<MO> {
    fn success(&self) -> bool {
        self.message.starts_with("success")
    }

    fn error<'o>(&'o self) -> Option<&'o str> {
        if self.success() {
            None
        } else {
            Some(&self.message)
        }
    }
}

#[derive(Debug, Serialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CheckRequest {
    /// some uuid to identify the payload
    #[builder(default = default_uuid())]
    id: Option<String>,
    /// the current Unix timestamp in seconds
    #[builder(
		default = SystemTime::now()
			.duration_since(UNIX_EPOCH)
			.expect("Time went backwards, we are before the Unix epoch")
			.as_secs()
	)]
    created: u64,
    /// your api key, for authorization
    api_key: Rc<String>,
    /// flag telling the REST call wait on the server for results, up to 50s
    #[builder(default = false)]
    long_poll: bool,
    /// the async task ID to fetch results for
    #[serde(rename = "callID")]
    call_id: String,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckResponse<MO> {
    /// the return payload id
    pub id: String,
    /// success or server error messages. Our API does not throw 500 errors, so always check this field for the substring "error" to catch errors
    pub message: String,
    /// the current Unix timestamp in seconds
    pub created: u64,
    /// identifier on which backend was used, to help us debug
    pub api_version: String,
    /// the json returned from the model inference server
    pub model_outputs: Option<Vec<MO>>,
}
impl<MO> MessageStatus for CheckResponse<MO> {
    fn success(&self) -> bool {
        self.message.starts_with("success")
    }

    fn error<'o>(&'o self) -> Option<&'o str> {
        if self.success() {
            None
        } else {
            Some(&self.message)
        }
    }
}
