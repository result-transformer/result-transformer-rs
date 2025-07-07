use result_transformer_dependencies::*;
use std::marker::PhantomData;

use super::LogConfig;
use crate::sync::flow::ResultFlow;

/// Logs both success and error values and returns them unchanged.
pub struct ResultLogBothStep<OkType, ErrType> {
    ok_log: LogConfig<OkType>,
    err_log: LogConfig<ErrType>,
    _phantom: PhantomData<(OkType, ErrType)>,
}

impl<OkType, ErrType> ResultLogBothStep<OkType, ErrType> {
    pub fn new(ok_log: LogConfig<OkType>, err_log: LogConfig<ErrType>) -> Self {
        Self {
            ok_log,
            err_log,
            _phantom: PhantomData,
        }
    }

    pub fn new_levels(
        ok_level: log::Level,
        ok_format: fn(&OkType) -> String,
        err_level: log::Level,
        err_format: fn(&ErrType) -> String,
    ) -> Self {
        Self {
            ok_log: LogConfig::new(ok_level, ok_format),
            err_log: LogConfig::new(err_level, err_format),
            _phantom: PhantomData,
        }
    }
}

impl<OkType, ErrType> ResultFlow<OkType, ErrType> for ResultLogBothStep<OkType, ErrType> {
    type OutputOk = OkType;
    type OutputErr = ErrType;

    fn apply_result(
        &self,
        input_result: Result<OkType, ErrType>,
    ) -> Result<Self::OutputOk, Self::OutputErr> {
        match &input_result {
            Ok(ok) => log::log!(
                *self.ok_log.log_level(),
                "{}",
                (self.ok_log.log_format())(ok)
            ),
            Err(err) => log::log!(
                *self.err_log.log_level(),
                "{}",
                (self.err_log.log_format())(err)
            ),
        }
        input_result
    }
}
