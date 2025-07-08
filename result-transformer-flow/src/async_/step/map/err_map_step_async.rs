use result_transformer_dependencies::*;

use std::{marker::PhantomData, pin::Pin};

use crate::async_::AsyncErrFlow;

/// Step that maps the error value using a provided function.
#[derive(Debug, Clone, Copy)]
pub struct ErrMapStepAsync<MapperFn, InputErr, OutputErr>
where
    MapperFn: Fn(InputErr)  -> Pin<Box<dyn Future<Output = OutputErr> + Send + Sync>> + Send + Sync,
    InputErr: Send + Sync,
    OutputErr: Send + Sync,
{
    mapper: MapperFn,
    _phantom: PhantomData<(InputErr, OutputErr)>,
}

impl<MapperFn, InputErr, OutputErr> ErrMapStepAsync<MapperFn, InputErr, OutputErr>
where
    MapperFn: Fn(InputErr)  -> Pin<Box<dyn Future<Output = OutputErr> + Send + Sync>> + Send + Sync,
    InputErr: Send + Sync,
    OutputErr: Send + Sync,
{
    /// Creates a new [`ErrMapStepAsync`].
    ///
    /// * `mapper` - function converting the error value to another type
    pub fn new(mapper: MapperFn) -> Self {
        Self {
            mapper,
            _phantom: PhantomData,
        }
    }
}

#[async_trait::async_trait]
impl<MapperFn, InputErr, OutputErr> AsyncErrFlow<InputErr> for ErrMapStepAsync<MapperFn, InputErr, OutputErr>
where
    MapperFn: Fn(InputErr)  -> Pin<Box<dyn Future<Output = OutputErr> + Send + Sync>> + Send + Sync,
    InputErr: Send + Sync,
    OutputErr: Send + Sync,
{
    type OutputErr = OutputErr;

    async fn apply_err(&self, input_err: InputErr) -> Self::OutputErr {
        (self.mapper)(input_err).await
    }
}
