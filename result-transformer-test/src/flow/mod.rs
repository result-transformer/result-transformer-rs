//! Tests for the flow-based APIs.

#[cfg(feature = "flow-sync")]
mod sync;

#[cfg(feature = "flow-async")]
mod async_;

mod macros;
