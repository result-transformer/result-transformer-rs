//! Async implementation of err tap step.

use result_transformer_dependencies::*;

use crate::__internal::shared_step::ErrTapStep;
use crate::async_::AsyncErrFlow;

#[async_trait::async_trait]
impl<TapFn, InputErr, OutputErr> AsyncErrFlow<InputErr> for ErrTapStep<TapFn, InputErr, OutputErr>
where
    TapFn: Fn(InputErr) -> OutputErr + Send + Sync,
    InputErr: Send + Sync,
    OutputErr: Send + Sync,
{
    type OutputErr = OutputErr;

    async fn apply_err(&self, input_err: InputErr) -> Self::OutputErr {
        self.apply(input_err)
    }
}
