use result_transformer_dependencies::*;

use async_trait::async_trait;
use result_transformer::async_::{AsyncErrTransformer, AsyncOkTransformer, AsyncResultTransformer};

struct AsyncTransformer;

#[async_trait]
impl AsyncOkTransformer<i32> for AsyncTransformer {
    type OutputOk = i64;
    async fn transform_ok_async(&self, ok: i32) -> Self::OutputOk {
        (ok as i64) + 2
    }
}

#[async_trait]
impl AsyncErrTransformer<&'static str> for AsyncTransformer {
    type OutputErr = String;
    async fn transform_err_async(&self, err: &'static str) -> Self::OutputErr {
        format!("AE:{}", err)
    }
}

#[async_trait]
impl AsyncResultTransformer<i32, &'static str> for AsyncTransformer {
    type OutputOk = <Self as AsyncOkTransformer<i32>>::OutputOk;
    type OutputErr = <Self as AsyncErrTransformer<&'static str>>::OutputErr;

    async fn transform_async(
        &self,
        result: Result<i32, &'static str>,
    ) -> Result<Self::OutputOk, Self::OutputErr> {
        match result {
            Ok(v) => Ok(self.transform_ok_async(v).await),
            Err(e) => Err(self.transform_err_async(e).await),
        }
    }
}

#[tokio::test]
async fn transforms_ok_value_async() {
    let transformer = AsyncTransformer;
    let result = transformer.transform_async(Ok(1)).await;
    assert_eq!(result, Ok(3));
}

#[tokio::test]
async fn transforms_err_value_async() {
    let transformer = AsyncTransformer;
    let result = transformer.transform_async(Err("bad")).await;
    assert_eq!(result, Err("AE:bad".to_string()));
}
