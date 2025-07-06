use std::marker::PhantomData;

use crate::sync::flow::ErrFlow;

/// Maps the error value using the provided function.
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

    fn apply_err(&self, input_err: InputErr) -> Self::OutputErr {
        (self.mapper)(input_err)
    }
}
