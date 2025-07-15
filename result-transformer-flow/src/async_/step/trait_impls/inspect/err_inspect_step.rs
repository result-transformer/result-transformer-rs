//! Async implementation of err inspect step.

use crate::__internal::shared_step::ErrInspectStep;
use crate::async_::AsyncErrFlow;

impl<ErrType, InspectorFn> AsyncErrFlow<ErrType> for ErrInspectStep<ErrType, InspectorFn>
where
    ErrType: Send + Sync,
    InspectorFn: Fn(&ErrType) + Send + Sync,
{
    type OutputErr = ErrType;

    fn apply_err_async<'a>(
        &'a self,
        input_err: ErrType,
    ) -> impl Future<Output = Self::OutputErr> + Send + 'a {
        async { self.apply(input_err) }
    }
}
