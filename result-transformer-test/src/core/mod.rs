//! Tests covering the core transformer traits.

#[cfg(feature = "core-sync")]
mod sync;

#[cfg(feature = "core-async")]
mod async_;

mod macros;
