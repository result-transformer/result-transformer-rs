use result_transformer_dependencies::*;

#[async_trait::async_trait]
pub trait AsyncResultTransformer<InputOk, InputErr> {
    type OutputOk;
    type OutputErr;
    async fn transform_async(
        &self,
        result: Result<InputOk, InputErr>,
    ) -> Result<Self::OutputOk, Self::OutputErr>;
}
