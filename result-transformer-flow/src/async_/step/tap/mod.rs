//! Steps that perform side effects while forwarding the value.
//!
//! Tap steps pass the current value to a closure to run side effects and then
//! forward the value (or a transformed value) to the next step.

mod err_tap_step_async;
mod ok_tap_step_async;
mod result_tap_both_bind_step_async;
mod result_tap_both_step_async;
mod result_tap_step_async;

pub use err_tap_step_async::ErrTapStepAsync;
pub use ok_tap_step_async::OkTapStepAsync;
pub use result_tap_both_bind_step_async::ResultTapBothBindStepAsync;
pub use result_tap_both_step_async::ResultTapBothStepAsync;
pub use result_tap_step_async::ResultTapStepAsync;
