//! Async implementation of result map both bind step.

use result_transformer_dependencies::*;

use crate::__internal::shared_step::ResultMapBothBindStep;
use crate::async_::AsyncResultFlow;

#[async_trait::async_trait]
impl<OkMapperFn, ErrMapperFn, InputOk, InputErr, OutputOk, OutputErr>
    AsyncResultFlow<InputOk, InputErr>
    for ResultMapBothBindStep<OkMapperFn, ErrMapperFn, InputOk, InputErr, OutputOk, OutputErr>
where
    OkMapperFn: Fn(InputOk) -> Result<OutputOk, OutputErr> + Send + Sync,
    ErrMapperFn: Fn(InputErr) -> Result<OutputOk, OutputErr> + Send + Sync,
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
