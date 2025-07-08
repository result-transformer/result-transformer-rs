//! Async implementation of result map both step.

use result_transformer_dependencies::*;

use crate::__internal::shared_step::ResultMapBothStep;
use crate::async_::AsyncResultFlow;

#[async_trait::async_trait]
impl<OkMapperFn, ErrMapperFn, InputOk, InputErr, OutputOk, OutputErr>
    AsyncResultFlow<InputOk, InputErr>
    for ResultMapBothStep<OkMapperFn, ErrMapperFn, InputOk, InputErr, OutputOk, OutputErr>
where
    OkMapperFn: Fn(InputOk) -> OutputOk + Send + Sync,
    ErrMapperFn: Fn(InputErr) -> OutputErr + Send + Sync,
    InputOk: Send + Sync,
    InputErr: Send + Sync,
    OutputOk: Send + Sync,
    OutputErr: Send + Sync,
{
    type OutputOk = OutputOk;
    type OutputErr = OutputErr;

    async fn apply_result(
        &self,
        input_result: Result<InputOk, InputErr>,
    ) -> Result<Self::OutputOk, Self::OutputErr> {
        self.apply(input_result)
    }
}
