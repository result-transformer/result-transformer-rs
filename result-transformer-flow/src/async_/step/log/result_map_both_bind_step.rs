use crate::async_::AsyncResultFlow;
use result_transformer_dependencies::*;

use crate::sync::{flow::*, step::*};

#[async_trait::async_trait]
impl<OkType, ErrType, OutputOk, OutputErr, OkFn, ErrFn> AsyncResultFlow<OkType, ErrType>
    for ResultLogBothBindStep<OkType, ErrType, OutputOk, OutputErr, OkFn, ErrFn>
where
    OkFn: Fn(OkType) -> Result<OutputOk, OutputErr> + Send + Sync,
    ErrFn: Fn(ErrType) -> Result<OutputOk, OutputErr> + Send + Sync,
    OkType: Send + Sync,
    ErrType: Send + Sync,
    OutputOk: Send + Sync,
    OutputErr: Send + Sync,
{
    type OutputOk = OutputOk;
    type OutputErr = OutputErr;

    async fn apply_result(
        &self,
        input_result: Result<OkType, ErrType>,
    ) -> Result<Self::OutputOk, Self::OutputErr> {
        ResultFlow::apply_result(self, input_result)
    }
}
