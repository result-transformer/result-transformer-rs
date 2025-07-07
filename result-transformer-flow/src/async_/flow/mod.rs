//! Asynchronous counterparts to the synchronous flows.
//!
//! These traits behave like the blocking variants but return `Future`s so they
//! can be awaited.  Implementation uses [`async-trait`](https://docs.rs/async-
//! trait) which adds a small runtime cost.  For best performance consider
//! writing your own `Async*Transformer` implementations rather than chaining
//! these flows.

mod async_err_flow;
mod async_ok_flow;
mod async_result_flow;

pub use async_err_flow::*;
pub use async_ok_flow::*;
pub use async_result_flow::*;
