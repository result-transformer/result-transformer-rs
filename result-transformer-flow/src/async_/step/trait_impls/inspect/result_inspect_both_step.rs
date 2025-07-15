//! Async implementation of result inspect both step.

use crate::__internal::shared_step::ResultInspectBothStep;
use crate::async_::AsyncResultFlow;

impl<OkType, ErrType, OkInspectorFn, ErrInspectorFn> AsyncResultFlow<OkType, ErrType>
    for ResultInspectBothStep<OkType, ErrType, OkInspectorFn, ErrInspectorFn>
where
    OkType: Send + Sync,
    ErrType: Send + Sync,
    OkInspectorFn: Fn(&OkType) + Send + Sync,
    ErrInspectorFn: Fn(&ErrType) + Send + Sync,
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
