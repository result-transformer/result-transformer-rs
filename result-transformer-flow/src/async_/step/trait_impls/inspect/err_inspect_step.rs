//! Async implementation of err inspect step.

use result_transformer_dependencies::*;

use crate::__internal::shared_step::ErrInspectStep;
use crate::async_::AsyncErrFlow;

#[async_trait::async_trait]
impl<InspectorFn, ErrType> AsyncErrFlow<ErrType> for ErrInspectStep<InspectorFn, ErrType>
where
    InspectorFn: Fn(&ErrType) + Send + Sync,
    ErrType: Send + Sync,
{
    type OutputErr = ErrType;

    async fn apply_err(&self, input_err: ErrType) -> Self::OutputErr {
        self.apply(input_err)
    }
}
