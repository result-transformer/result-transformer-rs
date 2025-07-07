//! Mapping steps that convert a value into another value.
//!
//! Each step applies a closure to the input to produce a new value. Variants
//! exist for mapping only the success variant, only the error variant or both
//! sides of a `Result`.

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
