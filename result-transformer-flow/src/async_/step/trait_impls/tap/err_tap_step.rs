//! Async implementation of err tap step.

use crate::__internal::shared_step::ErrTapStep;
use crate::async_::AsyncErrFlow;

impl<InputErr, OutputErr, TapFn> AsyncErrFlow<InputErr> for ErrTapStep<InputErr, OutputErr, TapFn>
where
    InputErr: Send + Sync,
    OutputErr: Send + Sync,
    TapFn: Fn(InputErr) -> OutputErr + Send + Sync,
{
    type OutputErr = OutputErr;

    fn apply_err_async<'a>(
        &'a self,
        input_err: InputErr,
    ) -> impl Future<Output = Self::OutputErr> + Send + 'a {
        async { self.apply(input_err) }
    }
}
