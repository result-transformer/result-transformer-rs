//! Collection of synchronous transformer traits.
//!
//! These traits are intended for lightweight conversions that do not require
//! asynchronous operations. They form the foundation of the higher level flow
//! utilities provided by other crates in this workspace.

#[cfg(feature = "sync-macros")]
pub mod macros;

mod err_transformer;
mod ok_transformer;
mod result_transformer;

pub use err_transformer::ErrTransformer;
pub use ok_transformer::OkTransformer;
pub use result_transformer::ResultTransformer;
