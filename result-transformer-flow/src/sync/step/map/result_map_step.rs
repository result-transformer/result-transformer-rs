use std::marker::PhantomData;

use crate::sync::flow::ResultFlow;

pub struct ResultMapStep<MapperFn, InputOk, InputErr, OutputOk, OutputErr>
where
    MapperFn: Fn(Result<InputOk, InputErr>) -> Result<OutputOk, OutputErr>,
{
    mapper: MapperFn,
    _phantom: PhantomData<(InputOk, InputErr, OutputOk, OutputErr)>,
}

impl<MapperFn, InputOk, InputErr, OutputOk, OutputErr>
    ResultMapStep<MapperFn, InputOk, InputErr, OutputOk, OutputErr>
where
    MapperFn: Fn(Result<InputOk, InputErr>) -> Result<OutputOk, OutputErr>,
{
    pub fn new(mapper: MapperFn) -> Self {
        Self {
            mapper,
            _phantom: PhantomData,
        }
    }
}

impl<MapperFn, InputOk, InputErr, OutputOk, OutputErr> ResultFlow<InputOk, InputErr>
    for ResultMapStep<MapperFn, InputOk, InputErr, OutputOk, OutputErr>
where
    MapperFn: Fn(Result<InputOk, InputErr>) -> Result<OutputOk, OutputErr>,
{
    type OutputOk = OutputOk;
    type OutputErr = OutputErr;

    fn apply_result(
        &self,
        input_result: Result<InputOk, InputErr>,
    ) -> Result<Self::OutputOk, Self::OutputErr> {
        (self.mapper)(input_result)
    }
}