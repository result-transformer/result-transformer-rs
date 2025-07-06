//! Helper macros for implementing the core transformer traits.
//!
//! This module exposes macros for both the synchronous and asynchronous
//! transformer definitions. They generate the necessary trait implementations
//! given a minimal set of parameters.

#[cfg(feature = "core-sync")]
mod sync;

#[cfg(feature = "core-async")]
mod async_;
