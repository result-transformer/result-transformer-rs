use std::marker::PhantomData;

use crate::sync::flow::ResultFlow;

/// Applies a closure to the entire [`Result`] and returns its outcome.
pub struct ResultTapStep<TapFn, InputOk, InputErr, OutputOk, OutputErr>
where
    TapFn: Fn(Result<InputOk, InputErr>) -> Result<OutputOk, OutputErr>,
{
    tap: TapFn,
    _phantom: PhantomData<(InputOk, InputErr, OutputOk, OutputErr)>,
}

impl<TapFn, InputOk, InputErr, OutputOk, OutputErr>
    ResultTapStep<TapFn, InputOk, InputErr, OutputOk, OutputErr>
where
    TapFn: Fn(Result<InputOk, InputErr>) -> Result<OutputOk, OutputErr>,
{
    pub fn new(tap: TapFn) -> Self {
        Self {
            tap,
            _phantom: PhantomData,
        }
    }
}

impl<TapFn, InputOk, InputErr, OutputOk, OutputErr> ResultFlow<InputOk, InputErr>
    for ResultTapStep<TapFn, InputOk, InputErr, OutputOk, OutputErr>
where
    TapFn: Fn(Result<InputOk, InputErr>) -> Result<OutputOk, OutputErr>,
{
    type OutputOk = OutputOk;
    type OutputErr = OutputErr;

    fn apply_result(
        &self,
        input_result: Result<InputOk, InputErr>,
    ) -> Result<Self::OutputOk, Self::OutputErr> {
        (self.tap)(input_result)
    }
}
