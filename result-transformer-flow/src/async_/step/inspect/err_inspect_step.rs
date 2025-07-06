use crate::async_::AsyncErrFlow;
use result_transformer_dependencies::*;

use crate::sync::{flow::*, step::*};

#[async_trait::async_trait]
impl<InspectorFn, ErrType> AsyncErrFlow<ErrType> for ErrInspectStep<InspectorFn, ErrType>
where
    InspectorFn: Fn(&ErrType) + Send + Sync,
    ErrType: Send + Sync,
{
    type OutputErr = ErrType;

    async fn apply_err(&self, input_err: ErrType) -> Self::OutputErr {
        ErrFlow::apply_err(self, input_err)
    }
}
