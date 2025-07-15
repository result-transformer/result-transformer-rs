use std::marker::PhantomData;

/// Step that maps the success value using a provided function.
#[derive(Debug, Clone, Copy)]
pub struct OkMapStep<InputOk, OutputOk, MapperFn>
where
    MapperFn: Fn(InputOk) -> OutputOk,
{
    mapper: MapperFn,
    _phantom: PhantomData<(InputOk, OutputOk)>,
}

impl<InputOk, OutputOk, MapperFn> OkMapStep<InputOk, OutputOk, MapperFn>
where
    MapperFn: Fn(InputOk) -> OutputOk,
{
    /// Creates a new [`OkMapStep`].
    ///
    /// * `mapper` - function converting the success value to another type
    pub fn new(mapper: MapperFn) -> Self {
        Self {
            mapper,
            _phantom: PhantomData,
        }
    }

    pub(crate) fn apply(&self, input_ok: InputOk) -> OutputOk {
        (self.mapper)(input_ok)
    }
}
