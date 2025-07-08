//! Async implementation of result noop step.

use result_transformer_dependencies::*;

use crate::__internal::shared_step::ResultNoopStep;
use crate::async_::AsyncResultFlow;

#[async_trait::async_trait]
impl<OkType, ErrType> AsyncResultFlow<OkType, ErrType> for ResultNoopStep<OkType, ErrType>
where
    OkType: Send + Sync,
    ErrType: Send + Sync,
{
    type OutputOk = OkType;
    type OutputErr = ErrType;

    async fn apply_result(
        &self,
        input_result: Result<OkType, ErrType>,
    ) -> Result<Self::OutputOk, Self::OutputErr> {
        self.apply(input_result)
    }
}
