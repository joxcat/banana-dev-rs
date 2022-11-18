#[cfg(all(feature = "sync_ureq_rustls", feature = "sync_ureq_native-tls"))]
compile_error!(
    "Only one of the `sync_ureq_rustls` and `sync_ureq_native-tls` features can be enabled at once"
);

use std::fmt::Debug;

use serde::{de::DeserializeOwned, Serialize};
use tracing::{debug, trace};
use ureq::AgentBuilder;

#[tracing::instrument(skip(body))]
pub(crate) fn post_request<'de, B, R>(url: &str, body: B) -> Result<R, crate::Error>
where
    B: Serialize + Debug,
    R: DeserializeOwned + Debug,
{
    let agent = AgentBuilder::new();

    #[cfg(feature = "sync_ureq_native-tls")]
    let agent = agent.tls_connector(std::sync::Arc::new(native_tls::TlsConnector::new()?));

    let agent = agent.user_agent("banana-dev-rust-sdk/0.1.0").build();

    trace!(body =? body, "Sending post request using ureq");
    let result = agent
        .post(url)
        .set("Content-Type", "application/json")
        .send_string(&serde_json::to_string(&body)?)?;

    debug!(code = result.status(), "Received response using ureq");

    let result: R = serde_json::from_reader(result.into_reader())?;
    trace!(data =? result, "Received data using ureq");

    Ok(result)
}
