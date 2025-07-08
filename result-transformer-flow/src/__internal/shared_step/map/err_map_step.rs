use std::marker::PhantomData;

/// Step that maps the error value using a provided function.
#[derive(Debug, Clone, Copy)]
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
    
    pub(crate) fn apply(&self, input_err: InputErr) -> OutputErr {
        (self.mapper)(input_err)
    }
}
