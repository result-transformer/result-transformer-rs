//! Steps that simply forward the input to the next stage.
//!
//! They can act as placeholders when no transformation is desired within a flow.

mod err_noop_step;
mod ok_noop_step;
mod result_noop_step;

pub use err_noop_step::ErrNoopStep;
pub use ok_noop_step::OkNoopStep;
pub use result_noop_step::ResultNoopStep;
