use std::rc::Rc;

use prelude::{CheckRequest, StartRequest};
use thiserror::Error;

#[cfg(feature = "_async")]
pub mod async_client;
pub mod banana_api;
#[cfg(feature = "_sync")]
pub mod sync_client;

pub mod prelude {
    pub use crate::{BananaClient, Error};

    pub use crate::banana_api::*;

    #[cfg(feature = "_async")]
    pub use crate::async_client::*;
    #[cfg(feature = "_sync")]
    pub use crate::sync_client::*;
}

#[derive(Debug)]
pub struct BananaClient {
    api_key: Rc<String>,
}

impl BananaClient {
    pub fn new(api_key: impl AsRef<str>) -> Self {
        Self {
            api_key: Rc::new(api_key.as_ref().to_string()),
        }
    }

    pub fn new_start_request<MI>(
        &self,
    ) -> banana_api::StartRequestBuilder<((), (), (Rc<String>,), (), (), ()), MI> {
        StartRequest::builder().api_key(self.api_key.clone())
    }

    pub fn new_check_request(
        &self,
    ) -> banana_api::CheckRequestBuilder<((), (), (Rc<String>,), (), ())> {
        CheckRequest::builder().api_key(self.api_key.clone())
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to serialize data before sending it")]
    SerializationFailed(#[from] serde_json::Error),
    #[cfg(feature = "sync_ureq")]
    #[error("Failed to send request")]
    Ureq(#[from] ureq::Error),
    #[cfg(feature = "sync_ureq_native-tls")]
    #[error("Failed to build native_tls connector")]
    SyncNativeTls(#[from] native_tls::Error),
    #[error("banana.dev returned an error {0:?}")]
    BananaDev(String),
}

#[cfg(test)]
mod tests {
    use tracing_test::traced_test;

    use crate::prelude::*;

    #[cfg(feature = "_sync")]
    #[traced_test]
    #[test]
    fn api_key_does_not_exist() {
        let client = BananaClient::new("EEEEEEEEEEE");
        let result = client
            .start_sync(
                client
                    .new_start_request()
                    .model_key("null")
                    .model_inputs(())
                    .build(),
            )
            .unwrap();
        assert!(result.message.contains("API Key does not exist"))
    }
}
