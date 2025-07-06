//! Definitions for composing synchronous flows.

mod err_flow;
mod ok_flow;
mod result_flow;

pub use err_flow::*;
pub use ok_flow::*;
pub use result_flow::*;
