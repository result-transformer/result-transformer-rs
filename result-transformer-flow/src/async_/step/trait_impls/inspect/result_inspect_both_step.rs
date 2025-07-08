//! Async implementation of result inspect both step.

use result_transformer_dependencies::*;

use crate::__internal::shared_step::ResultInspectBothStep;
use crate::async_::AsyncResultFlow;

#[async_trait::async_trait]
impl<OkInspectorFn, ErrInspectorFn, OkType, ErrType> AsyncResultFlow<OkType, ErrType>
    for ResultInspectBothStep<OkInspectorFn, ErrInspectorFn, OkType, ErrType>
where
    OkInspectorFn: Fn(&OkType) + Send + Sync,
    ErrInspectorFn: Fn(&ErrType) + Send + Sync,
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
