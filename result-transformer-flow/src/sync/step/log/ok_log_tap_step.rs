use result_transformer_dependencies::*;

use std::marker::PhantomData;

use crate::sync::flow::OkFlow;

/// Step that logs the success value at a specified level.
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
}

impl<OkType> OkFlow<OkType> for OkLogTapStep<OkType> {
    type OutputOk = OkType;

    /// Implementation of [`OkFlow::apply_ok`].
    /// Logs the value and returns the original success.
    fn apply_ok(&self, input_ok: OkType) -> Self::OutputOk {
        let msg = (self.log_format)(&input_ok);
        log::log!(self.log_level, "{}", msg);
        input_ok
    }
}
