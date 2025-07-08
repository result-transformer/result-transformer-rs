//! Async implementation of result if step.

use result_transformer_dependencies::*;

use crate::{__internal::shared_step::ResultIfStep, async_::AsyncResultFlow, sync::ResultFlow};

#[async_trait::async_trait]
impl<InputOk, InputErr, OutputOk, OutputErr, ConditionFn, ThenFlow, ElseFlow>
    AsyncResultFlow<InputOk, InputErr>
    for ResultIfStep<InputOk, InputErr, OutputOk, OutputErr, ConditionFn, ThenFlow, ElseFlow>
where
    ConditionFn: Fn(&Result<InputOk, InputErr>) -> bool + Send + Sync,
    ThenFlow:
        ResultFlow<InputOk, InputErr, OutputOk = OutputOk, OutputErr = OutputErr> + Send + Sync,
    ElseFlow:
        ResultFlow<InputOk, InputErr, OutputOk = OutputOk, OutputErr = OutputErr> + Send + Sync,
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
