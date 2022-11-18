#[cfg(all(feature = "async_tokio_rustls", feature = "async_tokio_native-tls"))]
compile_error!(
    "Only one of the `async_tokio_rustls` and `async_tokio_native-tls` features can be enabled at once"
);
