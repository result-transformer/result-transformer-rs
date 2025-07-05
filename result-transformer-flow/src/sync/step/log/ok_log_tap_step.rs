use result_transformer_dependencies::*;

use std::marker::PhantomData;

use crate::sync::flow::OkFlow;

pub struct OkLogTapStep<OkType> {
    log_level: log::Level,
    log_format: fn(&OkType) -> String,
    _phantom: PhantomData<OkType>,
}

impl<OkType> OkLogTapStep<OkType> {
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

    fn apply_ok(&self, input_ok: OkType) -> Self::OutputOk {
        let msg = (self.log_format)(&input_ok);
        log::log!(self.log_level, "{}", msg);
        input_ok
    }
}
