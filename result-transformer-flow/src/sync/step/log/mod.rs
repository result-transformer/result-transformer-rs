//! Steps that record values using the [`log`] crate.
//!
//! Each step logs the received value through the `log` facade and forwards it
//! unchanged to the next step. They are available only when the `logging`
//! feature is enabled. Output level and formatting are configured via
//! [`LogConfig`].

mod err_log_tap_step;
mod log_config;
mod ok_log_tap_step;
mod result_log_both_step;
mod result_log_tap_step;

pub use err_log_tap_step::ErrLogTapStep;
pub use log_config::LogConfig;
pub use ok_log_tap_step::OkLogTapStep;
pub use result_log_both_step::ResultLogBothStep;
pub use result_log_tap_step::ResultLogTapStep;
