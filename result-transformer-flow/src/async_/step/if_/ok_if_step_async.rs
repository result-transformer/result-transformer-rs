use result_transformer_dependencies::*;

use std::{marker::PhantomData, pin::Pin};

use crate::{async_::AsyncOkFlow};

/// Step that executes one of two [`AsyncOkFlow`] branches depending on a condition.
#[derive(Debug, Clone, Copy)]
pub struct OkIfStepAsync<InputOk, OutputOk, ConditionFn, ThenFlow, ElseFlow> {
    condition: ConditionFn,
    then_flow: ThenFlow,
    else_flow: ElseFlow,
    _phantom: PhantomData<(InputOk, OutputOk)>,
}

impl<InputOk, OutputOk, ConditionFn, ThenFlow, ElseFlow>
    OkIfStepAsync<InputOk, OutputOk, ConditionFn, ThenFlow, ElseFlow>
where
    InputOk: Send + Sync,
    OutputOk: Send + Sync,
    ConditionFn: Fn(&InputOk) -> Pin<Box<dyn Future<Output = bool> + Send + Sync>> + Send + Sync,
    ThenFlow: AsyncOkFlow<InputOk, OutputOk = OutputOk> + Send + Sync,
    ElseFlow: AsyncOkFlow<InputOk, OutputOk = OutputOk> + Send + Sync,
{
    /// Creates a new [`OkIfStepAsync`].
    ///
    /// * `condition` - predicate that evaluates the successful value
    /// * `then_flow` - flow run when the predicate returns `true`
    /// * `else_flow` - flow run when the predicate returns `false`
    pub fn new(condition: ConditionFn, then_flow: ThenFlow, else_flow: ElseFlow) -> Self {
        Self {
            condition,
            then_flow,
            else_flow,
            _phantom: PhantomData,
        }
    }
}

#[async_trait::async_trait]
impl<InputOk, OutputOk, ConditionFn, ThenFlow, ElseFlow> AsyncOkFlow<InputOk>
    for OkIfStepAsync<InputOk, OutputOk, ConditionFn, ThenFlow, ElseFlow>
where
    InputOk: Send + Sync,
    OutputOk: Send + Sync,
    ConditionFn: Fn(&InputOk) -> Pin<Box<dyn Future<Output = bool> + Send + Sync>> + Send + Sync,
    ThenFlow: AsyncOkFlow<InputOk, OutputOk = OutputOk> + Send + Sync,
    ElseFlow: AsyncOkFlow<InputOk, OutputOk = OutputOk> + Send + Sync,
{
    type OutputOk = OutputOk;

    /// Implementation of [`AsyncOkFlow::apply_ok`].
    /// Evaluates the condition and forwards the value to the chosen flow.
    async fn apply_ok(&self, input_ok: InputOk) -> Self::OutputOk {
        if (self.condition)(&input_ok).await {
            self.then_flow.apply_ok(input_ok).await
        } else {
            self.else_flow.apply_ok(input_ok).await
        }
    }
}
