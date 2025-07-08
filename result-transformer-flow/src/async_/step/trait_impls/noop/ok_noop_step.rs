//! Async implementation of ok noop step.

use result_transformer_dependencies::*;

use crate::__internal::shared_step::OkNoopStep;
use crate::async_::AsyncOkFlow;

#[async_trait::async_trait]
impl<OkType> AsyncOkFlow<OkType> for OkNoopStep<OkType>
where
    OkType: Send + Sync,
{
    type OutputOk = OkType;

    async fn apply_ok(&self, input_ok: OkType) -> Self::OutputOk {
        self.apply(input_ok)
    }
}
