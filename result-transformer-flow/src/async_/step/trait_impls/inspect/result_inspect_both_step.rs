//! Async implementation of result inspect both step.

use crate::__internal::shared_step::ResultInspectBothStep;
use crate::async_::AsyncResultFlow;

impl<OkInspectorFn, ErrInspectorFn, OkType, ErrType> AsyncResultFlow<OkType, ErrType>
    for ResultInspectBothStep<OkInspectorFn, ErrInspectorFn, OkType, ErrType>
where
    OkInspectorFn: Fn(&OkType) + Send + Sync,
    ErrInspectorFn: Fn(&ErrType) + Send + Sync,
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
