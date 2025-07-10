//! Async implementation of err if step.

use crate::{
    async_::AsyncOkFlow,
    sync::{flow::OkFlow, step::OkIfStep},
};

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

    fn apply_ok_async<'a>(
        &'a self,
        input_ok: InputOk,
    ) -> impl Future<Output = Self::OutputOk> + Send + 'a {
        async { self.apply(input_ok) }
    }
}
