//! Async implementation of result tap both bind step.

use crate::__internal::shared_step::ResultTapBothBindStep;
use crate::async_::AsyncResultFlow;

impl<OkTapFn, ErrTapFn, InputOk, InputErr, OutputOk, OutputErr> AsyncResultFlow<InputOk, InputErr>
    for ResultTapBothBindStep<OkTapFn, ErrTapFn, InputOk, InputErr, OutputOk, OutputErr>
where
    OkTapFn: Fn(InputOk) -> Result<OutputOk, OutputErr> + Send + Sync,
    ErrTapFn: Fn(InputErr) -> Result<OutputOk, OutputErr> + Send + Sync,
    InputOk: Send + Sync,
    InputErr: Send + Sync,
    OutputOk: Send + Sync,
    OutputErr: Send + Sync,
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
