//! Async implementation of ok noop step.

use crate::__internal::shared_step::OkNoopStep;
use crate::async_::AsyncOkFlow;

impl<OkType> AsyncOkFlow<OkType> for OkNoopStep<OkType>
where
    OkType: Send + Sync,
{
    type OutputOk = OkType;

    fn apply_ok_async<'a>(
        &'a self,
        input_ok: OkType,
    ) -> impl Future<Output = Self::OutputOk> + Send + 'a {
        async { self.apply(input_ok) }
    }
}
