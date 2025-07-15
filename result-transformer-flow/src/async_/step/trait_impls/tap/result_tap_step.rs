//! Async implementation of result tap step.

use crate::__internal::shared_step::ResultTapStep;
use crate::async_::AsyncResultFlow;

impl<InputOk, InputErr, OutputOk, OutputErr, TapFn> AsyncResultFlow<InputOk, InputErr>
    for ResultTapStep<InputOk, InputErr, OutputOk, OutputErr, TapFn>
where
    InputOk: Send + Sync,
    InputErr: Send + Sync,
    OutputOk: Send + Sync,
    OutputErr: Send + Sync,
    TapFn: Fn(Result<InputOk, InputErr>) -> Result<OutputOk, OutputErr> + Send + Sync,
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
