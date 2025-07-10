//! Async implementation of err map step.

use crate::__internal::shared_step::ErrMapStep;
use crate::async_::AsyncErrFlow;

impl<MapperFn, InputErr, OutputErr> AsyncErrFlow<InputErr>
    for ErrMapStep<MapperFn, InputErr, OutputErr>
where
    MapperFn: Fn(InputErr) -> OutputErr + Send + Sync,
    InputErr: Send + Sync,
    OutputErr: Send + Sync,
{
    type OutputErr = OutputErr;

    fn apply_err_async<'a>(
        &'a self,
        input_err: InputErr,
    ) -> impl Future<Output = Self::OutputErr> + Send + 'a {
        async { self.apply(input_err) }
    }
}
