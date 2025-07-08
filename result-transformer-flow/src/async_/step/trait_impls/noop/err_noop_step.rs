//! Async implementation of err noop step.

use result_transformer_dependencies::*;

use crate::__internal::shared_step::ErrNoopStep;
use crate::async_::AsyncErrFlow;

#[async_trait::async_trait]
impl<ErrType> AsyncErrFlow<ErrType> for ErrNoopStep<ErrType>
where
    ErrType: Send + Sync,
{
    type OutputErr = ErrType;

    async fn apply_err(&self, input_err: ErrType) -> Self::OutputErr {
        self.apply(input_err)
    }
}
