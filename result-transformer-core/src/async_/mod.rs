//! Collection of asynchronous transformer traits.
//!
//! These traits mirror the synchronous variants found under [`crate::sync`]
//! but expose `async` methods. These traits work on stable Rust using
//! `async fn` and do not require external crates for emulation.

mod async_err_transformer;
mod async_ok_transformer;
mod async_result_transformer;

pub use async_err_transformer::AsyncErrTransformer;
pub use async_ok_transformer::AsyncOkTransformer;
pub use async_result_transformer::AsyncResultTransformer;
