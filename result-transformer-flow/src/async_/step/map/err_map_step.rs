//! Async implementation of err map step.

use crate::async_::AsyncErrFlow;
use result_transformer_dependencies::*;

use crate::sync::{flow::*, step::*};

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
        ErrFlow::apply_err(self, input_err)
    }
}
