use result_transformer_dependencies::*;

use std::marker::PhantomData;

/// Step that logs the success value at a specified level.
#[derive(Debug, Clone, Copy)]
pub struct OkLogTapStep<OkType> {
    log_level: log::Level,
    log_format: fn(&OkType) -> String,
    _phantom: PhantomData<OkType>,
}

impl<OkType> OkLogTapStep<OkType> {
    /// Creates a new [`OkLogTapStep`].
    ///
    /// * `log_level` - log level used for output
    /// * `log_format` - function that converts the success value into a log message
    pub fn new(log_level: log::Level, log_format: fn(&OkType) -> String) -> Self {
        Self {
            log_level,
            log_format,
            _phantom: PhantomData,
        }
    }

    pub(crate) fn apply(&self, input_ok: OkType) -> OkType {
        let msg = (self.log_format)(&input_ok);
        log::log!(self.log_level, "{}", msg);
        input_ok
    }
}
