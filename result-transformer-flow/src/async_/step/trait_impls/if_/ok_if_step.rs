//! Async implementation of err if step.

use result_transformer_dependencies::*;

use crate::{__internal::shared_step::OkIfStep, async_::AsyncOkFlow, sync::OkFlow};

#[async_trait::async_trait]
impl<InputOk, OutputOk, ConditionFn, ThenFlow, ElseFlow> AsyncOkFlow<InputOk>
    for OkIfStep<InputOk, OutputOk, ConditionFn, ThenFlow, ElseFlow>
where
    ConditionFn: Fn(&InputOk) -> bool + Send + Sync,
    ThenFlow: OkFlow<InputOk, OutputOk = OutputOk> + Send + Sync,
    ElseFlow: OkFlow<InputOk, OutputOk = OutputOk> + Send + Sync,
    InputOk: Send + Sync,
    OutputOk: Send + Sync,
{
    type OutputOk = OutputOk;

    async fn apply_ok(&self, input_ok: InputOk) -> Self::OutputOk {
        self.apply(input_ok)
    }
}
