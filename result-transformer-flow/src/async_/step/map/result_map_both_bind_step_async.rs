use std::{marker::PhantomData, pin::Pin};

use crate::async_::AsyncResultFlow;

/// Step that maps success and error values using functions returning a `Result`.
#[derive(Debug, Clone, Copy)]
pub struct ResultMapBothBindStepAsync<
    OkMapperFn,
    ErrMapperFn,
    InputOk,
    InputErr,
    OutputOk,
    OutputErr,
> where
    OkMapperFn: Fn(InputOk) -> Pin<Box<dyn Future<Output = Result<OutputOk, OutputErr>> + Send + Sync>>
        + Send
        + Sync,
    ErrMapperFn: Fn(InputErr) -> Pin<Box<dyn Future<Output = Result<OutputOk, OutputErr>> + Send + Sync>>
        + Send
        + Sync,
    InputOk: Send + Sync,
    InputErr: Send + Sync,
    OutputOk: Send + Sync,
    OutputErr: Send + Sync,
{
    ok_mapper: OkMapperFn,
    err_mapper: ErrMapperFn,
    _phantom: PhantomData<(InputOk, InputErr, OutputOk, OutputErr)>,
}

impl<OkMapperFn, ErrMapperFn, InputOk, InputErr, OutputOk, OutputErr>
    ResultMapBothBindStepAsync<OkMapperFn, ErrMapperFn, InputOk, InputErr, OutputOk, OutputErr>
where
    OkMapperFn: Fn(InputOk) -> Pin<Box<dyn Future<Output = Result<OutputOk, OutputErr>> + Send + Sync>>
        + Send
        + Sync,
    ErrMapperFn: Fn(InputErr) -> Pin<Box<dyn Future<Output = Result<OutputOk, OutputErr>> + Send + Sync>>
        + Send
        + Sync,
    InputOk: Send + Sync,
    InputErr: Send + Sync,
    OutputOk: Send + Sync,
    OutputErr: Send + Sync,
{
    /// Creates a new [`ResultMapBothBindStepAsync`].
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

impl<OkMapperFn, ErrMapperFn, InputOk, InputErr, OutputOk, OutputErr>
    AsyncResultFlow<InputOk, InputErr>
    for ResultMapBothBindStepAsync<OkMapperFn, ErrMapperFn, InputOk, InputErr, OutputOk, OutputErr>
where
    OkMapperFn: Fn(InputOk) -> Pin<Box<dyn Future<Output = Result<OutputOk, OutputErr>> + Send + Sync>>
        + Send
        + Sync,
    ErrMapperFn: Fn(InputErr) -> Pin<Box<dyn Future<Output = Result<OutputOk, OutputErr>> + Send + Sync>>
        + Send
        + Sync,
    InputOk: Send + Sync,
    InputErr: Send + Sync,
    OutputOk: Send + Sync,
    OutputErr: Send + Sync,
{
    type OutputOk = OutputOk;
    type OutputErr = OutputErr;

    fn apply_result_async<'a>(
        &'a self,
        input_result: Result<InputOk, InputErr>,
    ) -> impl Future<Output = Result<Self::OutputOk, Self::OutputErr>> + Send + 'a {
        match input_result {
            Ok(ok) => (self.ok_mapper)(ok),
            Err(err) => (self.err_mapper)(err),
        }
    }
}
