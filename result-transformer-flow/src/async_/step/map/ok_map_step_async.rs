use core::future::Future;
use std::marker::PhantomData;

use crate::async_::AsyncOkFlow;

/// Step that maps the success value using a provided function.
#[derive(Debug, Clone, Copy)]
pub struct OkMapStepAsync<InputOk, OutputOk, MapperFn, MapperFut>
where
    InputOk: Send + Sync,
    OutputOk: Send + Sync,
    MapperFn: Fn(InputOk) -> MapperFut + Send + Sync,
    MapperFut: Future<Output = OutputOk> + Send,
{
    mapper: MapperFn,
    _phantom: PhantomData<InputOk>,
}

impl<InputOk, OutputOk, MapperFn, MapperFut> OkMapStepAsync<InputOk, OutputOk, MapperFn, MapperFut>
where
    InputOk: Send + Sync,
    OutputOk: Send + Sync,
    MapperFn: Fn(InputOk) -> MapperFut + Send + Sync,
    MapperFut: Future<Output = OutputOk> + Send,
{
    /// Creates a new [`OkMapStepAsync`].
    ///
    /// * `mapper` - function converting the success value to another type
    pub fn new(mapper: MapperFn) -> Self {
        Self {
            mapper,
            _phantom: PhantomData,
        }
    }
}

impl<InputOk, OutputOk, MapperFn, MapperFut> AsyncOkFlow<InputOk>
    for OkMapStepAsync<InputOk, OutputOk, MapperFn, MapperFut>
where
    InputOk: Send + Sync,
    OutputOk: Send + Sync,
    MapperFn: Fn(InputOk) -> MapperFut + Send + Sync,
    MapperFut: Future<Output = OutputOk> + Send,
{
    type OutputOk = OutputOk;

    fn apply_ok_async<'a>(
        &'a self,
        input_ok: InputOk,
    ) -> impl Future<Output = Self::OutputOk> + Send + 'a {
        (self.mapper)(input_ok)
    }
}
