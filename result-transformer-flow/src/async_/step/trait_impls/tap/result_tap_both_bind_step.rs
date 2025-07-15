//! Async implementation of result tap both bind step.

use crate::__internal::shared_step::ResultTapBothBindStep;
use crate::async_::AsyncResultFlow;

impl<InputOk, InputErr, OutputOk, OutputErr, OkTapFn, ErrTapFn> AsyncResultFlow<InputOk, InputErr>
    for ResultTapBothBindStep<InputOk, InputErr, OutputOk, OutputErr, OkTapFn, ErrTapFn>
where
    InputOk: Send + Sync,
    InputErr: Send + Sync,
    OutputOk: Send + Sync,
    OutputErr: Send + Sync,
    OkTapFn: Fn(InputOk) -> Result<OutputOk, OutputErr> + Send + Sync,
    ErrTapFn: Fn(InputErr) -> Result<OutputOk, OutputErr> + Send + Sync,
{
    type OutputOk = OutputOk;
    type OutputErr = OutputErr;

    fn apply_result_async<'a>(
        &'a self,
        input_result: Result<InputOk, InputErr>,
    ) -> impl Future<Output = Result<Self::OutputOk, Self::OutputErr>> + Send + 'a {
        async { self.apply(input_result) }
    }
}
