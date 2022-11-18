use std::fmt::Debug;

use crate::banana_api::{CheckRequest, CheckResponse, StartRequest, StartResponse};

#[cfg(feature = "sync_ureq")]
mod ureq_backend;
#[cfg(feature = "sync_ureq")]
pub(crate) use ureq_backend::*;

pub trait SyncClient {
    type Output: serde::de::DeserializeOwned + Debug;

    fn start_sync<MI>(
        &self,
        request: StartRequest<MI>,
    ) -> Result<StartResponse<Self::Output>, crate::Error>
    where
        MI: serde::Serialize + Debug,
    {
        // TODO: Check banana.dev result
        #[cfg(any(feature = "sync_ureq"))]
        post_request("https://api.banana.dev/start/v4/", request)
    }

    fn check_sync<MO>(
        &self,
        request: CheckRequest,
    ) -> Result<CheckResponse<Self::Output>, crate::Error>
    where
        MO: serde::de::DeserializeOwned + Debug,
    {
        // TODO: Check banana.dev result
        #[cfg(any(feature = "sync_ureq"))]
        post_request("https://api.banana.dev/check/v4/", request)
    }
}

impl SyncClient for crate::BananaClient {
    type Output = serde_json::Value;
}
