//! Collection of synchronous transformation steps.
//!
//! A *step* is a small, reusable unit that operates on a value. Steps are
//! typically chained together using the flow traits to build up complex
//! processing pipelines.

mod if_;
mod inspect;
mod map;
mod noop;
mod tap;

pub use if_::*;
pub use inspect::*;
pub use map::*;
pub use noop::*;
pub use tap::*;

#[cfg(feature = "logging")]
mod log;
#[cfg(feature = "logging")]
pub use log::*;
