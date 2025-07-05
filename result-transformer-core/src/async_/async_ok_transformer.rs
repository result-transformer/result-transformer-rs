use result_transformer_dependencies::*;

#[async_trait::async_trait]
pub trait AsyncOkTransformer<InputOk> {
    type OutputOk;
    async fn transform_ok_async(&self, ok: InputOk) -> Self::OutputOk;
}
