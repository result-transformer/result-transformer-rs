//! Async implementation of result map step.

use crate::async_::AsyncResultFlow;
use result_transformer_dependencies::*;

use crate::sync::{flow::*, step::*};

#[async_trait::async_trait]
impl<MapperFn, InputOk, InputErr, OutputOk, OutputErr> AsyncResultFlow<InputOk, InputErr>
    for ResultMapStep<MapperFn, InputOk, InputErr, OutputOk, OutputErr>
where
    MapperFn: Fn(Result<InputOk, InputErr>) -> Result<OutputOk, OutputErr> + Send + Sync,
    InputOk: Send + Sync,
    InputErr: Send + Sync,
    OutputOk: Send + Sync,
    OutputErr: Send + Sync,
{
    type OutputOk = OutputOk;
    type OutputErr = OutputErr;

    async fn apply_result(
        &self,
        input_result: Result<InputOk, InputErr>,
    ) -> Result<Self::OutputOk, Self::OutputErr> {
        ResultFlow::apply_result(self, input_result)
    }
}
