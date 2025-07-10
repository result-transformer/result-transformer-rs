use result_transformer_dependencies::*;

use std::marker::PhantomData;

use super::LogConfig;

/// Step that logs both success and error values according to [`LogConfig`].
#[derive(Debug, Clone, Copy)]
pub struct ResultLogTapStep<OkType, ErrType> {
    log_config: LogConfig<Result<OkType, ErrType>>,
    _phantom: PhantomData<(OkType, ErrType)>,
}

impl<OkType, ErrType> ResultLogTapStep<OkType, ErrType> {
    /// Creates a new [`ResultLogTapStep`].
    ///
    /// * `log_level` - log level used for output
    /// * `log_format` - function that converts the `Result` value into a log message
    pub fn new(log_level: log::Level, log_format: fn(&Result<OkType, ErrType>) -> String) -> Self {
        Self::with_log_config(LogConfig::new(log_level, log_format))
    }

    /// Creates a new [`ResultLogTapStep`] using the provided [`LogConfig`].
    ///
    /// * `log_config` - Holds the log level for output and the function
    ///   that converts the `Result` value into a log message
    pub fn with_log_config(log_config: LogConfig<Result<OkType, ErrType>>) -> Self {
        Self {
            log_config,
            _phantom: PhantomData,
        }
    }

    pub(crate) fn apply(&self, input_result: Result<OkType, ErrType>) -> Result<OkType, ErrType> {
        log::log!(
            *self.log_config.log_level(),
            "{}",
            (self.log_config.log_format())(&input_result)
        );
        input_result
    }
}
