use result_transformer_dependencies::*;

use std::marker::PhantomData;

use super::LogConfig;

/// Step that logs both success and error values according to [`LogConfig`].
#[derive(Debug, Clone, Copy)]
pub struct ResultLogTapStep<OkType, ErrType> {
    ok_log: Option<LogConfig<OkType>>,
    err_log: Option<LogConfig<ErrType>>,
    _phantom: PhantomData<(OkType, ErrType)>,
}

impl<OkType, ErrType> ResultLogTapStep<OkType, ErrType> {
    /// Creates a new [`ResultLogTapStep`] with optional log settings.
    pub fn new(ok_log: Option<LogConfig<OkType>>, err_log: Option<LogConfig<ErrType>>) -> Self {
        Self {
            ok_log,
            err_log,
            _phantom: PhantomData,
        }
    }

    /// Creates a step that logs only the success value.
    ///
    /// * `level` - log level to use
    /// * `format` - formatter for the success value
    pub fn with_ok_log(level: log::Level, format: fn(&OkType) -> String) -> Self {
        Self {
            ok_log: Some(LogConfig::new(level, format)),
            err_log: None,
            _phantom: PhantomData,
        }
    }

    /// Creates a step that logs only the error value.
    pub fn with_err_log(level: log::Level, format: fn(&ErrType) -> String) -> Self {
        Self {
            ok_log: None,
            err_log: Some(LogConfig::new(level, format)),
            _phantom: PhantomData,
        }
    }

    /// Creates a step that logs both success and error values.
    pub fn with_both_logs(
        ok_level: log::Level,
        ok_format: fn(&OkType) -> String,
        err_level: log::Level,
        err_format: fn(&ErrType) -> String,
    ) -> Self {
        Self {
            ok_log: Some(LogConfig::new(ok_level, ok_format)),
            err_log: Some(LogConfig::new(err_level, err_format)),
            _phantom: PhantomData,
        }
    }

    /// Creates a step that performs no logging.
    pub fn silent() -> Self {
        Self {
            ok_log: None,
            err_log: None,
            _phantom: PhantomData,
        }
    }

    pub(crate) fn apply(&self, input_result: Result<OkType, ErrType>) -> Result<OkType, ErrType> {
        match &input_result {
            Ok(ok) => {
                if let Some(ok_log) = &self.ok_log {
                    log::log!(*ok_log.log_level(), "{}", (ok_log.log_format())(ok));
                }
            }
            Err(err) => {
                if let Some(err_log) = &self.err_log {
                    log::log!(*err_log.log_level(), "{}", (err_log.log_format())(err));
                }
            }
        }
        input_result
    }
}
