//! Async implementation of ok noop step.

use crate::async_::AsyncOkFlow;
use result_transformer_dependencies::*;

use crate::sync::{flow::*, step::*};

#[async_trait::async_trait]
impl<OkType> AsyncOkFlow<OkType> for OkNoopStep<OkType>
where
    OkType: Send + Sync,
{
    type OutputOk = OkType;

    async fn apply_ok(&self, input_ok: OkType) -> Self::OutputOk {
        OkFlow::apply_ok(self, input_ok)
    }
}
