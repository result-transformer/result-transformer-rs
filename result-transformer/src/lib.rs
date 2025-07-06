//! Reexports the core transformer traits along with optional helper crates.
//!
//! Enable the desired features in your `Cargo.toml` to bring the various
//! synchronous or asynchronous APIs into scope.  Consumers typically depend on
//! this crate and activate the features they need:
//!
//! ```toml
//! [dependencies]
//! result-transformer = { version = "0.0.1", features = ["core-sync", "result-transformer-macros"] }
//! ```

#[doc(hidden)]
pub mod __internal;

#[cfg(feature = "core-sync")]
pub use result_transformer_core::sync::{self, *};

#[cfg(feature = "core-async")]
pub use result_transformer_core::async_::{self, *};

#[cfg(feature = "result-transformer-flow")]
pub use result_transformer_flow as flow;

#[cfg(feature = "result-transformer-macros")]
pub use result_transformer_macros as macros;

#[cfg(feature = "result-transformer-derive")]
pub use result_transformer_derive as derive;
