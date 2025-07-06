use std::marker::PhantomData;

use crate::sync::flow::ResultFlow;

pub struct ResultMapBothBindStep<OkMapperFn, ErrMapperFn, InputOk, InputErr, OutputOk, OutputErr>
where
    OkMapperFn: Fn(InputOk) -> Result<OutputOk, OutputErr>,
    ErrMapperFn: Fn(InputErr) -> Result<OutputOk, OutputErr>,
{
    ok_mapper: OkMapperFn,
    err_mapper: ErrMapperFn,
    _phantom: PhantomData<(InputOk, InputErr, OutputOk, OutputErr)>,
}

impl<OkMapperFn, ErrMapperFn, InputOk, InputErr, OutputOk, OutputErr>
    ResultMapBothBindStep<OkMapperFn, ErrMapperFn, InputOk, InputErr, OutputOk, OutputErr>
where
    OkMapperFn: Fn(InputOk) -> Result<OutputOk, OutputErr>,
    ErrMapperFn: Fn(InputErr) -> Result<OutputOk, OutputErr>,
{
    pub fn new(ok_fn: OkMapperFn, err_fn: ErrMapperFn) -> Self {
        Self {
            ok_mapper: ok_fn,
            err_mapper: err_fn,
            _phantom: PhantomData,
        }
    }
}

impl<OkMapperFn, ErrMapperFn, InputOk, InputErr, OutputOk, OutputErr> ResultFlow<InputOk, InputErr>
    for ResultMapBothBindStep<OkMapperFn, ErrMapperFn, InputOk, InputErr, OutputOk, OutputErr>
where
    OkMapperFn: Fn(InputOk) -> Result<OutputOk, OutputErr>,
    ErrMapperFn: Fn(InputErr) -> Result<OutputOk, OutputErr>,
{
    type OutputOk = OutputOk;
    type OutputErr = OutputErr;

    fn apply_result(&self, input: Result<InputOk, InputErr>) -> Result<OutputOk, OutputErr> {
        match input {
            Ok(v) => (self.ok_mapper)(v),
            Err(e) => (self.err_mapper)(e),
        }
    }
}
