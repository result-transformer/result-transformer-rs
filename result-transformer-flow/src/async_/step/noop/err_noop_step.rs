use crate::async_::AsyncErrFlow;
use result_transformer_dependencies::*;

use crate::sync::{flow::*, step::*};

#[async_trait::async_trait]
impl<ErrType> AsyncErrFlow<ErrType> for ErrNoopStep<ErrType>
where
    ErrType: Send + Sync,
{
    type OutputErr = ErrType;

    async fn apply_err(&self, input_err: ErrType) -> Self::OutputErr {
        ErrFlow::apply_err(self, input_err)
    }
}
