//! Async implementation of err if step.

use crate::{
    async_::AsyncErrFlow,
    sync::{flow::ErrFlow, step::ErrIfStep},
};

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

    fn apply_err_async<'a>(
        &'a self,
        input_err: InputErr,
    ) -> impl Future<Output = Self::OutputErr> + Send + 'a {
        async { self.apply(input_err) }
    }
}
