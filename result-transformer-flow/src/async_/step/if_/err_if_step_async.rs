use core::future::Future;
use std::marker::PhantomData;

use crate::async_::AsyncErrFlow;

/// Executes one of two [`AsyncErrFlow`]s depending on the condition.
///
/// `InputErr` denotes the input error type, and `OutputErr` the output error type.
#[derive(Debug, Clone, Copy)]
pub struct ErrIfStepAsync<InputErr, OutputErr, ConditionFn, ConditionFut, ThenFlow, ElseFlow>
where
    InputErr: Send + Sync,
    OutputErr: Send + Sync,
    ConditionFn: Fn(&InputErr) -> ConditionFut + Send + Sync,
    ConditionFut: Future<Output = bool> + Send,
    ThenFlow: AsyncErrFlow<InputErr, OutputErr = OutputErr> + Send + Sync,
    ElseFlow: AsyncErrFlow<InputErr, OutputErr = OutputErr> + Send + Sync,
{
    condition: ConditionFn,
    then_flow: ThenFlow,
    else_flow: ElseFlow,
    _phantom: PhantomData<InputErr>,
}

impl<InputErr, OutputErr, ConditionFn, ConditionFut, ThenFlow, ElseFlow>
    ErrIfStepAsync<InputErr, OutputErr, ConditionFn, ConditionFut, ThenFlow, ElseFlow>
where
    InputErr: Send + Sync,
    OutputErr: Send + Sync,
    ConditionFn: Fn(&InputErr) -> ConditionFut + Send + Sync,
    ConditionFut: Future<Output = bool> + Send,
    ThenFlow: AsyncErrFlow<InputErr, OutputErr = OutputErr> + Send + Sync,
    ElseFlow: AsyncErrFlow<InputErr, OutputErr = OutputErr> + Send + Sync,
{
    /// Creates a new [`ErrIfStepAsync`].
    ///
    /// * `condition` - predicate that evaluates the error value
    /// * `then_flow` - flow executed when the predicate is `true`
    /// * `else_flow` - flow executed when the predicate is `false`
    pub const fn new(condition: ConditionFn, then_flow: ThenFlow, else_flow: ElseFlow) -> Self {
        Self {
            condition,
            then_flow,
            else_flow,
            _phantom: PhantomData,
        }
    }
}

impl<InputErr, OutputErr, ConditionFn, ConditionFut, ThenFlow, ElseFlow> AsyncErrFlow<InputErr>
    for ErrIfStepAsync<InputErr, OutputErr, ConditionFn, ConditionFut, ThenFlow, ElseFlow>
where
    InputErr: Send + Sync,
    OutputErr: Send + Sync,
    ConditionFn: Fn(&InputErr) -> ConditionFut + Send + Sync,
    ConditionFut: Future<Output = bool> + Send,
    ThenFlow: AsyncErrFlow<InputErr, OutputErr = OutputErr> + Send + Sync,
    ElseFlow: AsyncErrFlow<InputErr, OutputErr = OutputErr> + Send + Sync,
{
    type OutputErr = OutputErr;

    /// Implementation of [`AsyncErrFlow::apply_err`].
    /// Evaluates the condition and passes the error to the selected flow.
    fn apply_err_async<'a>(
        &'a self,
        input_err: InputErr,
    ) -> impl Future<Output = Self::OutputErr> + Send + 'a {
        async {
            if (self.condition)(&input_err).await {
                self.then_flow.apply_err_async(input_err).await
            } else {
                self.else_flow.apply_err_async(input_err).await
            }
        }
    }
}
