//! Steps that map values to new ones.
//!
//! Each mapping step applies a closure to the incoming value and produces a
//! new one. Specific implementations operate on the success value, error
//! value, or both sides of a `Result`.

mod err_map_step;
mod ok_map_step;
mod result_map_both_bind_step;
mod result_map_both_step;
mod result_map_step;

pub use err_map_step::ErrMapStep;
pub use ok_map_step::OkMapStep;
pub use result_map_both_bind_step::ResultMapBothBindStep;
pub use result_map_both_step::ResultMapBothStep;
pub use result_map_step::ResultMapStep;
