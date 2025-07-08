use result_transformer_dependencies::*;

use std::{marker::PhantomData, pin::Pin};

use crate::async_::AsyncResultFlow;

/// Step that maps success and error values using separate functions.
#[derive(Debug, Clone, Copy)]
pub struct ResultMapBothStepAsync<OkMapperFn, ErrMapperFn, InputOk, InputErr, OutputOk, OutputErr>
where
    OkMapperFn: Fn(InputOk) -> Pin<Box<dyn Future<Output = OutputOk> + Send + Sync>> + Send + Sync,
    ErrMapperFn:
        Fn(InputErr) -> Pin<Box<dyn Future<Output = OutputErr> + Send + Sync>> + Send + Sync,
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
    ResultMapBothStepAsync<OkMapperFn, ErrMapperFn, InputOk, InputErr, OutputOk, OutputErr>
where
    OkMapperFn: Fn(InputOk) -> Pin<Box<dyn Future<Output = OutputOk> + Send + Sync>> + Send + Sync,
    ErrMapperFn:
        Fn(InputErr) -> Pin<Box<dyn Future<Output = OutputErr> + Send + Sync>> + Send + Sync,
    InputOk: Send + Sync,
    InputErr: Send + Sync,
    OutputOk: Send + Sync,
    OutputErr: Send + Sync,
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

#[async_trait::async_trait]
impl<OkMapperFn, ErrMapperFn, InputOk, InputErr, OutputOk, OutputErr>
    AsyncResultFlow<InputOk, InputErr>
    for ResultMapBothStepAsync<OkMapperFn, ErrMapperFn, InputOk, InputErr, OutputOk, OutputErr>
where
    OkMapperFn: Fn(InputOk) -> Pin<Box<dyn Future<Output = OutputOk> + Send + Sync>> + Send + Sync,
    ErrMapperFn:
        Fn(InputErr) -> Pin<Box<dyn Future<Output = OutputErr> + Send + Sync>> + Send + Sync,
    InputOk: Send + Sync,
    InputErr: Send + Sync,
    OutputOk: Send + Sync,
    OutputErr: Send + Sync,
{
    type OutputOk = OutputOk;
    type OutputErr = OutputErr;

    async fn apply_result(
        &self,
        input_result: Result<InputOk, InputErr>,
    ) -> Result<Self::OutputOk, Self::OutputErr> {
        match input_result {
            Ok(ok) => Ok((self.ok_mapper)(ok).await),
            Err(err) => Err((self.err_mapper)(err).await),
        }
    }
}
