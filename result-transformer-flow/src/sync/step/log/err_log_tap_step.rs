use result_transformer_dependencies::*;

use std::marker::PhantomData;

use crate::sync::flow::ErrFlow;

/// Step that logs the error value at a specified level.
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
}

impl<ErrType> ErrFlow<ErrType> for ErrLogTapStep<ErrType> {
    type OutputErr = ErrType;

    /// Implementation of [`ErrFlow::apply_err`].
    /// Logs the value and then returns the original error.
    fn apply_err(&self, input_err: ErrType) -> Self::OutputErr {
        let msg = (self.log_format)(&input_err);
        log::log!(self.log_level, "{}", msg);
        input_err
    }
}
