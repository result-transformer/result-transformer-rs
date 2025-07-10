//! Async implementation of result log tap step.

use crate::__internal::shared_step::ResultLogTapStep;
use crate::async_::AsyncResultFlow;

impl<OkType, ErrType> AsyncResultFlow<OkType, ErrType> for ResultLogTapStep<OkType, ErrType>
where
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
