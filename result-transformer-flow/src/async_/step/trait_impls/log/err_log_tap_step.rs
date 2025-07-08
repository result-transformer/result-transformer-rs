//! Async implementation of err log tap step.

use result_transformer_dependencies::*;

use crate::__internal::shared_step::ErrLogTapStep;
use crate::async_::AsyncErrFlow;

#[async_trait::async_trait]
impl<ErrType> AsyncErrFlow<ErrType> for ErrLogTapStep<ErrType>
where
    ErrType: Send + Sync,
{
    type OutputErr = ErrType;

    async fn apply_err(&self, input_err: ErrType) -> Self::OutputErr {
        self.apply(input_err)
    }
}
