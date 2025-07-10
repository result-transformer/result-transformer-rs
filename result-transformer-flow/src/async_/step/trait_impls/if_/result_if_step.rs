//! Async implementation of result if step.

use crate::{
    async_::AsyncResultFlow,
    sync::{flow::ResultFlow, step::ResultIfStep},
};

impl<InputOk, InputErr, OutputOk, OutputErr, ConditionFn, ThenFlow, ElseFlow>
    AsyncResultFlow<InputOk, InputErr>
    for ResultIfStep<InputOk, InputErr, OutputOk, OutputErr, ConditionFn, ThenFlow, ElseFlow>
where
    ConditionFn: Fn(&Result<InputOk, InputErr>) -> bool + Send + Sync,
    ThenFlow:
        ResultFlow<InputOk, InputErr, OutputOk = OutputOk, OutputErr = OutputErr> + Send + Sync,
    ElseFlow:
        ResultFlow<InputOk, InputErr, OutputOk = OutputOk, OutputErr = OutputErr> + Send + Sync,
    InputOk: Send + Sync,
    InputErr: Send + Sync,
    OutputOk: Send + Sync,
    OutputErr: Send + Sync,
{
    type OutputOk = OutputOk;
    type OutputErr = OutputErr;

    fn apply_result_async<'a>(
        &'a self,
        input_result: Result<InputOk, InputErr>,
    ) -> impl Future<Output = Result<Self::OutputOk, Self::OutputErr>> + Send + 'a {
        async { self.apply(input_result) }
    }
}
