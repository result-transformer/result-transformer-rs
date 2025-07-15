use std::marker::PhantomData;

/// Step that maps the entire `Result` using a single function.
#[derive(Debug, Clone, Copy)]
pub struct ResultMapStep<InputOk, InputErr, OutputOk, OutputErr, MapperFn>
where
    MapperFn: Fn(Result<InputOk, InputErr>) -> Result<OutputOk, OutputErr>,
{
    mapper: MapperFn,
    _phantom: PhantomData<(InputOk, InputErr, OutputOk, OutputErr)>,
}

impl<InputOk, InputErr, OutputOk, OutputErr, MapperFn>
    ResultMapStep<InputOk, InputErr, OutputOk, OutputErr, MapperFn>
where
    MapperFn: Fn(Result<InputOk, InputErr>) -> Result<OutputOk, OutputErr>,
{
    /// Creates a new [`ResultMapStep`].
    ///
    /// * `mapper` - function converting a `Result` into another `Result`
    pub const fn new(mapper: MapperFn) -> Self {
        Self {
            mapper,
            _phantom: PhantomData,
        }
    }

    pub(crate) fn apply(
        &self,
        input_result: Result<InputOk, InputErr>,
    ) -> Result<OutputOk, OutputErr> {
        (self.mapper)(input_result)
    }
}
