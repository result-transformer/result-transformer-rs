//! Async implementation of result map both step.

use crate::__internal::shared_step::ResultMapBothStep;
use crate::async_::AsyncResultFlow;

impl<InputOk, InputErr, OutputOk, OutputErr, OkMapperFn, ErrMapperFn>
    AsyncResultFlow<InputOk, InputErr>
    for ResultMapBothStep<InputOk, InputErr, OutputOk, OutputErr, OkMapperFn, ErrMapperFn>
where
    InputOk: Send + Sync,
    InputErr: Send + Sync,
    OutputOk: Send + Sync,
    OutputErr: Send + Sync,
    OkMapperFn: Fn(InputOk) -> OutputOk + Send + Sync,
    ErrMapperFn: Fn(InputErr) -> OutputErr + Send + Sync,
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
