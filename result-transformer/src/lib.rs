//! Reexports the core transformer traits along with optional helper crates.
//!
//! Enable the desired features in your `Cargo.toml` to bring the various
//! synchronous or asynchronous APIs into scope.  Consumers typically depend on
//! this crate and activate the features they need:
//!
//! ```toml
//! [dependencies]
//! result-transformer = { version = "0.0.2", features = ["core-sync", "core-sync-macros"] }
//! ```

#[doc(hidden)]
pub mod __internal;

#[cfg(feature = "result-transformer-core")]
pub use result_transformer_core as core;

#[cfg(feature = "core-sync")]
pub use core::sync::{self, ErrTransformer, OkTransformer, ResultTransformer};

#[cfg(feature = "core-async")]
pub use core::async_::{self, AsyncErrTransformer, AsyncOkTransformer, AsyncResultTransformer};

#[cfg(feature = "result-transformer-flow")]
pub use result_transformer_flow as flow;
