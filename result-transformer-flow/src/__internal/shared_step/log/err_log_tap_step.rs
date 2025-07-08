use result_transformer_dependencies::*;

use std::marker::PhantomData;

/// Step that logs the error value at a specified level.
#[derive(Debug, Clone, Copy)]
pub struct ErrLogTapStep<ErrType> {
    log_level: log::Level,
    log_format: fn(&ErrType) -> String,
    _phantom: PhantomData<ErrType>,
}

impl<ErrType> ErrLogTapStep<ErrType> {
    /// Creates a new [`ErrLogTapStep`].
    ///
    /// * `log_level` - log level used for output
    /// * `log_format` - function that converts the error value into a log message
    pub fn new(log_level: log::Level, log_format: fn(&ErrType) -> String) -> Self {
        Self {
            log_level,
            log_format,
            _phantom: PhantomData,
        }
    }

    pub(crate) fn apply(&self, input_err: ErrType) -> ErrType {
        let msg = (self.log_format)(&input_err);
        log::log!(self.log_level, "{}", msg);
        input_err
    }
}
