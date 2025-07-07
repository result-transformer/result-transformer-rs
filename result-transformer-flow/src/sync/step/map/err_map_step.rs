use std::marker::PhantomData;

use crate::sync::flow::ErrFlow;

/// Step that maps the error value using a provided function.
pub struct ErrMapStep<MapperFn, InputErr, OutputErr>
where
    MapperFn: Fn(InputErr) -> OutputErr,
{
    mapper: MapperFn,
    _phantom: PhantomData<(InputErr, OutputErr)>,
}

impl<MapperFn, InputErr, OutputErr> ErrMapStep<MapperFn, InputErr, OutputErr>
where
    MapperFn: Fn(InputErr) -> OutputErr,
{
    /// Creates a new [`ErrMapStep`].
    ///
    /// * `mapper` - function converting the error value to another type
    pub fn new(mapper: MapperFn) -> Self {
        Self {
            mapper,
            _phantom: PhantomData,
        }
    }
}

impl<MapperFn, InputErr, OutputErr> ErrFlow<InputErr> for ErrMapStep<MapperFn, InputErr, OutputErr>
where
    MapperFn: Fn(InputErr) -> OutputErr,
{
    type OutputErr = OutputErr;

    /// Implementation of [`ErrFlow::apply_err`].
    fn apply_err(&self, input_err: InputErr) -> Self::OutputErr {
        (self.mapper)(input_err)
    }
}
