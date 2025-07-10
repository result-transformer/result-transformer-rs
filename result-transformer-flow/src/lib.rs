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
//! bounds.  The async versions are implemented with `async fn` and remain
//! lightweight. If you require maximum performance, implementing the
//! `Async*Transformer` traits directly may still be preferable.

mod __internal;

#[cfg(feature = "sync")]
pub mod sync;

#[cfg(feature = "async")]
pub mod async_;
