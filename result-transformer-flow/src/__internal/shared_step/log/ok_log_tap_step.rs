use result_transformer_dependencies::*;

use std::marker::PhantomData;

use super::LogConfig;

/// Step that logs the success value at a specified level.
#[derive(Debug, Clone, Copy)]
pub struct OkLogTapStep<OkType> {
    log_config: LogConfig<OkType>,
    _phantom: PhantomData<OkType>,
}

impl<OkType> OkLogTapStep<OkType> {
    /// Creates a new [`OkLogTapStep`].
    ///
    /// * `log_level` - log level used for output
    /// * `log_format` - function that converts the success value into a log message
    pub fn new(log_level: log::Level, log_format: fn(&OkType) -> String) -> Self {
        Self::with_log_config(LogConfig::new(log_level, log_format))
    }

    /// Creates a new [`OkLogTapStep`] using the provided [`LogConfig`].
    ///
    /// * `log_config` - Holds the log level for output and the function that converts
    ///   the success value into a log message
    pub fn with_log_config(log_config: LogConfig<OkType>) -> Self {
        Self {
            log_config,
            _phantom: PhantomData,
        }
    }

    pub(crate) fn apply(&self, input_ok: OkType) -> OkType {
        let msg = (self.log_config.log_format())(&input_ok);
        log::log!(*self.log_config.log_level(), "{}", msg);
        input_ok
    }
}
