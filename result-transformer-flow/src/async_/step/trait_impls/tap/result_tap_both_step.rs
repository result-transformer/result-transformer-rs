//! Async implementation of result tap both step.

use crate::__internal::shared_step::ResultTapBothStep;
use crate::async_::AsyncResultFlow;

impl<InputOk, InputErr, OutputOk, OutputErr, OkTapFn, ErrTapFn> AsyncResultFlow<InputOk, InputErr>
    for ResultTapBothStep<InputOk, InputErr, OutputOk, OutputErr, OkTapFn, ErrTapFn>
where
    InputOk: Send + Sync,
    InputErr: Send + Sync,
    OutputOk: Send + Sync,
    OutputErr: Send + Sync,
    OkTapFn: Fn(InputOk) -> OutputOk + Send + Sync,
    ErrTapFn: Fn(InputErr) -> OutputErr + Send + Sync,
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
