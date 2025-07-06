//! Steps that execute side effects while forwarding the value.
//!
//! Tap steps allow a closure to run for its side effects using the current
//! value. After the closure completes, the value (or a mapped version) is
//! forwarded to the next step unmodified.

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
