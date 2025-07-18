//! Async implementation of result map both bind step.

use crate::__internal::shared_step::ResultMapBothBindStep;
use crate::async_::AsyncResultFlow;

impl<InputOk, InputErr, OutputOk, OutputErr, OkMapperFn, ErrMapperFn>
    AsyncResultFlow<InputOk, InputErr>
    for ResultMapBothBindStep<InputOk, InputErr, OutputOk, OutputErr, OkMapperFn, ErrMapperFn>
where
    InputOk: Send + Sync,
    InputErr: Send + Sync,
    OutputOk: Send + Sync,
    OutputErr: Send + Sync,
    OkMapperFn: Fn(InputOk) -> Result<OutputOk, OutputErr> + Send + Sync,
    ErrMapperFn: Fn(InputErr) -> Result<OutputOk, OutputErr> + Send + Sync,
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
