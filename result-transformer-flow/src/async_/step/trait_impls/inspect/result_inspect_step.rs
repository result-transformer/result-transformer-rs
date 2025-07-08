//! Async implementation of result inspect step.

use result_transformer_dependencies::*;

use crate::__internal::shared_step::ResultInspectStep;
use crate::async_::AsyncResultFlow;

#[async_trait::async_trait]
impl<InspectorFn, InputOk, InputErr> AsyncResultFlow<InputOk, InputErr>
    for ResultInspectStep<InspectorFn, InputOk, InputErr>
where
    InspectorFn: Fn(&Result<InputOk, InputErr>) + Send + Sync,
    InputOk: Send + Sync,
    InputErr: Send + Sync,
{
    type OutputOk = InputOk;
    type OutputErr = InputErr;

    async fn apply_result(
        &self,
        input_result: Result<InputOk, InputErr>,
    ) -> Result<Self::OutputOk, Self::OutputErr> {
        self.apply(input_result)
    }
}
