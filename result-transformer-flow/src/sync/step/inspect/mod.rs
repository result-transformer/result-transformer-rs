//! Steps that inspect values without modifying them.
//!
//! These steps are useful for debugging or logging purposes. They run a
//! user-provided closure with a reference to the current value and then pass
//! the value through without alteration.

mod err_inspect_step;
mod ok_inspect_step;
mod result_inspect_both_step;
mod result_inspect_step;

pub use err_inspect_step::ErrInspectStep;
pub use ok_inspect_step::OkInspectStep;
pub use result_inspect_both_step::ResultInspectBothStep;
pub use result_inspect_step::ResultInspectStep;
