use std::{marker::PhantomData, pin::Pin};

use crate::async_::AsyncResultFlow;

/// Step that maps the entire `Result` using a single function.
#[derive(Debug, Clone, Copy)]
pub struct ResultMapStepAsync<MapperFn, InputOk, InputErr, OutputOk, OutputErr>
where
    MapperFn: Fn(
            Result<InputOk, InputErr>,
        ) -> Pin<Box<dyn Future<Output = Result<OutputOk, OutputErr>> + Send + Sync>>
        + Send
        + Sync,
    InputOk: Send + Sync,
    InputErr: Send + Sync,
    OutputOk: Send + Sync,
    OutputErr: Send + Sync,
{
    mapper: MapperFn,
    _phantom: PhantomData<(InputOk, InputErr, OutputOk, OutputErr)>,
}

impl<MapperFn, InputOk, InputErr, OutputOk, OutputErr>
    ResultMapStepAsync<MapperFn, InputOk, InputErr, OutputOk, OutputErr>
where
    MapperFn: Fn(
            Result<InputOk, InputErr>,
        ) -> Pin<Box<dyn Future<Output = Result<OutputOk, OutputErr>> + Send + Sync>>
        + Send
        + Sync,
    InputOk: Send + Sync,
    InputErr: Send + Sync,
    OutputOk: Send + Sync,
    OutputErr: Send + Sync,
{
    /// Creates a new [`ResultMapStepAsync`].
    ///
    /// * `mapper` - function converting a `Result` into another `Result`
    pub fn new(mapper: MapperFn) -> Self {
        Self {
            mapper,
            _phantom: PhantomData,
        }
    }
}

impl<MapperFn, InputOk, InputErr, OutputOk, OutputErr> AsyncResultFlow<InputOk, InputErr>
    for ResultMapStepAsync<MapperFn, InputOk, InputErr, OutputOk, OutputErr>
where
    MapperFn: Fn(
            Result<InputOk, InputErr>,
        ) -> Pin<Box<dyn Future<Output = Result<OutputOk, OutputErr>> + Send + Sync>>
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
        (self.mapper)(input_result)
    }
}
