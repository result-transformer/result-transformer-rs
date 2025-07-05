use std::marker::PhantomData;

use crate::sync::flow::ErrFlow;

pub struct ErrStep<InputErr, OutputErr>
{
    _phantom: PhantomData<(InputErr, OutputErr)>,
}

impl<InputErr, OutputErr> ErrStep<InputErr, OutputErr>
{
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<InputErr, OutputErr> ErrFlow<InputErr> for ErrStep<InputErr, OutputErr>
{
    type OutputErr = OutputErr;

    fn apply_err(&self, input_err: InputErr) -> Self::OutputErr {
        
    }
}
