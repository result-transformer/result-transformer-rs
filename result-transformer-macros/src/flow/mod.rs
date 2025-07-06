//! Macros for constructing flows from transformer implementations.
//!
//! These utilities bridge the gap between individual transformers and the
//! composable flow APIs.  Separate modules are provided for the synchronous
//! and asynchronous variants.

mod async_;
mod sync;
