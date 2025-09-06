#[cfg(feature = "sync")]
use tokio::runtime::Runtime;

#[cfg(feature = "sync")]
/// Run an async future to completion using a temporary Tokio runtime.
pub fn block_on<F: std::future::Future>(future: F) -> F::Output {
    Runtime::new().expect("create runtime").block_on(future)
}

