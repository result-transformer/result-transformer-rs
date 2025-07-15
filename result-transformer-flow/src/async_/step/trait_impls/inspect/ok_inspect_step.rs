//! Async implementation of ok inspect step.

use crate::__internal::shared_step::OkInspectStep;
use crate::async_::AsyncOkFlow;

impl<OkType, InspectorFn> AsyncOkFlow<OkType> for OkInspectStep<OkType, InspectorFn>
where
    OkType: Send + Sync,
    InspectorFn: Fn(&OkType) + Send + Sync,
{
    type OutputOk = OkType;

    fn apply_ok_async<'a>(
        &'a self,
        input_ok: OkType,
    ) -> impl Future<Output = Self::OutputOk> + Send + 'a {
        async { self.apply(input_ok) }
    }
}
