//! Async implementation of result map step.

use crate::__internal::shared_step::ResultMapStep;
use crate::async_::AsyncResultFlow;

impl<InputOk, InputErr, OutputOk, OutputErr, MapperFn> AsyncResultFlow<InputOk, InputErr>
    for ResultMapStep<InputOk, InputErr, OutputOk, OutputErr, MapperFn>
where
    InputOk: Send + Sync,
    InputErr: Send + Sync,
    OutputOk: Send + Sync,
    OutputErr: Send + Sync,
    MapperFn: Fn(Result<InputOk, InputErr>) -> Result<OutputOk, OutputErr> + Send + Sync,
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
