//! async_.rs – Tests for asynchronous transformer behavior
//!
//! Located at `result-transformer-test/src/core/`.
//! ────────────────────────────────────────────────────────────────
//!  HOW TO RUN
//!  $ cargo test --package result-transformer-test --lib --features core-async -- core::async_ --show-output
//! ────────────────────────────────────────────────────────────────
//!
//! Simple asynchronous transformer used in tests.

use result_transformer_dependencies::*;

use result_transformer::async_::{AsyncErrTransformer, AsyncOkTransformer, AsyncResultTransformer};

struct AsyncTransformer;

impl AsyncOkTransformer<i32> for AsyncTransformer {
    type OutputOk = i64;
    async fn transform_ok_async(&self, ok: i32) -> Self::OutputOk {
        (ok as i64) + 2
    }
}

impl AsyncErrTransformer<&'static str> for AsyncTransformer {
    type OutputErr = String;
    async fn transform_err_async(&self, err: &'static str) -> Self::OutputErr {
        format!("AE:{}", err)
    }
}

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
