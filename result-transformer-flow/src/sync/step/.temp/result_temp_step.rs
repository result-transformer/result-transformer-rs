use std::marker::PhantomData;

use crate::sync::flow::ResultFlow;

pub struct ResultStep<InputOk, InputErr, OutputOk, OutputErr> {
    _phantom: PhantomData<(InputOk, InputErr, OutputOk, OutputErr)>,
}

impl<InputOk, InputErr, OutputOk, OutputErr> ResultStep<InputOk, InputErr, OutputOk, OutputErr> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<InputOk, InputErr, OutputOk, OutputErr> ResultFlow<InputOk, InputErr>
    for ResultStep<InputOk, InputErr, OutputOk, OutputErr>
{
    type OutputOk = OutputOk;
    type OutputErr = OutputErr;

    fn apply_result(
        &self,
        input_result: Result<InputOk, InputErr>,
    ) -> Result<Self::OutputOk, Self::OutputErr> {
    }
}
