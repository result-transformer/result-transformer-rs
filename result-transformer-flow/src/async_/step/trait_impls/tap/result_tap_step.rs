//! Async implementation of result tap step.

use result_transformer_dependencies::*;

use crate::__internal::shared_step::ResultTapStep;
use crate::async_::AsyncResultFlow;

#[async_trait::async_trait]
impl<TapFn, InputOk, InputErr, OutputOk, OutputErr> AsyncResultFlow<InputOk, InputErr>
    for ResultTapStep<TapFn, InputOk, InputErr, OutputOk, OutputErr>
where
    TapFn: Fn(Result<InputOk, InputErr>) -> Result<OutputOk, OutputErr> + Send + Sync,
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
