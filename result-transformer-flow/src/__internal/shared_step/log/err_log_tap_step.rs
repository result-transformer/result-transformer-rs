use result_transformer_dependencies::*;

use std::marker::PhantomData;

use super::LogConfig;

/// Step that logs the error value at a specified level.
#[derive(Debug, Clone, Copy)]
pub struct ErrLogTapStep<ErrType> {
    log_config: LogConfig<ErrType>,
    _phantom: PhantomData<ErrType>,
}

impl<ErrType> ErrLogTapStep<ErrType> {
    /// Creates a new [`ErrLogTapStep`].
    ///
    /// * `log_level` - log level used for output
    /// * `log_format` - function that converts the error value into a log message
    pub fn new(log_level: log::Level, log_format: fn(&ErrType) -> String) -> Self {
        Self::with_log_config(LogConfig::new(log_level, log_format))
    }

    /// Creates a new [`ErrLogTapStep`] using the provided [`LogConfig`].
    ///
    /// * `log_config` - Holds the log level for output and the function that converts
    ///   the error value into a log message
    pub fn with_log_config(log_config: LogConfig<ErrType>) -> Self {
        Self {
            log_config,
            _phantom: PhantomData,
        }
    }

    pub(crate) fn apply(&self, input_err: ErrType) -> ErrType {
        let msg = (self.log_config.log_format())(&input_err);
        log::log!(*self.log_config.log_level(), "{}", msg);
        input_err
    }
}
