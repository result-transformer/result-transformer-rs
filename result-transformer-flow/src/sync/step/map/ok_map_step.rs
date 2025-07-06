use std::marker::PhantomData;

use crate::sync::flow::OkFlow;

/// Maps the successful value using the provided function.
pub struct OkMapStep<MapperFn, InputOk, OutputOk>
where
    MapperFn: Fn(InputOk) -> OutputOk,
{
    mapper: MapperFn,
    _phantom: PhantomData<(InputOk, OutputOk)>,
}

impl<MapperFn, InputOk, OutputOk> OkMapStep<MapperFn, InputOk, OutputOk>
where
    MapperFn: Fn(InputOk) -> OutputOk,
{
    pub fn new(mapper: MapperFn) -> Self {
        Self {
            mapper,
            _phantom: PhantomData,
        }
    }
}

impl<MapperFn, InputOk, OutputOk> OkFlow<InputOk> for OkMapStep<MapperFn, InputOk, OutputOk>
where
    MapperFn: Fn(InputOk) -> OutputOk,
{
    type OutputOk = OutputOk;

    fn apply_ok(&self, input_ok: InputOk) -> Self::OutputOk {
        (self.mapper)(input_ok)
    }
}
