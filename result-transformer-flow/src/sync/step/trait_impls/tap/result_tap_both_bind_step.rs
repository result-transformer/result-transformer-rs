use crate::sync::flow::ResultFlow;

use crate::__internal::shared_step::ResultTapBothBindStep;

impl<OkTapFn, ErrTapFn, InputOk, InputErr, OutputOk, OutputErr> ResultFlow<InputOk, InputErr>
    for ResultTapBothBindStep<OkTapFn, ErrTapFn, InputOk, InputErr, OutputOk, OutputErr>
where
    OkTapFn: Fn(InputOk) -> Result<OutputOk, OutputErr>,
    ErrTapFn: Fn(InputErr) -> Result<OutputOk, OutputErr>,
{
    type OutputOk = OutputOk;
    type OutputErr = OutputErr;

    /// Implementation of [`ResultFlow::apply_result`].
    fn apply_result(&self, input_result: Result<InputOk, InputErr>) -> Result<OutputOk, OutputErr> {
        self.apply(input_result)
    }
}
