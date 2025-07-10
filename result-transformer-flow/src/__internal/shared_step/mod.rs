//! Collection of steps used in synchronous flows.
//!
//! Steps are reusable units that apply small transformations or side effects.
//! By chaining them with traits such as `OkFlow`, `ErrFlow` and `ResultFlow`,
//! complex pipelines can be expressed concisely. This module offers generic
//! steps for branching, mapping, inspection and logging.
//!
//! This module provides the following step categories:
//!
//! - [`if_`]: conditional branching steps
//! - [`inspect`]: view values without modifying them
//! - [`map`]: transform values into new ones
//! - [`noop`]: pass values through unchanged
//! - [`tap`]: run side effects while forwarding values
//! - [`log`] *(requires the `logging` feature)*: log output via the `log` crate

mod inspect;
mod map;
mod noop;
mod tap;

pub use inspect::*;
pub use map::*;
pub use noop::*;
pub use tap::*;

#[cfg(feature = "logging")]
mod log;
#[cfg(feature = "logging")]
pub use log::*;
