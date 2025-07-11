use core::future::Future;
use std::marker::PhantomData;

use crate::async_::AsyncResultFlow;

/// Step that maps success and error values using separate functions.
#[derive(Debug, Clone, Copy)]
pub struct ResultMapBothStepAsync<
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
    OkMapperFut: Future<Output = OutputOk> + Send,
    ErrMapperFut: Future<Output = OutputErr> + Send,
{
    ok_mapper: OkMapperFn,
    err_mapper: ErrMapperFn,
    _phantom: PhantomData<(InputOk, InputErr)>,
}

impl<InputOk, InputErr, OutputOk, OutputErr, OkMapperFn, ErrMapperFn, OkMapperFut, ErrMapperFut>
    ResultMapBothStepAsync<
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
    OkMapperFut: Future<Output = OutputOk> + Send,
    ErrMapperFut: Future<Output = OutputErr> + Send,
{
    /// Creates a new [`ResultMapBothStepAsync`].
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

impl<InputOk, InputErr, OutputOk, OutputErr, OkMapperFn, ErrMapperFn, OkMapperFut, ErrMapperFut>
    AsyncResultFlow<InputOk, InputErr>
    for ResultMapBothStepAsync<
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
    OkMapperFut: Future<Output = OutputOk> + Send,
    ErrMapperFut: Future<Output = OutputErr> + Send,
{
    type OutputOk = OutputOk;
    type OutputErr = OutputErr;

    fn apply_result_async<'a>(
        &'a self,
        input_result: Result<InputOk, InputErr>,
    ) -> impl Future<Output = Result<Self::OutputOk, Self::OutputErr>> + Send + 'a {
        async {
            match input_result {
                Ok(ok) => Ok((self.ok_mapper)(ok).await),
                Err(err) => Err((self.err_mapper)(err).await),
            }
        }
    }
}
