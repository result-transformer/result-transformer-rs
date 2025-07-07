use std::marker::PhantomData;

use crate::sync::flow::ResultFlow;

/// Step that maps success and error values using separate functions.
pub struct ResultMapBothStep<OkMapperFn, ErrMapperFn, InputOk, InputErr, OutputOk, OutputErr>
where
    OkMapperFn: Fn(InputOk) -> OutputOk,
    ErrMapperFn: Fn(InputErr) -> OutputErr,
{
    ok_mapper: OkMapperFn,
    err_mapper: ErrMapperFn,
    _phantom: PhantomData<(InputOk, InputErr, OutputOk, OutputErr)>,
}

impl<OkMapperFn, ErrMapperFn, InputOk, InputErr, OutputOk, OutputErr>
    ResultMapBothStep<OkMapperFn, ErrMapperFn, InputOk, InputErr, OutputOk, OutputErr>
where
    OkMapperFn: Fn(InputOk) -> OutputOk,
    ErrMapperFn: Fn(InputErr) -> OutputErr,
{
    /// Creates a new [`ResultMapBothStep`].
    ///
    /// * `ok_fn` - function to transform the success value
    /// * `err_fn` - function to transform the error value
    pub fn new(ok_fn: OkMapperFn, err_fn: ErrMapperFn) -> Self {
        Self {
            ok_mapper: ok_fn,
            err_mapper: err_fn,
            _phantom: PhantomData,
        }
    }
}

impl<OkMapperFn, ErrMapperFn, InputOk, InputErr, OutputOk, OutputErr> ResultFlow<InputOk, InputErr>
    for ResultMapBothStep<OkMapperFn, ErrMapperFn, InputOk, InputErr, OutputOk, OutputErr>
where
    OkMapperFn: Fn(InputOk) -> OutputOk,
    ErrMapperFn: Fn(InputErr) -> OutputErr,
{
    type OutputOk = OutputOk;
    type OutputErr = OutputErr;

    /// Implementation of [`ResultFlow::apply_result`].
    fn apply_result(&self, input: Result<InputOk, InputErr>) -> Result<OutputOk, OutputErr> {
        match input {
            Ok(v) => Ok((self.ok_mapper)(v)),
            Err(e) => Err((self.err_mapper)(e)),
        }
    }
}
