//! Async implementation of err map step.

use result_transformer_dependencies::*;

use crate::__internal::shared_step::ErrMapStep;
use crate::async_::AsyncErrFlow;

#[async_trait::async_trait]
impl<MapperFn, InputErr, OutputErr> AsyncErrFlow<InputErr>
    for ErrMapStep<MapperFn, InputErr, OutputErr>
where
    MapperFn: Fn(InputErr) -> OutputErr + Send + Sync,
    InputErr: Send + Sync,
    OutputErr: Send + Sync,
{
    type OutputErr = OutputErr;

    async fn apply_err(&self, input_err: InputErr) -> Self::OutputErr {
        self.apply(input_err)
    }
}
