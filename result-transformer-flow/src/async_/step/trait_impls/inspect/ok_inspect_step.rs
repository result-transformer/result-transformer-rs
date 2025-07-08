//! Async implementation of ok inspect step.

use result_transformer_dependencies::*;

use crate::__internal::shared_step::OkInspectStep;
use crate::async_::AsyncOkFlow;

#[async_trait::async_trait]
impl<InspectorFn, OkType> AsyncOkFlow<OkType> for OkInspectStep<InspectorFn, OkType>
where
    InspectorFn: Fn(&OkType) + Send + Sync,
    OkType: Send + Sync,
{
    type OutputOk = OkType;

    async fn apply_ok(&self, input_ok: OkType) -> Self::OutputOk {
        self.apply(input_ok)
    }
}
