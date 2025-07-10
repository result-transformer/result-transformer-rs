use std::{marker::PhantomData, pin::Pin};

use crate::async_::AsyncResultFlow;

/// Step that applies one of two [`AsyncResultFlow`]s depending on a predicate.
#[derive(Debug, Clone, Copy)]
pub struct ResultIfStepAsync<
    InputOk,
    InputErr,
    OutputOk,
    OutputErr,
    ConditionFn,
    ThenFlow,
    ElseFlow,
> {
    condition: ConditionFn,
    then_flow: ThenFlow,
    else_flow: ElseFlow,
    _phantom: PhantomData<(InputOk, InputErr, OutputOk, OutputErr)>,
}

impl<InputOk, InputErr, OutputOk, OutputErr, ConditionFn, ThenFlow, ElseFlow>
    ResultIfStepAsync<InputOk, InputErr, OutputOk, OutputErr, ConditionFn, ThenFlow, ElseFlow>
where
    InputOk: Send + Sync,
    InputErr: Send + Sync,
    OutputOk: Send + Sync,
    OutputErr: Send + Sync,
    ConditionFn: Fn(&Result<InputOk, InputErr>) -> Pin<Box<dyn Future<Output = bool> + Send + Sync>>
        + Send
        + Sync,
    ThenFlow: AsyncResultFlow<InputOk, InputErr, OutputOk = OutputOk, OutputErr = OutputErr>
        + Send
        + Sync,
    ElseFlow: AsyncResultFlow<InputOk, InputErr, OutputOk = OutputOk, OutputErr = OutputErr>
        + Send
        + Sync,
{
    /// Creates a new [`ResultIfStepAsync`].
    ///
    /// * `condition` - function that inspects the entire `Result`
    /// * `then_flow` - flow executed when the predicate returns `true`
    /// * `else_flow` - flow executed when the predicate returns `false`
    pub fn new(condition: ConditionFn, then_flow: ThenFlow, else_flow: ElseFlow) -> Self {
        Self {
            condition,
            then_flow,
            else_flow,
            _phantom: PhantomData,
        }
    }
}

impl<InputOk, InputErr, OutputOk, OutputErr, ConditionFn, ThenFlow, ElseFlow>
    AsyncResultFlow<InputOk, InputErr>
    for ResultIfStepAsync<InputOk, InputErr, OutputOk, OutputErr, ConditionFn, ThenFlow, ElseFlow>
where
    InputOk: Send + Sync,
    InputErr: Send + Sync,
    OutputOk: Send + Sync,
    OutputErr: Send + Sync,
    ConditionFn: Fn(&Result<InputOk, InputErr>) -> Pin<Box<dyn Future<Output = bool> + Send + Sync>>
        + Send
        + Sync,
    ThenFlow: AsyncResultFlow<InputOk, InputErr, OutputOk = OutputOk, OutputErr = OutputErr>
        + Send
        + Sync,
    ElseFlow: AsyncResultFlow<InputOk, InputErr, OutputOk = OutputOk, OutputErr = OutputErr>
        + Send
        + Sync,
{
    type OutputOk = OutputOk;
    type OutputErr = OutputErr;

    /// Implementation of [`AsyncResultFlow::apply_result`].
    ///
    /// Delegates processing to `then_flow` or `else_flow` based on the predicate.
    fn apply_result_async<'a>(
        &'a self,
        input_result: Result<InputOk, InputErr>,
    ) -> impl Future<Output = Result<Self::OutputOk, Self::OutputErr>> + Send + 'a {
        async {
            if (self.condition)(&input_result).await {
                self.then_flow.apply_result_async(input_result).await
            } else {
                self.else_flow.apply_result_async(input_result).await
            }
        }
    }
}
