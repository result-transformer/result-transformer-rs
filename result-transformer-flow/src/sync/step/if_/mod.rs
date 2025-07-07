//! Steps that branch the execution flow based on a condition.
//!
//! Each step evaluates a predicate and delegates to different flows for the
//! `true` and `false` cases. Variants exist for branching on `Ok` values,
//! `Err` values or the entire `Result`.

mod err_if_step;
mod ok_if_step;
mod result_if_step;

pub use err_if_step::ErrIfStep;
pub use ok_if_step::OkIfStep;
pub use result_if_step::ResultIfStep;
