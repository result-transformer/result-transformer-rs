use result_transformer_dependencies::*;

use std::{marker::PhantomData, pin::Pin};

use crate::async_::AsyncErrFlow;

/// Executes one of two [`AsyncErrFlow`]s depending on the condition.
///
/// `InputErr` denotes the input error type, and `OutputErr` the output error type.
#[derive(Debug, Clone, Copy)]
pub struct ErrIfStepAsync<InputErr, OutputErr, ConditionFn, ThenFlow, ElseFlow> {
    condition: ConditionFn,
    then_flow: ThenFlow,
    else_flow: ElseFlow,
    _phantom: PhantomData<(InputErr, OutputErr)>,
}

impl<InputErr, OutputErr, ConditionFn, ThenFlow, ElseFlow>
    ErrIfStepAsync<InputErr, OutputErr, ConditionFn, ThenFlow, ElseFlow>
where
    InputErr: Send + Sync,
    OutputErr: Send + Sync,
    ConditionFn: Fn(&InputErr) -> Pin<Box<dyn Future<Output = bool> + Send + Sync>> + Send + Sync,
    ThenFlow: AsyncErrFlow<InputErr, OutputErr = OutputErr> + Send + Sync,
    ElseFlow: AsyncErrFlow<InputErr, OutputErr = OutputErr> + Send + Sync,
{
    /// Creates a new [`ErrIfStepAsync`].
    ///
    /// * `condition` - predicate that evaluates the error value
    /// * `then_flow` - flow executed when the predicate is `true`
    /// * `else_flow` - flow executed when the predicate is `false`
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
impl<InputErr, OutputErr, ConditionFn, ThenFlow, ElseFlow> AsyncErrFlow<InputErr>
    for ErrIfStepAsync<InputErr, OutputErr, ConditionFn, ThenFlow, ElseFlow>
where
    InputErr: Send + Sync,
    OutputErr: Send + Sync,
    ConditionFn: Fn(&InputErr) -> Pin<Box<dyn Future<Output = bool> + Send + Sync>> + Send + Sync,
    ThenFlow: AsyncErrFlow<InputErr, OutputErr = OutputErr> + Send + Sync,
    ElseFlow: AsyncErrFlow<InputErr, OutputErr = OutputErr> + Send + Sync,
{
    type OutputErr = OutputErr;

    /// Implementation of [`AsyncErrFlow::apply_err`].
    /// Evaluates the condition and passes the error to the selected flow.
    async fn apply_err(&self, input_err: InputErr) -> Self::OutputErr {
        if (self.condition)(&input_err).await {
            self.then_flow.apply_err(input_err).await
        } else {
            self.else_flow.apply_err(input_err).await
        }
    }
}
