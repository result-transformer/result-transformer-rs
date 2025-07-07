//! Steps that perform side effects while forwarding the value.
//!
//! Tap steps pass the current value to a closure to run side effects and then
//! forward the value (or a transformed value) to the next step.

mod err_tap_step;
mod ok_tap_step;
mod result_tap_both_bind_step;
mod result_tap_both_step;
mod result_tap_step;

pub use err_tap_step::ErrTapStep;
pub use ok_tap_step::OkTapStep;
pub use result_tap_both_bind_step::ResultTapBothBindStep;
pub use result_tap_both_step::ResultTapBothStep;
pub use result_tap_step::ResultTapStep;
