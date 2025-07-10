use result_transformer_dependencies::*;
use std::marker::PhantomData;

use super::LogConfig;

/// Step that logs both success and error values without modifying them.
#[derive(Debug, Clone, Copy)]
pub struct ResultLogTapBothStep<OkType, ErrType> {
    ok_log_config: LogConfig<OkType>,
    err_log_config: LogConfig<ErrType>,
    _phantom: PhantomData<(OkType, ErrType)>,
}

impl<OkType, ErrType> ResultLogTapBothStep<OkType, ErrType> {
    /// Creates a new [`ResultLogTapBothStep`].
    ///
    /// * `ok_log_level` - log level used for the success value
    /// * `ok_log_format` - function that converts the success value into a log message
    /// * `err_log_level` - log level used for the error value
    /// * `err_log_format` - function that converts the error value into a log message
    pub fn new(
        ok_log_level: log::Level,
        ok_log_format: fn(&OkType) -> String,
        err_log_level: log::Level,
        err_log_format: fn(&ErrType) -> String,
    ) -> Self {
        Self::with_log_config(
            LogConfig::new(ok_log_level, ok_log_format),
            LogConfig::new(err_log_level, err_log_format),
        )
    }

    /// Creates a new [`ResultLogTapBothStep`] using the provided [`LogConfig`] values.
    ///
    /// * `ok_log_config` - log settings for the success value
    /// * `err_log_config` - log settings for the error value
    pub fn with_log_config(
        ok_log_config: LogConfig<OkType>,
        err_log_config: LogConfig<ErrType>,
    ) -> Self {
        Self {
            ok_log_config,
            err_log_config,
            _phantom: PhantomData,
        }
    }

    pub(crate) fn apply(&self, input_result: Result<OkType, ErrType>) -> Result<OkType, ErrType> {
        match &input_result {
            Ok(ok) => log::log!(
                *self.ok_log_config.log_level(),
                "{}",
                (self.ok_log_config.log_format())(ok)
            ),
            Err(err) => log::log!(
                *self.err_log_config.log_level(),
                "{}",
                (self.err_log_config.log_format())(err)
            ),
        }
        input_result
    }
}
