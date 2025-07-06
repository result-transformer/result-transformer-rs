//! Conditional branching steps.
//!
//! Each step in this module evaluates a predicate and then selects one of two
//! flows to continue execution. Different variants target the `Ok`, `Err`, or
//! entire `Result` value.

mod err_if_step;
mod ok_if_step;
mod result_if_step;

pub use err_if_step::ErrIfStep;
pub use ok_if_step::OkIfStep;
pub use result_if_step::ResultIfStep;
