//! Async implementation of result tap both bind step.

use result_transformer_dependencies::*;

use crate::__internal::shared_step::ResultTapBothBindStep;
use crate::async_::AsyncResultFlow;

#[async_trait::async_trait]
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

    async fn apply_result(
        &self,
        input_result: Result<InputOk, InputErr>,
    ) -> Result<Self::OutputOk, Self::OutputErr> {
        self.apply(input_result)
    }
}
