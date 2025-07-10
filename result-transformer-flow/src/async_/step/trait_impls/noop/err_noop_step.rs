//! Async implementation of err noop step.

use crate::__internal::shared_step::ErrNoopStep;
use crate::async_::AsyncErrFlow;

impl<ErrType> AsyncErrFlow<ErrType> for ErrNoopStep<ErrType>
where
    ErrType: Send + Sync,
{
    type OutputErr = ErrType;

    fn apply_err_async<'a>(
        &'a self,
        input_err: ErrType,
    ) -> impl Future<Output = Self::OutputErr> + Send + 'a {
        async { self.apply(input_err) }
    }
}
