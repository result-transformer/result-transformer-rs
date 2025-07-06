//! Async implementation of ok if step.

use crate::async_::AsyncOkFlow;
use result_transformer_dependencies::*;

use crate::sync::{flow::*, step::*};

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
        OkFlow::apply_ok(self, input_ok)
    }
}
