use std::marker::PhantomData;

use crate::sync::flow::OkFlow;

pub struct OkStep<InputOk, OutputOk>
{
    _phantom: PhantomData<(InputOk, OutputOk)>,
}

impl<InputOk, OutputOk> OkStep<InputOk, OutputOk>
{
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<InputOk, OutputOk> OkFlow<InputOk> for OkStep<InputOk, OutputOk>
{
    type OutputOk = OutputOk;

    fn apply_ok(&self, input_ok: InputOk) -> Self::OutputOk {

    }
}
