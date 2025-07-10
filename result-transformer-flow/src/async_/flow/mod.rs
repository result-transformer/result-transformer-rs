//! Asynchronous counterparts to the synchronous flows.
//!
//! can be awaited.  The implementation relies solely on `async fn` and keeps
//! overhead minimal. For best performance you may still implement your own
//! `Async*Transformer` traits instead of chaining these flows.

mod async_err_flow;
mod async_ok_flow;
mod async_result_flow;

pub use async_err_flow::*;
pub use async_ok_flow::*;
pub use async_result_flow::*;
