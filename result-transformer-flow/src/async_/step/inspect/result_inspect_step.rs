//! Async implementation of result inspect step.

use crate::async_::AsyncResultFlow;
use result_transformer_dependencies::*;

use crate::sync::{flow::*, step::*};

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
        ResultFlow::apply_result(self, input_result)
    }
}
