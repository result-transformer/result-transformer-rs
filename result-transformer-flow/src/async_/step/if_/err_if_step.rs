//! Async implementation of err if step.

use crate::async_::AsyncErrFlow;
use result_transformer_dependencies::*;

use crate::sync::{flow::*, step::*};

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
        ErrFlow::apply_err(self, input_err)
    }
}
