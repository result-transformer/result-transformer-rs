//! Async implementation of [`OkMapStep`].

use result_transformer_dependencies::*;

use crate::__internal::shared_step::OkMapStep;
use crate::async_::AsyncOkFlow;

#[async_trait::async_trait]
impl<MapperFn, InputOk, OutputOk> AsyncOkFlow<InputOk> for OkMapStep<MapperFn, InputOk, OutputOk>
where
    MapperFn: Fn(InputOk) -> OutputOk + Send + Sync,
    InputOk: Send + Sync,
    OutputOk: Send + Sync,
{
    type OutputOk = OutputOk;

    async fn apply_ok(&self, input_ok: InputOk) -> Self::OutputOk {
        self.apply(input_ok)
    }
}
