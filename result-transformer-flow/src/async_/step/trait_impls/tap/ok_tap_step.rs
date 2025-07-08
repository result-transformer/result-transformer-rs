//! Async implementation of ok tap step.

use result_transformer_dependencies::*;

use crate::__internal::shared_step::OkTapStep;
use crate::async_::AsyncOkFlow;

#[async_trait::async_trait]
impl<TapFn, InputOk, OutputOk> AsyncOkFlow<InputOk> for OkTapStep<TapFn, InputOk, OutputOk>
where
    TapFn: Fn(InputOk) -> OutputOk + Send + Sync,
    InputOk: Send + Sync,
    OutputOk: Send + Sync,
{
    type OutputOk = OutputOk;

    async fn apply_ok(&self, input_ok: InputOk) -> Self::OutputOk {
        self.apply(input_ok)
    }
}
