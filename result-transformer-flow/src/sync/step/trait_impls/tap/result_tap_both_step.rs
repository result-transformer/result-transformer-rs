use crate::sync::flow::ResultFlow;

use crate::__internal::shared_step::ResultTapBothStep;

impl<InputOk, InputErr, OutputOk, OutputErr, OkTapFn, ErrTapFn> ResultFlow<InputOk, InputErr>
    for ResultTapBothStep<InputOk, InputErr, OutputOk, OutputErr, OkTapFn, ErrTapFn>
where
    OkTapFn: Fn(InputOk) -> OutputOk,
    ErrTapFn: Fn(InputErr) -> OutputErr,
{
    type OutputOk = OutputOk;
    type OutputErr = OutputErr;

    /// Implementation of [`ResultFlow::apply_result`].
    fn apply_result(&self, input_result: Result<InputOk, InputErr>) -> Result<OutputOk, OutputErr> {
        self.apply(input_result)
    }
}
