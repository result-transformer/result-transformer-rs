//! Async implementation of ok log tap step.

use crate::__internal::shared_step::OkLogTapStep;
use crate::async_::AsyncOkFlow;

impl<OkType> AsyncOkFlow<OkType> for OkLogTapStep<OkType>
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
