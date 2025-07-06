//! Async implementation of result log tap step.

use crate::async_::AsyncResultFlow;
use result_transformer_dependencies::*;

use crate::sync::{flow::*, step::*};

#[async_trait::async_trait]
impl<OkType, ErrType> AsyncResultFlow<OkType, ErrType> for ResultLogTapStep<OkType, ErrType>
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
        ResultFlow::apply_result(self, input_result)
    }
}
