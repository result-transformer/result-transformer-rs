//! Collection of asynchronous transformer traits.
//!
//! These traits mirror the synchronous variants found under [`crate::sync`]
//! but expose `async` methods. Implementations typically rely on the
//! `async-trait` crate (reexported via `result-transformer-dependencies`) so
//! that they can be used on stable Rust without requiring nightly feature

mod async_err_transformer;
mod async_ok_transformer;
mod async_result_transformer;

pub use async_err_transformer::AsyncErrTransformer;
pub use async_ok_transformer::AsyncOkTransformer;
pub use async_result_transformer::AsyncResultTransformer;
