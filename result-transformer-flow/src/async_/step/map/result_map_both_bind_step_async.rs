use core::future::Future;
use std::marker::PhantomData;

use crate::async_::AsyncResultFlow;

/// Step that maps success and error values using functions returning a `Result`.
#[derive(Debug, Clone, Copy)]
pub struct ResultMapBothBindStepAsync<
    InputOk,
    InputErr,
    OutputOk,
    OutputErr,
    OkMapperFn,
    ErrMapperFn,
    OkMapperFut,
    ErrMapperFut,
> where
    InputOk: Send + Sync,
    InputErr: Send + Sync,
    OutputOk: Send + Sync,
    OutputErr: Send + Sync,
    OkMapperFn: Fn(InputOk) -> OkMapperFut + Send + Sync,
    ErrMapperFn: Fn(InputErr) -> ErrMapperFut + Send + Sync,
    OkMapperFut: Future<Output = Result<OutputOk, OutputErr>> + Send,
    ErrMapperFut: Future<Output = Result<OutputOk, OutputErr>> + Send,
{
    ok_mapper: OkMapperFn,
    err_mapper: ErrMapperFn,
    _phantom: PhantomData<(InputOk, InputErr)>,
}

impl<InputOk, InputErr, OutputOk, OutputErr, OkMapperFn, ErrMapperFn, OkMapperFut, ErrMapperFut>
    ResultMapBothBindStepAsync<
        InputOk,
        InputErr,
        OutputOk,
        OutputErr,
        OkMapperFn,
        ErrMapperFn,
        OkMapperFut,
        ErrMapperFut,
    >
where
    InputOk: Send + Sync,
    InputErr: Send + Sync,
    OutputOk: Send + Sync,
    OutputErr: Send + Sync,
    OkMapperFn: Fn(InputOk) -> OkMapperFut + Send + Sync,
    ErrMapperFn: Fn(InputErr) -> ErrMapperFut + Send + Sync,
    OkMapperFut: Future<Output = Result<OutputOk, OutputErr>> + Send,
    ErrMapperFut: Future<Output = Result<OutputOk, OutputErr>> + Send,
{
    /// Creates a new [`ResultMapBothBindStepAsync`].
    ///
    /// * `ok_fn` - function to transform the success value
    /// * `err_fn` - function to transform the error value
    pub const fn new(ok_fn: OkMapperFn, err_fn: ErrMapperFn) -> Self {
        Self {
            ok_mapper: ok_fn,
            err_mapper: err_fn,
            _phantom: PhantomData,
        }
    }
}

impl<InputOk, InputErr, OutputOk, OutputErr, OkMapperFn, ErrMapperFn, OkMapperFut, ErrMapperFut>
    AsyncResultFlow<InputOk, InputErr>
    for ResultMapBothBindStepAsync<
        InputOk,
        InputErr,
        OutputOk,
        OutputErr,
        OkMapperFn,
        ErrMapperFn,
        OkMapperFut,
        ErrMapperFut,
    >
where
    InputOk: Send + Sync,
    InputErr: Send + Sync,
    OutputOk: Send + Sync,
    OutputErr: Send + Sync,
    OkMapperFn: Fn(InputOk) -> OkMapperFut + Send + Sync,
    ErrMapperFn: Fn(InputErr) -> ErrMapperFut + Send + Sync,
    OkMapperFut: Future<Output = Result<OutputOk, OutputErr>> + Send,
    ErrMapperFut: Future<Output = Result<OutputOk, OutputErr>> + Send,
{
    type OutputOk = OutputOk;
    type OutputErr = OutputErr;

    fn apply_result_async<'a>(
        &'a self,
        input_result: Result<InputOk, InputErr>,
    ) -> impl Future<Output = Result<Self::OutputOk, Self::OutputErr>> + Send + 'a {
        async {
            match input_result {
                Ok(ok) => (self.ok_mapper)(ok).await,
                Err(err) => (self.err_mapper)(err).await,
            }
        }
    }
}
