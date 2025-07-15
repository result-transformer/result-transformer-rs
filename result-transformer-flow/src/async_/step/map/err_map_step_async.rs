use core::future::Future;
use std::marker::PhantomData;

use crate::async_::AsyncErrFlow;

/// Step that maps the error value using a provided function.
#[derive(Debug, Clone, Copy)]
pub struct ErrMapStepAsync<InputErr, OutputErr, MapperFn, MapperFut>
where
    InputErr: Send + Sync,
    OutputErr: Send + Sync,
    MapperFn: Fn(InputErr) -> MapperFut + Send + Sync,
    MapperFut: Future<Output = OutputErr> + Send,
{
    mapper: MapperFn,
    _phantom: PhantomData<InputErr>,
}

impl<InputErr, OutputErr, MapperFn, MapperFut>
    ErrMapStepAsync<InputErr, OutputErr, MapperFn, MapperFut>
where
    InputErr: Send + Sync,
    OutputErr: Send + Sync,
    MapperFn: Fn(InputErr) -> MapperFut + Send + Sync,
    MapperFut: Future<Output = OutputErr> + Send,
{
    /// Creates a new [`ErrMapStepAsync`].
    ///
    /// * `mapper` - function converting the error value to another type
    pub const fn new(mapper: MapperFn) -> Self {
        Self {
            mapper,
            _phantom: PhantomData,
        }
    }
}

impl<InputErr, OutputErr, MapperFn, MapperFut> AsyncErrFlow<InputErr>
    for ErrMapStepAsync<InputErr, OutputErr, MapperFn, MapperFut>
where
    InputErr: Send + Sync,
    OutputErr: Send + Sync,
    MapperFn: Fn(InputErr) -> MapperFut + Send + Sync,
    MapperFut: Future<Output = OutputErr> + Send,
{
    type OutputErr = OutputErr;

    fn apply_err_async<'a>(
        &'a self,
        input_err: InputErr,
    ) -> impl Future<Output = Self::OutputErr> + Send + 'a {
        (self.mapper)(input_err)
    }
}
