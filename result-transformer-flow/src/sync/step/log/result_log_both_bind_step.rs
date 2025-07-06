use result_transformer_dependencies::*;
use std::marker::PhantomData;

use super::LogConfig;
use crate::sync::flow::ResultFlow;

/// Logs both values and produces new `Result`s from the provided closures.
pub struct ResultLogBothBindStep<OkType, ErrType, OutputOk, OutputErr, OkFn, ErrFn>
where
    OkFn: Fn(OkType) -> Result<OutputOk, OutputErr>,
    ErrFn: Fn(ErrType) -> Result<OutputOk, OutputErr>,
{
    ok_log: LogConfig<OkType>,
    err_log: LogConfig<ErrType>,
    ok_fn: OkFn,
    err_fn: ErrFn,
    _phantom: PhantomData<(OutputOk, OutputErr)>,
}

impl<OkType, ErrType, OutputOk, OutputErr, OkFn, ErrFn>
    ResultLogBothBindStep<OkType, ErrType, OutputOk, OutputErr, OkFn, ErrFn>
where
    OkFn: Fn(OkType) -> Result<OutputOk, OutputErr>,
    ErrFn: Fn(ErrType) -> Result<OutputOk, OutputErr>,
{
    pub fn new(
        ok_log: LogConfig<OkType>,
        err_log: LogConfig<ErrType>,
        ok_fn: OkFn,
        err_fn: ErrFn,
    ) -> Self {
        Self {
            ok_log,
            err_log,
            ok_fn,
            err_fn,
            _phantom: PhantomData,
        }
    }
}

impl<OkType, ErrType, OutputOk, OutputErr, OkFn, ErrFn> ResultFlow<OkType, ErrType>
    for ResultLogBothBindStep<OkType, ErrType, OutputOk, OutputErr, OkFn, ErrFn>
where
    OkFn: Fn(OkType) -> Result<OutputOk, OutputErr>,
    ErrFn: Fn(ErrType) -> Result<OutputOk, OutputErr>,
{
    type OutputOk = OutputOk;
    type OutputErr = OutputErr;

    fn apply_result(&self, input_result: Result<OkType, ErrType>) -> Result<OutputOk, OutputErr> {
        match input_result {
            Ok(ok) => {
                log::log!(
                    *self.ok_log.log_level(),
                    "{}",
                    (self.ok_log.log_format())(&ok)
                );
                (self.ok_fn)(ok)
            }
            Err(err) => {
                log::log!(
                    *self.err_log.log_level(),
                    "{}",
                    (self.err_log.log_format())(&err)
                );
                (self.err_fn)(err)
            }
        }
    }
}
