use result_transformer_dependencies::*;

#[async_trait::async_trait]
pub trait AsyncErrTransformer<InputErr> {
    type OutputErr;
    async fn transform_err_async(&self, err: InputErr) -> Self::OutputErr;
}
