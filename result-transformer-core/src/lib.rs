//! Core trait definitions used by the `result-transformer` workspace.
//!
//! Depending on the enabled feature flags this crate exposes either the
//! synchronous traits under [`sync`] or the asynchronous variants under
//! [`async_`].  These traits on their own are lightweight and contain no
//! additional dependencies except when the async feature is enabled.

#[cfg(feature = "sync")]
pub mod sync;

#[cfg(feature = "async")]
pub mod async_;
