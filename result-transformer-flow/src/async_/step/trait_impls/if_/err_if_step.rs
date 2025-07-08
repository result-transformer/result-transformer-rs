//! Async implementation of err if step.

use result_transformer_dependencies::*;

use crate::{__internal::shared_step::ErrIfStep, async_::AsyncErrFlow, sync::ErrFlow};

#[async_trait::async_trait]
impl<InputErr, OutputErr, ConditionFn, ThenFlow, ElseFlow> AsyncErrFlow<InputErr>
    for ErrIfStep<InputErr, OutputErr, ConditionFn, ThenFlow, ElseFlow>
where
    ConditionFn: Fn(&InputErr) -> bool + Send + Sync,
    ThenFlow: ErrFlow<InputErr, OutputErr = OutputErr> + Send + Sync,
    ElseFlow: ErrFlow<InputErr, OutputErr = OutputErr> + Send + Sync,
    InputErr: Send + Sync,
    OutputErr: Send + Sync,
{
    type OutputErr = OutputErr;

    async fn apply_err(&self, input_err: InputErr) -> Self::OutputErr {
        self.apply(input_err)
    }
}
