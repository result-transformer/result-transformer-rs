//! Helper flows and steps for composing transformations.
//!
//! This crate supplements [`result-transformer-core`](https://crates.io/crates/result-transformer-core)
//! by providing composable "flows" that can be chained together. Each flow
//! consumes an input value and produces an output of the same `Result` shape.
//! Both synchronous and asynchronous variants are available behind feature
//! flags. Enable the `"sync"` feature for blocking flows or `"async"` for
//! versions that operate with `async`/`await`.
//!
//! The synchronous flows are zero-cost abstractions built purely from trait
//! bounds.  The async versions rely on the [`async-trait`](https://docs.rs/async-trait)
//! crate and therefore have extra runtime overhead. When you need asynchronous
//! behavior, consider implementing the `Async*Transformer` traits directly
//! instead of relying on these flow utilities.

mod __internal;

#[cfg(feature = "sync")]
pub mod sync;

#[cfg(feature = "async")]
pub mod async_;
