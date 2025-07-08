use result_transformer_dependencies::*;

use std::{marker::PhantomData, pin::Pin};

use crate::async_::AsyncOkFlow;

/// Step that maps the success value using a provided function.
#[derive(Debug, Clone, Copy)]
pub struct OkMapStepAsync<MapperFn, InputOk, OutputOk>
where
    MapperFn: Fn(InputOk) -> Pin<Box<dyn Future<Output = OutputOk> + Send + Sync>> + Send + Sync,
    InputOk: Send + Sync,
    OutputOk: Send + Sync,
{
    mapper: MapperFn,
    _phantom: PhantomData<(InputOk, OutputOk)>,
}

impl<MapperFn, InputOk, OutputOk> OkMapStepAsync<MapperFn, InputOk, OutputOk>
where
    MapperFn: Fn(InputOk) -> Pin<Box<dyn Future<Output = OutputOk> + Send + Sync>> + Send + Sync,
    InputOk: Send + Sync,
    OutputOk: Send + Sync,
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

#[async_trait::async_trait]
impl<MapperFn, InputOk, OutputOk> AsyncOkFlow<InputOk> for OkMapStepAsync<MapperFn, InputOk, OutputOk>
where
    MapperFn: Fn(InputOk) -> Pin<Box<dyn Future<Output = OutputOk> + Send + Sync>> + Send + Sync,
    InputOk: Send + Sync,
    OutputOk: Send + Sync,
{
    type OutputOk = OutputOk;

    async fn apply_ok(&self, input_ok: InputOk) -> Self::OutputOk {
        (self.mapper)(input_ok).await
    }
}
