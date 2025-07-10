//! Async implementation of result inspect step.

use crate::__internal::shared_step::ResultInspectStep;
use crate::async_::AsyncResultFlow;

impl<InspectorFn, OkType, ErrType> AsyncResultFlow<OkType, ErrType>
    for ResultInspectStep<InspectorFn, OkType, ErrType>
where
    InspectorFn: Fn(&Result<OkType, ErrType>) + Send + Sync,
    OkType: Send + Sync,
    ErrType: Send + Sync,
{
    type OutputOk = OkType;
    type OutputErr = ErrType;

    fn apply_result_async<'a>(
        &'a self,
        input_result: Result<OkType, ErrType>,
    ) -> impl Future<Output = Result<Self::OutputOk, Self::OutputErr>> + Send + 'a {
        async { self.apply(input_result) }
    }
}
