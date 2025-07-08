//! Steps that inspect values without modifying them.
//!
//! They are primarily useful for debugging or logging. A user-provided
//! closure receives a reference to the current value, performs side effects
//! and then forwards the original value to the next step.

mod err_inspect_step_async;
mod ok_inspect_step_async;
mod result_inspect_both_step_async;
mod result_inspect_step_async;

pub use err_inspect_step_async::ErrInspectStepAsync;
pub use ok_inspect_step_async::OkInspectStepAsync;
pub use result_inspect_both_step_async::ResultInspectBothStepAsync;
pub use result_inspect_step_async::ResultInspectStepAsync;
