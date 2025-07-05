use result_transformer_dependencies::*;

use std::marker::PhantomData;

use crate::sync::flow::ErrFlow;

pub struct ErrLogTapStep<ErrType> {
    log_level: log::Level,
    log_format: fn(&ErrType) -> String,
    _phantom: PhantomData<ErrType>,
}

impl<ErrType> ErrLogTapStep<ErrType> {
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

    fn apply_err(&self, input_err: ErrType) -> Self::OutputErr {
        let msg = (self.log_format)(&input_err);
        log::log!(self.log_level, "{}", msg);
        input_err
    }
}
