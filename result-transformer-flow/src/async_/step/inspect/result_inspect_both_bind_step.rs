//! Async implementation of result inspect both bind step.

use crate::async_::AsyncResultFlow;
use result_transformer_dependencies::*;

use crate::sync::{flow::*, step::*};

#[async_trait::async_trait]
impl<OkInspectorFn, ErrInspectorFn, OkType, ErrType, OutputOk, OutputErr>
    AsyncResultFlow<OkType, ErrType>
    for ResultInspectBothBindStep<
        OkInspectorFn,
        ErrInspectorFn,
        OkType,
        ErrType,
        OutputOk,
        OutputErr,
    >
where
    OkInspectorFn: Fn(&OkType) -> Result<OutputOk, OutputErr> + Send + Sync,
    ErrInspectorFn: Fn(&ErrType) -> Result<OutputOk, OutputErr> + Send + Sync,
    OkType: Send + Sync,
    ErrType: Send + Sync,
    OutputOk: Send + Sync,
    OutputErr: Send + Sync,
{
    type OutputOk = OutputOk;
    type OutputErr = OutputErr;

    async fn apply_result(
        &self,
        input: Result<OkType, ErrType>,
    ) -> Result<Self::OutputOk, Self::OutputErr> {
        ResultFlow::apply_result(self, input)
    }
}
