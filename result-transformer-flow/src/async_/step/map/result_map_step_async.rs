use core::future::Future;
use std::marker::PhantomData;

use crate::async_::AsyncResultFlow;

/// Step that maps the entire `Result` using a single function.
#[derive(Debug, Clone, Copy)]
pub struct ResultMapStepAsync<InputOk, InputErr, OutputOk, OutputErr, MapperFn, MapperFut>
where
    InputOk: Send + Sync,
    InputErr: Send + Sync,
    OutputOk: Send + Sync,
    OutputErr: Send + Sync,
    MapperFn: Fn(Result<InputOk, InputErr>) -> MapperFut + Send + Sync,
    MapperFut: Future<Output = Result<OutputOk, OutputErr>> + Send,
{
    mapper: MapperFn,
    _phantom: PhantomData<(InputOk, InputErr)>,
}

impl<InputOk, InputErr, OutputOk, OutputErr, MapperFn, MapperFut>
    ResultMapStepAsync<InputOk, InputErr, OutputOk, OutputErr, MapperFn, MapperFut>
where
    InputOk: Send + Sync,
    InputErr: Send + Sync,
    OutputOk: Send + Sync,
    OutputErr: Send + Sync,
    MapperFn: Fn(Result<InputOk, InputErr>) -> MapperFut + Send + Sync,
    MapperFut: Future<Output = Result<OutputOk, OutputErr>> + Send,
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

impl<InputOk, InputErr, OutputOk, OutputErr, MapperFn, MapperFut> AsyncResultFlow<InputOk, InputErr>
    for ResultMapStepAsync<InputOk, InputErr, OutputOk, OutputErr, MapperFn, MapperFut>
where
    InputOk: Send + Sync,
    InputErr: Send + Sync,
    OutputOk: Send + Sync,
    OutputErr: Send + Sync,
    MapperFn: Fn(Result<InputOk, InputErr>) -> MapperFut + Send + Sync,
    MapperFut: Future<Output = Result<OutputOk, OutputErr>> + Send,
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
