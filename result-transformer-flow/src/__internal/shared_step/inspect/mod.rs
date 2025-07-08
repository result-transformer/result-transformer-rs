//! Steps that inspect values without modifying them.
//!
//! They are primarily useful for debugging or logging. A user-provided
//! closure receives a reference to the current value, performs side effects
//! and then forwards the original value to the next step.

mod err_inspect_step;
mod ok_inspect_step;
mod result_inspect_both_step;
mod result_inspect_step;

pub use err_inspect_step::ErrInspectStep;
pub use ok_inspect_step::OkInspectStep;
pub use result_inspect_both_step::ResultInspectBothStep;
pub use result_inspect_step::ResultInspectStep;
